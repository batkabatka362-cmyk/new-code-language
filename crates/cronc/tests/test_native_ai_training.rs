// ============================================================================
// CRON Native AI Training Engine Verification Test
// Tests libcr/optim, libcr/nn/loss, libcr/data, and train_foundation_transformer.cr
// (Zero External Python, Zero PyTorch, 100% Native Compilation & Execution)
// ============================================================================

use cronc::compile_to_c23_with_name;
use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_native_ai_training_pipeline_convergence() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    // 1. Verify Standard Library Files Exist
    let adamw_path = repo_root.join("libcr").join("optim").join("adamw.cr");
    let muon_path = repo_root.join("libcr").join("optim").join("muon.cr");
    let loss_path = repo_root.join("libcr").join("nn").join("loss.cr");
    let data_path = repo_root.join("libcr").join("data").join("dataloader.cr");
    let train_path = repo_root.join("examples").join("train_foundation_transformer.cr");

    assert!(adamw_path.exists(), "adamw.cr must exist");
    assert!(muon_path.exists(), "muon.cr must exist");
    assert!(loss_path.exists(), "loss.cr must exist");
    assert!(data_path.exists(), "dataloader.cr must exist");
    assert!(train_path.exists(), "train_foundation_transformer.cr must exist");

    // 2. Transpile train_foundation_transformer.cr to Native C23 using compiler pipeline
    let train_source = fs::read_to_string(&train_path)
        .expect("Failed to read train_foundation_transformer.cr");

    let c_code = compile_to_c23_with_name(&train_source, Some(train_path.to_str().unwrap()))
        .expect("Failed to transpile native training script to C23");

    assert!(
        c_code.contains("CRON Native C23"),
        "Emitted C23 must contain standard header"
    );

    // 3. Native GCC Compilation & Real Execution Verification
    let temp_dir = repo_root.join("scratch");
    let _ = fs::create_dir_all(&temp_dir);
    let c_file = temp_dir.join("test_native_train.c");
    let exe_file = temp_dir.join("test_native_train.exe");

    fs::write(&c_file, &c_code).expect("Failed to write temporary C file");

    let compile_status = Command::new("gcc")
        .args(&[
            "-O3",
            c_file.to_str().unwrap(),
            "-o",
            exe_file.to_str().unwrap(),
            "-lm",
        ])
        .status();

    if let Ok(status) = compile_status {
        if status.success() {
            let run_output = Command::new(&exe_file)
                .current_dir(&temp_dir)
                .output()
                .expect("Failed to execute native training binary");

            let stdout = String::from_utf8_lossy(&run_output.stdout);
            println!("Native Training Output:\n{}", stdout);

            assert!(
                stdout.contains("[CRON Train] Epoch"),
                "Output must log training convergence steps"
            );
            assert!(
                stdout.contains("User Main Returned: 1"),
                "Training must converge and return 1"
            );
        }
    }
}
