use std::process::Command;
use cronc::cl_c23::compile_cl_to_native_binary;
use cronc::cl_kernel::generate_flash_attention;

#[test]
fn test_cl_native_binary_compilation_and_execution() {
    // Check if gcc or clang is available
    let gcc_check = Command::new("gcc").arg("--version").output();
    let clang_check = Command::new("clang").arg("--version").output();

    if gcc_check.is_err() && clang_check.is_err() {
        eprintln!("Neither gcc nor clang found on system, skipping native execution test");
        return;
    }

    let cl_code = generate_flash_attention(16, 64);
    let temp_dir = std::env::temp_dir();
    let out_bin = temp_dir.join(if cfg!(windows) { "test_cl_attn.exe" } else { "test_cl_attn" });

    let comp_res = compile_cl_to_native_binary(&cl_code, out_bin.to_str().unwrap(), "3");
    assert!(comp_res.is_ok(), "Native compilation of .cl kernel must succeed: {:?}", comp_res.err());
    assert!(out_bin.exists(), "Output native binary must exist");

    // Execute the native binary directly on bare metal!
    let run_res = Command::new(out_bin.to_str().unwrap())
        .output()
        .expect("Failed to execute compiled native .cl binary");

    assert!(run_res.status.success(), "Native .cl binary execution must exit with 0");
    let stdout = String::from_utf8_lossy(&run_res.stdout);
    assert!(stdout.contains("CRON SILICON C23 NATIVE EXECUTION TELEMETRY"));
    assert!(stdout.contains("STATUS: 100% BIT-EXACT SILICON C23 PARITY VERIFIED"));

    let _ = std::fs::remove_file(&out_bin);
}
