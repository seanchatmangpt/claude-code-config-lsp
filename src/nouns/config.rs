use clap_noun_verb::Result;
use clap_noun_verb_macros::verb;

use claude_code_config_lsp::inventory::{
    self, ConfigTree, ConfigUsageReport, ScanReport, WorkspaceConformance,
};

/// Route all `tracing` output to stderr, and do it before anything can log.
///
/// stdout is the LSP frame channel. `tracing`'s default subscriber writes to
/// stdout, so a single INFO line emitted during startup (for example
/// `lsp_max::service::watchdog` announcing the parent PID) lands *in front of*
/// the first `Content-Length:` header and corrupts the stream for any client
/// that parses strictly. Installing a stderr subscriber first claims the global
/// default, so no later initializer can point logging at stdout.
///
/// `try_init` rather than `init`: if a caller (a test, an embedding binary) has
/// already installed a subscriber, losing the race is fine -- failing to start
/// the server over it is not.
fn route_logs_to_stderr() {
    let _ = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .try_init();
}

/// Start the LSP server over stdio. Shared by the `serve` verb and by a bare
/// (arg-less) invocation of the binary — see `src/main.rs`.
pub async fn start_stdio_server() {
    route_logs_to_stderr();
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
    Ok(inventory::scan_report(
        &path.unwrap_or_else(|| ".".to_string()),
    ))
}

/// Export the current Claude Code config tree as JSON
#[verb("export", "root")]
pub fn export(#[arg(index = 1)] path: Option<String>) -> Result<ConfigTree> {
    Ok(inventory::build_tree(
        &path.unwrap_or_else(|| ".".to_string()),
    ))
}

/// Report how much config is present but not being used (dead-config audit)
#[verb("unused", "root")]
pub fn unused(#[arg(index = 1)] path: Option<String>) -> Result<ConfigUsageReport> {
    Ok(inventory::usage_report(
        &path.unwrap_or_else(|| ".".to_string()),
    ))
}

/// Conformance scorecard for the Claude Code workspace configuration.
///
/// `--format json|yaml|table` controls the output shape (default json, matching
/// the `claude-config://health` vector). The default (no `--format`) path is
/// unchanged: the scorecard struct is returned and the CLI framework renders it.
/// Explicit formats are rendered here (JSON via serde_json, YAML hand-emitted,
/// table as aligned text) and printed directly; the verb exits 0 immediately so
/// the framework does not double-print the returned struct after it.
#[verb("conformance", "root")]
pub fn conformance(
    #[arg(index = 1)] path: Option<String>,
    format: Option<String>,
) -> Result<WorkspaceConformance> {
    let report = inventory::conformance_report(&path.unwrap_or_else(|| ".".to_string()));
    if let Some(fmt) = format.as_deref() {
        let rendered = match fmt {
            "json" => render_conformance_json(&report),
            "yaml" => render_conformance_yaml(&report),
            "table" => render_conformance_table(&report),
            other => {
                eprintln!(
                    "Error: unsupported --format '{}': expected json|yaml|table",
                    other
                );
                std::process::exit(1);
            }
        };
        println!("{}", rendered);
        std::process::exit(0);
    }
    Ok(report)
}

/// Render a [`WorkspaceConformance`] as pretty JSON (serde_json).
fn render_conformance_json(r: &WorkspaceConformance) -> String {
    serde_json::to_string_pretty(r).unwrap_or_else(|e| {
        eprintln!("Error: failed to serialize conformance report: {}", e);
        std::process::exit(1);
    })
}

/// Hand-emit a [`WorkspaceConformance`] as YAML (no serde_yaml dependency).
/// Strings are quoted so violation text (which may contain `: ` or `#`)
/// cannot break the scalar.
fn render_conformance_yaml(r: &WorkspaceConformance) -> String {
    let mut out = String::new();
    out.push_str(&format!("score: {}\n", r.score));
    out.push_str(&format!("surfaces_checked: {}\n", r.surfaces_checked));
    out.push_str(&format!("surfaces_admitted: {}\n", r.surfaces_admitted));
    if r.violations.is_empty() {
        out.push_str("violations: []\n");
    } else {
        out.push_str("violations:\n");
        for v in &r.violations {
            out.push_str(&format!("  - {}\n", yaml_quote(v)));
        }
    }
    out
}

