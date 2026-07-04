use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct FileChecksum {
    pub path: String,
    pub checksum: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Receipt {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub conformance_score: f64,
    pub file_checksums: Vec<FileChecksum>,
    pub previous_receipt_hash: Option<String>,
    #[serde(default)]
    pub hash: String,
}

/// Computes the hash of the receipt by serializing its target fields (excluding its own hash)
/// into JSON and hashing with BLAKE3.
pub fn compute_receipt_hash(
    timestamp: &chrono::DateTime<chrono::Utc>,
    conformance_score: f64,
    file_checksums: &[FileChecksum],
    previous_receipt_hash: &Option<String>,
) -> String {
    #[derive(Serialize)]
    struct HashTarget<'a> {
        timestamp: &'a chrono::DateTime<chrono::Utc>,
        conformance_score: f64,
        file_checksums: &'a [FileChecksum],
        previous_receipt_hash: &'a Option<String>,
    }
    let target = HashTarget {
        timestamp,
        conformance_score,
        file_checksums,
        previous_receipt_hash,
    };
    let json_str = serde_json::to_string(&target).expect("Failed to serialize hash target");
    blake3::hash(json_str.as_bytes()).to_hex().to_string()
}

/// Helper function to compute the BLAKE3 hash of a file on disk.
pub fn compute_file_checksum(file_path: &Path) -> std::io::Result<String> {
    let content = std::fs::read(file_path)?;
    let hash = blake3::hash(&content);
    Ok(hash.to_hex().to_string())
}

/// Returns the receipts directory: `[PATH]/.claude/receipts/`.
pub fn get_receipts_dir(workspace_root: &Path) -> PathBuf {
    workspace_root.join(".claude").join("receipts")
}

/// Read all receipts from the directory, verifying that filename matches receipt hash.
pub fn read_all_receipts(workspace_root: &Path) -> Result<Vec<Receipt>, String> {
    let dir = get_receipts_dir(workspace_root);
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut receipts = Vec::new();
    let entries = std::fs::read_dir(&dir)
        .map_err(|e| format!("Failed to read receipts directory: {e}"))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read receipt file {}: {e}", path.display()))?;
            let receipt: Receipt = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to deserialize receipt from {}: {e}", path.display()))?;
            
            // Verify computed hash matches receipt.hash
            let computed = compute_receipt_hash(
                &receipt.timestamp,
                receipt.conformance_score,
                &receipt.file_checksums,
                &receipt.previous_receipt_hash,
            );
            if computed != receipt.hash {
                return Err(format!(
                    "Tampering detected: Receipt in {} has hash {} but computed hash is {}",
                    path.display(),
                    receipt.hash,
                    computed
                ));
            }
            
            // Verify filename (without .json) matches receipt.hash
            let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if file_stem != receipt.hash {
                return Err(format!(
                    "Filename mismatch: Receipt file {} does not match its hash {}",
                    path.display(),
                    receipt.hash
                ));
            }
            
            receipts.push(receipt);
        }
    }
    Ok(receipts)
}

/// Find the latest receipt (the one whose hash is not referenced by any receipt as `previous_receipt_hash`).
pub fn find_latest_receipt(receipts: &[Receipt]) -> Option<Receipt> {
    if receipts.is_empty() {
        return None;
    }
    let referenced: std::collections::HashSet<&str> = receipts
        .iter()
        .filter_map(|r| r.previous_receipt_hash.as_deref())
        .collect();
    
    let mut candidates: Vec<&Receipt> = receipts
        .iter()
        .filter(|r| !referenced.contains(r.hash.as_str()))
        .collect();
    
    // Sort by timestamp to break ties or ensure latest in case of multiple heads,
    // though in a valid chain there should be at most one.
    candidates.sort_by_key(|r| r.timestamp);
    candidates.last().cloned().cloned()
}

/// Build the sequence of receipts (chain) starting from genesis to latest.
pub fn build_chain(receipts: &[Receipt]) -> Result<Vec<Receipt>, String> {
    if receipts.is_empty() {
        return Ok(Vec::new());
    }
    let latest = find_latest_receipt(receipts)
        .ok_or_else(|| "Could not find latest receipt (possible cycle or empty chain)".to_string())?;
    
    let mut chain = Vec::new();
    let mut current = latest;
    let mut visited = std::collections::HashSet::new();
    
    let by_hash: std::collections::HashMap<&str, &Receipt> = receipts
        .iter()
        .map(|r| (r.hash.as_str(), r))
        .collect();
        
    loop {
        if !visited.insert(current.hash.clone()) {
            return Err("Cycle detected in receipt chain".to_string());
        }
        chain.push(current.clone());
        if let Some(ref prev_hash) = current.previous_receipt_hash {
            if let Some(prev_receipt) = by_hash.get(prev_hash.as_str()) {
                current = (*prev_receipt).clone();
            } else {
                return Err(format!("Broken chain: previous receipt hash {} not found on disk", prev_hash));
            }
        } else {
            break; // reached genesis
        }
    }
    
    chain.reverse();
    Ok(chain)
}

