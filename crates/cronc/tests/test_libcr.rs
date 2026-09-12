use cronc::lexer::Lexer;
use cronc::parser::Parser;
use std::fs;
use std::path::{Path, PathBuf};

fn find_libcr_dir() -> PathBuf {
    let candidates = [
        PathBuf::from("libcr"),
        PathBuf::from("../../libcr"),
        PathBuf::from("../libcr"),
    ];
    for c in &candidates {
        if c.is_dir() {
            return c.clone();
        }
    }
    panic!("Could not locate libcr directory from current working directory: {:?}", std::env::current_dir());
}

fn collect_cr_files(dir: &Path) -> Vec<PathBuf> {
    let mut results = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                results.extend(collect_cr_files(&path));
            } else if path.extension().and_then(|s| s.to_str()) == Some("cr") {
                results.push(path);
            }
        }
    }
    results.sort();
    results
}

#[test]
fn test_libcr_standard_library_files() {
    let libcr_dir = find_libcr_dir();
    let files = collect_cr_files(&libcr_dir);

    assert!(
        files.len() >= 20,
        "Expected at least 20 standard library files in libcr, found: {}",
        files.len()
    );

    let mut failed_files = Vec::new();

    for file in &files {
        let content = fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", file.display(), e));

        // 1. Test Lexer
        let mut lexer = Lexer::new(&content);
        let tokens = match lexer.tokenize() {
            Ok(toks) => toks,
            Err(e) => {
                failed_files.push(format!("LEXER ERROR in {}: {}", file.display(), e));
                continue;
            }
        };

        // 2. Test Parser
        let mut parser = Parser::new(tokens);
        if let Err(e) = parser.parse_program() {
            failed_files.push(format!("PARSER ERROR in {}: {}", file.display(), e));
        }
    }

    if !failed_files.is_empty() {
        panic!(
            "Standard Library Validation Failed on {} files:\n{}",
            failed_files.len(),
            failed_files.join("\n")
        );
    }
}
