// ============================================================================
// CRON CLI Agglutinative Mongolian Morphological Tokenizer Test Suite
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use std::process::Command;

#[test]
fn test_cli_morph_help() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["morph", "--help"])
        .output()
        .expect("run cron morph --help");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Agglutinative Mongolian"));
    assert!(stdout.contains("--traditional"));
    assert!(stdout.contains("--json"));
    assert!(stdout.contains("--benchmark"));
}

#[test]
fn test_cli_morph_segmentation() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["morph", "Монголчуудтайгаа номуудаас уншив"])
        .output()
        .expect("run cron morph");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("CRON AGGLUTINATIVE MORPHOLOGICAL TOKENIZER"));
    assert!(stdout.contains("монгол"));
    assert!(stdout.contains("+чууд [Plural]"));
    assert!(stdout.contains("+тайгаа [CaseNoun]"));
    assert!(stdout.contains("FEWER TOKENS"));
}

#[test]
fn test_cli_morph_traditional_script() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["morph", "--traditional", "Монгол улс"])
        .output()
        .expect("run cron morph --traditional");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("TRADITIONAL MONGOLIAN SCRIPT"));
    assert!(stdout.contains("ᠮᠣᠩᠭᠣᠯ"));
    assert!(stdout.contains("ᠤᠯᠤᠰ"));
}

#[test]
fn test_cli_morph_json_output() {
    let out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["morph", "--json", "эрдэм ном"])
        .output()
        .expect("run cron morph --json");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"token_reduction_pct\":"));
    assert!(stdout.contains("\"cron_morph_token_count\":"));
}
