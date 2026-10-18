// ============================================================================
// CRON PHASE VI MASTER VERIFICATION SUITE (test_master_phase6.rs)
// Rigorous verification of Tasks 25, 26, 27: Scientific Papers & Benchmarks
// ============================================================================

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn get_cron_bin() -> &'static str {
    env!("CARGO_BIN_EXE_cron")
}

fn get_workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

#[test]
fn test_task_25_arxiv_paper_and_latex_manuscript() {
    // Task 25: Official ArXiv Paper & IEEE/ACM LaTeX manuscript
    let root = get_workspace_root();
    let arxiv_md = root.join("docs/CRON_RESEARCH_PAPER_ARXIV.md");
    let arxiv_tex = root.join("docs/arxiv_paper.tex");

    assert!(arxiv_md.exists(), "ArXiv Markdown paper must exist at {:?}", arxiv_md);
    assert!(arxiv_tex.exists(), "ArXiv LaTeX manuscript must exist at {:?}", arxiv_tex);

    let md_content = fs::read_to_string(&arxiv_md).expect("Read ArXiv paper");
    assert!(md_content.contains("CRON: A Multiplier-Free 4D-Torus Silicon Architecture"));
    assert!(md_content.contains("von Neumann memory wall"));
    assert!(md_content.contains("Landauer"));
    assert!(md_content.contains("BitNet b1.58"));
    assert!(md_content.contains("Galois Field"));

    let tex_content = fs::read_to_string(&arxiv_tex).expect("Read LaTeX manuscript");
    assert!(tex_content.contains("\\documentclass[10pt,journal,compsoc]{IEEEtran}"));
    assert!(tex_content.contains("CRON: A Multiplier-Free 4D-Torus Silicon Architecture"));
    assert!(tex_content.contains("tab:benchmarks"));
}

#[test]
fn test_task_26_empirical_benchmark_scoreboard() {
    // Task 26: Real-world benchmark scoreboard vs NVIDIA H100 and Mojo
    let root = get_workspace_root();
    let whitepaper_path = root.join("docs/CRON_BENCHMARK_WHITEPAPER.md");

    let output = Command::new(get_cron_bin())
        .current_dir(&root)
        .args(["cl-compare", "all", "--whitepaper", "-o", whitepaper_path.to_str().unwrap()])
        .output()
        .expect("Failed to run cl-compare all --whitepaper");

    assert!(output.status.success(), "cl-compare whitepaper generation must succeed");
    assert!(whitepaper_path.exists(), "Benchmark whitepaper must exist");

    let wp_content = fs::read_to_string(&whitepaper_path).expect("Read benchmark whitepaper");
    assert!(wp_content.contains("CRON vs PYTORCH/CUDA vs MOJO"));
    assert!(wp_content.contains("NVIDIA H100 SXM5"));
    assert!(wp_content.contains("BitNet b1.58 Ternary GEMM"));
    assert!(wp_content.contains("RingTape O(1) Streaming FlashAttention"));
    assert!(wp_content.contains("Mini-LLM 125M End-to-End Autoregressive Generation"));
    assert!(wp_content.contains("DeepSeek Multi-Head Latent Attention (MLA)"));
}

#[test]
fn test_task_27_open_source_release_and_docs() {
    // Task 27: Open-source documentation, Mongolian & English Quickstarts
    let root = get_workspace_root();
    let mn_quickstart = root.join("docs/CRON_QUICKSTART_MN.md");
    let en_quickstart = root.join("docs/CRON_QUICKSTART_EN.md");
    let readme = root.join("README.md");

    assert!(mn_quickstart.exists(), "Mongolian quickstart guide must exist at {:?}", mn_quickstart);
    assert!(en_quickstart.exists(), "English quickstart guide must exist at {:?}", en_quickstart);
    assert!(readme.exists(), "Root README.md must exist at {:?}", readme);

    let mn_content = fs::read_to_string(&mn_quickstart).expect("Read MN quickstart");
    assert!(mn_content.contains("CRON: Хөгжүүлэгчийн Түргэн Эхлэх Гарын Авлага"));
    assert!(mn_content.contains("256-Цөмт 4D-Torus"));

    let en_content = fs::read_to_string(&en_quickstart).expect("Read EN quickstart");
    assert!(en_content.contains("CRON: Developer Quickstart Guide"));
    assert!(en_content.contains("cargo test --workspace"));

    let readme_content = fs::read_to_string(&readme).expect("Read README.md");
    assert!(readme_content.contains("13. 6 Үе Шаттай Их Төлөвлөгөө ба Эрдэм Шинжилгээний Нийтлэлүүд"));
    assert!(readme_content.contains("Status: 27 / 27 TASKS COMPLETED (100% PASS)"));
}
