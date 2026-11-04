//! Lane P9 integration tests: slash-command markdown surface.
//! Chicago-style: real fixture files on disk, real `validate_command_file`, assert final findings.

use std::path::PathBuf;

use claude_code_config_lsp::analyzers::commands::{
    validate_command_file, CMD_001, CMD_002, CMD_003, CMD_004, CMD_006,
};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/commands")
        .join(name)
}

fn read(name: &str) -> String {
    std::fs::read_to_string(fixture(name)).expect("fixture readable")
}

fn dir_of(name: &str) -> PathBuf {
    fixture(name).parent().unwrap().to_path_buf()
}

#[test]
fn valid_fixture_is_clean() {
    let f = validate_command_file(&read("valid.md"), Some(&dir_of("valid.md")));
    assert_eq!(f, vec![], "valid.md must produce zero findings, got {f:?}");
    assert_eq!(read("valid.md").matches("$ARGUMENTS").count(), 1);
}

#[test]
fn missing_description_fixture_fires_ccc_cmd_001() {
    let f = validate_command_file(
        &read("missing-description.md"),
        Some(&dir_of("missing-description.md")),
    );
    assert!(f.iter().any(|x| x.code == CMD_001), "{f:?}");
}

#[test]
fn unbalanced_arguments_fixture_fires_ccc_cmd_002() {
    let f = validate_command_file(
        &read("unbalanced-arguments.md"),
        Some(&dir_of("unbalanced-arguments.md")),
    );
    assert!(f.iter().any(|x| x.code == CMD_002), "{f:?}");
}

#[test]
fn unresolvable_atref_fixture_fires_ccc_cmd_003() {
    let f = validate_command_file(
        &read("unresolvable-atref.md"),
        Some(&dir_of("unresolvable-atref.md")),
    );
    let hits: Vec<_> = f.iter().filter(|x| x.code == CMD_003).collect();
    assert_eq!(hits.len(), 1, "{f:?}");
    // `Read` list entry without a pattern also fires CCC-CMD-004.
    assert!(f.iter().any(|x| x.code == CMD_004), "{f:?}");
    assert!(hits[0].message.contains("does-not-exist-anywhere.md"));
}

#[test]
fn unclosed_shell_span_fixture_fires_ccc_cmd_006() {
    let f = validate_command_file(
        &read("unclosed-shell-span.md"),
        Some(&dir_of("unclosed-shell-span.md")),
    );
    assert!(f.iter().any(|x| x.code == CMD_006), "{f:?}");
    // disable-model-invocation: true is valid — no CCC-CMD-005 here.
    assert!(!f.iter().any(|x| x.code == "CCC-CMD-005"), "{f:?}");
}