/// Quote a string for emission as a YAML flow scalar.
fn yaml_quote(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Render a [`WorkspaceConformance`] as aligned text: a summary column block
/// followed by a fixed-width violation table (index, code, file, message).
fn render_conformance_table(r: &WorkspaceConformance) -> String {
    let mut out = String::new();
    out.push_str(&format!("score              {}\n", r.score));
    out.push_str(&format!("surfaces_checked   {}\n", r.surfaces_checked));
    out.push_str(&format!("surfaces_admitted  {}\n", r.surfaces_admitted));
    out.push_str(&format!("violations         {}\n", r.violations.len()));
    if !r.violations.is_empty() {
        out.push('\n');
        // Split "CODE: path message" violations into columns.
        let parsed: Vec<(String, String, String)> =
            r.violations.iter().map(|v| split_violation(v)).collect();
        let code_w = parsed.iter().map(|(c, _, _)| c.len()).max().unwrap_or(4);
        let file_w = parsed.iter().map(|(_, f, _)| f.len()).max().unwrap_or(4);
        out.push_str(&format!(
            "{:<width_code$}  {:<width_file$}  MESSAGE\n",
            "CODE",
            "FILE",
            width_code = code_w,
            width_file = file_w
        ));
        for (code, file, message) in &parsed {
            out.push_str(&format!(
                "{:<width_code$}  {:<width_file$}  {}\n",
                code,
                file,
                message,
                width_code = code_w,
                width_file = file_w
            ));
        }
    }
    out
}

/// Split a `CODE: path message...` violation string into (code, file, message).
/// The file column is the leading run of non-whitespace after the code.
fn split_violation(v: &str) -> (String, String, String) {
    let (code, rest) = match v.split_once(": ") {
        Some((c, r)) => (c.to_string(), r.to_string()),
        None => return (v.to_string(), String::new(), String::new()),
    };
    let mut parts = rest.splitn(2, char::is_whitespace);
    let file = parts.next().unwrap_or("").to_string();
    let message = parts.next().unwrap_or("").to_string();
    (code, file, message)
}

/// Get the list/sequence of receipts in the chain from genesis to latest
#[verb("chain", "receipt")]
pub fn chain(
    #[arg(index = 1)] path: Option<String>,
) -> Result<Vec<claude_code_config_lsp::receipt::Receipt>> {
    Ok(receipt_chain_or_die(
        &path.unwrap_or_else(|| ".".to_string()),
    ))
}

/// Read all receipts for `root` and build the genesis-to-latest chain,
/// exiting non-zero with a stderr diagnostic on any failure.
fn receipt_chain_or_die(root: &str) -> Vec<claude_code_config_lsp::receipt::Receipt> {
    let fail = |e: String| -> ! {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    };
    let receipts =
        match claude_code_config_lsp::receipt::read_all_receipts(std::path::Path::new(root)) {
            Ok(r) => r,
            Err(e) => fail(e),
        };
    match claude_code_config_lsp::receipt::build_chain(&receipts) {
        Ok(chain) => chain,
        Err(e) => fail(e),
    }
}

/// Verify the receipt chain's Merkle hashes, prior-hash linkage, and workspace
/// checksum agreement (exit 0 valid, exit 1 on any tampering or broken link).
#[verb("verify", "receipt")]
pub fn verify(#[arg(index = 1)] path: Option<String>) -> Result<String> {
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
    #[arg(short = 'y')] apply: bool,
) -> Result<inventory::FixReport> {
    Ok(inventory::fix_report(
        &path.unwrap_or_else(|| ".".to_string()),
        apply,
    ))
}


/// `receipt verify [PATH]` CLI shim.
///
/// The `affidavit` crate (transitive via `lsp-max`) owns the framework's
/// `receipt verify` slot: `CommandRegistry::init()` re-runs the registration
/// slices on EVERY call, so any pre-dispatch override is undone when
/// `clap_noun_verb::run()` re-inits. The R2 contract needs `receipt verify
/// [PATH]` (workspace root, exit 0 valid / exit 1 tampered), so `main`
/// dispatches that exact argv shape here before entering the framework.
/// Everything else (`receipt chain`, `receipt check`) routes normally.
pub fn receipt_verify_cli(path: Option<&str>) {
    let root = path.unwrap_or(".");
    match claude_code_config_lsp::receipt::verify_receipt_chain(std::path::Path::new(root)) {
        Ok(()) => {
            println!("Verification succeeded: receipt chain is valid.");
        }
        Err(e) => {
            eprintln!("Verification failed: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod render_tests {
    use super::*;

    fn sample() -> WorkspaceConformance {
        WorkspaceConformance {
            score: 0.7,
            surfaces_checked: 25,
            surfaces_admitted: 22,
            violations: vec![
                "CCC-JSON-001: .claude/settings.json Deprecated key".to_string(),
                "CCC-AGENT-002: .claude/agents/w.md Unknown model tier".to_string(),
            ],
        }
    }

    #[test]
    fn json_render_is_pretty_json_with_health_vector_fields() {
        let parsed: serde_json::Value =
            serde_json::from_str(&render_conformance_json(&sample())).expect("valid JSON");
        assert_eq!(parsed["score"], 0.7);
        assert!(parsed["surfaces_checked"].is_u64());
        assert!(parsed["surfaces_admitted"].is_u64());
        assert!(parsed["violations"].is_array());
        assert_eq!(parsed["violations"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn yaml_render_has_key_lines_and_quoted_list_items() {
        let yaml = render_conformance_yaml(&sample());
        assert!(yaml.starts_with("score: 0.7\n"));
        assert!(yaml.contains("surfaces_checked: 25"));
        assert!(yaml.contains("violations:\n"));
        assert!(yaml.contains("  - \"CCC-JSON-001: .claude/settings.json Deprecated key\"\n"));
    }

    #[test]
    fn yaml_render_empty_violations_is_empty_list() {
        let mut r = sample();
        r.violations.clear();
        assert!(render_conformance_yaml(&r).contains("violations: []\n"));
    }

    #[test]
    fn yaml_quote_escapes_embedded_quotes() {
        assert_eq!(yaml_quote("a \"b\" c"), "\"a \\\"b\\\" c\"");
    }

    #[test]
    fn table_render_is_aligned_with_column_headers() {
        let t = render_conformance_table(&sample());
        assert!(t.contains("score              0.7"));
        assert!(t.contains("CODE"));
        assert!(t.contains("FILE"));
        assert!(t.contains("MESSAGE"));
        // Aligned columns: both violation rows start with their code at col 0
        // and the file column begins at the same offset on every data line.
        let data: Vec<&str> = t.lines().filter(|l| l.starts_with("CCC-")).collect();
        assert_eq!(data.len(), 2);
        let file_col = data[0].find(".claude/settings.json").unwrap();
        for row in &data {
            assert!(
                row.find(".claude/").unwrap() == file_col
                    || row[..file_col].ends_with(' ')
            );
        }
    }

    #[test]
    fn split_violation_parses_code_file_message() {
        let (c, f, m) =
            split_violation("CCC-JSON-001: path/to/settings.json Deprecated key — use X");
        assert_eq!(c, "CCC-JSON-001");
        assert_eq!(f, "path/to/settings.json");
        assert_eq!(m, "Deprecated key — use X");
    }

    #[test]
    fn split_violation_handles_missing_colon() {
        let (c, f, m) = split_violation("bare violation");
        assert_eq!(c, "bare violation");
        assert_eq!(f, "");
        assert_eq!(m, "");
    }
}
