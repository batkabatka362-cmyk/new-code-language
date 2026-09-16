use cronc::compile_native_binary;
use std::env;
use std::process::Command;

#[test]
fn test_standalone_native_binary_linking_and_execution() {
    let cron_code = r#"
    .MODULE NativeCompute

    def compute_result() -> int {
        let x = 100
        let y = 250
        return x + y
    }

    let res = compute_result()
    "#;

    let temp_dir = env::temp_dir();
    let exe_path = temp_dir.join("cron_test_native_runner.exe");

    let result = compile_native_binary(cron_code, &exe_path, &[]);
    assert!(result.is_ok(), "Native binary linking failed: {:?}", result.err());

    // Execute the compiled native binary
    let output = Command::new(&exe_path)
        .output()
        .expect("Failed to execute native binary");

    assert!(output.status.success(), "Native binary exited with error: {:?}", output.status);

    // Clean up
    let _ = std::fs::remove_file(&exe_path);
}
