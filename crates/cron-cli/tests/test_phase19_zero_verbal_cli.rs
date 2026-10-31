use std::path::{Path, PathBuf};
use std::process::Command;

fn get_workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

#[test]
fn test_cli_zero_verbal_math_pipeline_and_codegen() {
    let cron_bin = env!("CARGO_BIN_EXE_cron");
    let root = get_workspace_root();

    // 1. Test E2E Text LM execution
    let out_e2e = Command::new(cron_bin)
        .current_dir(&root)
        .args(["math", "examples/zero_word_e2e_text_lm.cl"])
        .output()
        .expect("Failed to execute cron math examples/zero_word_e2e_text_lm.cl");

    assert!(out_e2e.status.success(), "E2E LM failed: {:?}", String::from_utf8_lossy(&out_e2e.stderr));
    let stdout_e2e = String::from_utf8_lossy(&out_e2e.stdout);
    assert!(stdout_e2e.contains("CRON MATHEMATICAL CALCULUS ENGINE"));
    assert!(stdout_e2e.contains("Evaluation Result"));

    // 2. Test Tensor Outer Product ⊗ & Matrix Calculus execution
    let out_mat = Command::new(cron_bin)
        .current_dir(&root)
        .args(["math", "examples/matrix_calculus.cl"])
        .output()
        .expect("Failed to execute cron math examples/matrix_calculus.cl");

    assert!(out_mat.status.success());
    let stdout_mat = String::from_utf8_lossy(&out_mat.stdout);
    assert!(stdout_mat.contains("270"));

    // 3. Test Standard Neural Calculus (Level 1A) execution
    let out_calc = Command::new(cron_bin)
        .current_dir(&root)
        .args(["math", "examples/standard_neural_calculus.cl"])
        .output()
        .expect("Failed to execute cron math examples/standard_neural_calculus.cl");

    assert!(out_calc.status.success());
    let stdout_calc = String::from_utf8_lossy(&out_calc.stdout);
    assert!(stdout_calc.contains("[32, 34]"));

    // 4. Test C23 Native Transpilation flag (--c23)
    let out_c23 = Command::new(cron_bin)
        .current_dir(&root)
        .args(["math", "examples/matrix_calculus.cl", "--c23"])
        .output()
        .expect("Failed to execute cron math --c23");

    assert!(out_c23.status.success());
    let stdout_c23 = String::from_utf8_lossy(&out_c23.stdout);
    assert!(stdout_c23.contains("ISO C23 Native Source") || stdout_c23.contains("<math.h>"));

    // 5. Test Dependency Graph Generation flag (--graph)
    let out_graph = Command::new(cron_bin)
        .current_dir(&root)
        .args(["math", "examples/matrix_calculus.cl", "--graph"])
        .output()
        .expect("Failed to execute cron math --graph");

    assert!(out_graph.status.success());
    let stdout_graph = String::from_utf8_lossy(&out_graph.stdout);
    assert!(stdout_graph.contains("flowchart TD") || stdout_graph.contains("-->"));

    // 6. Test Real Safetensors GQA Transformer Inference
    let out_infer = Command::new(cron_bin)
        .current_dir(&root)
        .args(["math", "examples/safetensors_transformer_inference.cl"])
        .output()
        .expect("Failed to execute cron math examples/safetensors_transformer_inference.cl");

    assert!(out_infer.status.success(), "Safetensors inference failed: {:?}", String::from_utf8_lossy(&out_infer.stderr));
    let stdout_infer = String::from_utf8_lossy(&out_infer.stdout);
    assert!(stdout_infer.contains("CRON MATHEMATICAL CALCULUS ENGINE"));
    assert!(stdout_infer.contains("Constants Bound"));
    assert!(stdout_infer.contains("Evaluation Result"));
}