/// Collects the current workspace configuration files and their checksums.
pub fn get_current_workspace_checksums(workspace_root: &Path) -> Result<Vec<FileChecksum>, String> {
    let mut current = Vec::new();
    for path in crate::scan::discover(workspace_root) {
        let path_str = path.to_string_lossy().to_string();
        if crate::scan::classify(&path_str) == "unknown" {
            continue;
        }
        
        let rel_path = path.strip_prefix(workspace_root)
            .map_err(|e| format!("Failed to strip prefix: {e}"))?
            .to_string_lossy()
            .to_string();
            
        let checksum = compute_file_checksum(&path)
            .map_err(|e| format!("Failed to compute checksum for {}: {e}", path.display()))?;
            
        current.push(FileChecksum {
            path: rel_path,
            checksum,
        });
    }
    current.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(current)
}

/// Verify receipt chain integrity: check Merkle hash consistency, chain link consistency,
/// and check that the current workspace files match the latest receipt's file checksums.
pub fn verify_receipt_chain(workspace_root: &Path) -> Result<(), String> {
    let receipts = read_all_receipts(workspace_root)?;
    if receipts.is_empty() {
        return Err("No receipts found. Chain is empty.".to_string());
    }
    
    // Build the chain from genesis to latest
    let chain = build_chain(&receipts)?;
    
    // Verify Merkle and chain link consistency
    for i in 0..chain.len() {
        let receipt = &chain[i];
        let computed = compute_receipt_hash(
            &receipt.timestamp,
            receipt.conformance_score,
            &receipt.file_checksums,
            &receipt.previous_receipt_hash,
        );
        if computed != receipt.hash {
            return Err(format!(
                "Merkle hash mismatch: receipt {} computed hash is {}",
                receipt.hash, computed
            ));
        }
        if i == 0 {
            if receipt.previous_receipt_hash.is_some() {
                return Err(format!(
                    "Genesis receipt {} must not have a previous receipt hash",
                    receipt.hash
                ));
            }
        } else {
            let prev = &chain[i - 1];
            if receipt.previous_receipt_hash.as_deref() != Some(prev.hash.as_str()) {
                return Err(format!(
                    "Chain link inconsistency: receipt {} references previous hash {:?} but preceding receipt has hash {}",
                    receipt.hash, receipt.previous_receipt_hash, prev.hash
                ));
            }
        }
    }
    
    // Check that the current workspace files match the latest receipt's file checksums
    let latest = chain.last().unwrap();
    let mut current_checksums = get_current_workspace_checksums(workspace_root)?;
    let mut latest_checksums = latest.file_checksums.clone();
    current_checksums.sort_by(|a, b| a.path.cmp(&b.path));
    latest_checksums.sort_by(|a, b| a.path.cmp(&b.path));
    
    if current_checksums != latest_checksums {
        return Err("Workspace config files do not match the latest receipt".to_string());
    }
    
    Ok(())
}

/// Automatically issue and write a new receipt to disk if the configuration state
/// (checksums or score) differs from the latest receipt.
pub fn maybe_issue_receipt(workspace_root: &Path, current_score: f64) -> Result<Option<Receipt>, String> {
    let current_checksums = get_current_workspace_checksums(workspace_root)?;
    let receipts = read_all_receipts(workspace_root)?;
    
    let latest = find_latest_receipt(&receipts);
    
    let should_issue = match &latest {
        None => true, // Genesis receipt
        Some(lat) => {
            let mut lat_checksums = lat.file_checksums.clone();
            lat_checksums.sort_by(|a, b| a.path.cmp(&b.path));
            let mut cur_checksums = current_checksums.clone();
            cur_checksums.sort_by(|a, b| a.path.cmp(&b.path));
            
            // Check if checksums differ or conformance score differs
            cur_checksums != lat_checksums || (lat.conformance_score - current_score).abs() > 1e-9
        }
    };
    
    if should_issue {
        let prev_hash = latest.map(|r| r.hash);
        let timestamp = chrono::Utc::now();
        
        let hash = compute_receipt_hash(
            &timestamp,
            current_score,
            &current_checksums,
            &prev_hash,
        );
        
        let receipt = Receipt {
            timestamp,
            conformance_score: current_score,
            file_checksums: current_checksums,
            previous_receipt_hash: prev_hash,
            hash: hash.clone(),
        };
        
        let dir = get_receipts_dir(workspace_root);
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create receipts directory: {e}"))?;
            
        let file_path = dir.join(format!("{}.json", hash));
        let content = serde_json::to_string_pretty(&receipt)
            .map_err(|e| format!("Failed to serialize receipt: {e}"))?;
            
        std::fs::write(&file_path, content)
            .map_err(|e| format!("Failed to write receipt to {}: {e}", file_path.display()))?;
            
        Ok(Some(receipt))
    } else {
        Ok(None)
    }
}
