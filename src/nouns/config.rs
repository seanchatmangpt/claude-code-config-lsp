use clap_noun_verb::Result;
use clap_noun_verb_macros::verb;

use claude_code_config_lsp::inventory::{self, ConfigTree, ConfigUsageReport, ScanReport, WorkspaceConformance};

/// Start the LSP server over stdio. Shared by the `serve` verb and by a bare
/// (arg-less) invocation of the binary — see `src/main.rs`.
pub async fn start_stdio_server() {
    let (service, socket) = lsp_max::LspService::new(claude_code_config_lsp::Backend::new);
    let _ = lsp_max::Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
        .serve(service)
        .await;
}

/// Start the LSP server over stdio
#[verb("serve", "root")]
pub fn serve(stdio: bool) -> Result<()> {
    if stdio {
        // Build a dedicated runtime here (main is sync — no ambient runtime to
        // nest inside, which would otherwise panic in `block_on`).
        let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
        rt.block_on(start_stdio_server());
    } else {
        eprintln!("Error: --stdio flag is required for LSP serve");
        std::process::exit(1);
    }
    Ok(())
}

/// Scan a directory (or single file) for Claude Code config diagnostics
#[verb("scan", "root")]
pub fn scan(#[arg(index = 1)] path: Option<String>) -> Result<ScanReport> {
    Ok(inventory::scan_report(&path.unwrap_or_else(|| ".".to_string())))
}

/// Export the current Claude Code config tree as JSON
#[verb("export", "root")]
pub fn export(#[arg(index = 1)] path: Option<String>) -> Result<ConfigTree> {
    Ok(inventory::build_tree(&path.unwrap_or_else(|| ".".to_string())))
}

/// Report how much config is present but not being used (dead-config audit)
#[verb("unused", "root")]
pub fn unused(#[arg(index = 1)] path: Option<String>) -> Result<ConfigUsageReport> {
    Ok(inventory::usage_report(&path.unwrap_or_else(|| ".".to_string())))
}

/// Conformance scorecard for the Claude Code workspace configuration
#[verb("conformance", "root")]
pub fn conformance(#[arg(index = 1)] path: Option<String>) -> Result<WorkspaceConformance> {
    Ok(inventory::conformance_report(&path.unwrap_or_else(|| ".".to_string())))
}

/// Get the list/sequence of receipts in the chain from genesis to latest
#[verb("chain", "receipt")]
pub fn chain(#[arg(index = 1)] path: Option<String>) -> Result<Vec<claude_code_config_lsp::receipt::Receipt>> {
    let root = path.unwrap_or_else(|| ".".to_string());
    match claude_code_config_lsp::receipt::read_all_receipts(std::path::Path::new(&root)) {
        Ok(receipts) => {
            match claude_code_config_lsp::receipt::build_chain(&receipts) {
                Ok(chain) => Ok(chain),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Check the Claude Code config receipt chain for tampering or broken links
#[verb("check", "receipt")]
pub fn check(#[arg(index = 1)] path: Option<String>) -> Result<String> {
    let root = path.unwrap_or_else(|| ".".to_string());
    match claude_code_config_lsp::receipt::verify_receipt_chain(std::path::Path::new(&root)) {
        Ok(()) => {
            println!("Verification succeeded: receipt chain is valid.");
            Ok("valid".to_string())
        }
        Err(e) => {
            eprintln!("Verification failed: {}", e);
            std::process::exit(1);
        }
    }
}


/// Auto-fix repairable Claude Code config issues.
///
/// By default runs in dry-run mode — shows what would be changed without
/// writing to disk. Pass `--apply` to write fixes back to the files.
#[verb("fix", "root")]
pub fn fix(
    #[arg(index = 1)] path: Option<String>,
    apply: bool,
) -> Result<inventory::FixReport> {
    Ok(inventory::fix_report(&path.unwrap_or_else(|| ".".to_string()), apply))
}

