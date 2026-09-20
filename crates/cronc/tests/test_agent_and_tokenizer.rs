// ============================================================================
// CRON Native AI Agent Tool Runtime & BPE Tokenizer Verification Test
// Tests libcr/tokenizer/bpe.cr, libcr/agent/runtime.cr, and examples/agentic_tool_reasoning.cr
// (Zero External Python, 100% Native Compilation & Real Machine Execution)
// ============================================================================

use cronc::compile_to_c23_with_name;
use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_native_bpe_tokenizer_and_agentic_tool_execution() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    // 1. Verify Standard Library Files Exist
    let bpe_path = repo_root.join("libcr").join("tokenizer").join("bpe.cr");
    let agent_path = repo_root.join("libcr").join("agent").join("runtime.cr");
    let example_path = repo_root.join("examples").join("agentic_tool_reasoning.cr");

    assert!(bpe_path.exists(), "bpe.cr must exist");
    assert!(agent_path.exists(), "runtime.cr must exist");
    assert!(example_path.exists(), "agentic_tool_reasoning.cr must exist");

    // 2. Transpile agentic_tool_reasoning.cr to Native C23
    let source = fs::read_to_string(&example_path)
        .expect("Failed to read agentic_tool_reasoning.cr");

    let c_code = compile_to_c23_with_name(&source, Some(example_path.to_str().unwrap()))
        .expect("Failed to transpile agentic tool reasoning script to C23");

    assert!(
        c_code.contains("CRON Native C23"),
        "Emitted C23 must contain standard header"
    );

    // 3. Native GCC Compilation & Real Execution Verification
    let temp_dir = repo_root.join("scratch");
    let _ = fs::create_dir_all(&temp_dir);
    let c_file = temp_dir.join("test_agent_reasoning.c");
    let exe_file = temp_dir.join("test_agent_reasoning.exe");

    fs::write(&c_file, &c_code).expect("Failed to write temporary C file");

    let compile_status = Command::new("gcc")
        .args(&[
            "-O3",
            c_file.to_str().unwrap(),
            "-o",
            exe_file.to_str().unwrap(),
            "-lm",
        ])
        .status()
        .expect("Failed to invoke GCC compiler");

    assert!(compile_status.success(), "GCC compilation of native agent pipeline must succeed");

    // 4. Run native executable and verify output
    let run_output = Command::new(&exe_file)
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run compiled agent executable");

    let stdout = String::from_utf8_lossy(&run_output.stdout);
    println!("Native Agent Output:\n{}", stdout);

    assert!(
        stdout.contains("User Main Returned: 1"),
        "Agent reasoning must return exit code 1 indicating full verified chain-of-thought completion"
    );
}
