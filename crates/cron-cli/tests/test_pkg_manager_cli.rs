// ============================================================================
// CRON CLI Package Manager Integration Tests
// ============================================================================

use std::fs;
use std::process::Command;

#[test]
fn test_cli_pkg_lifecycle() {
    let test_pkg_dir = "test_sample_pkg";
    let _ = fs::remove_dir_all(test_pkg_dir);

    // 1. Initialize project
    let init_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["pkg", "init", "my_silicon_kernel", test_pkg_dir])
        .output()
        .expect("Failed to run cron pkg init");

    assert!(init_out.status.success(), "cron pkg init must succeed");
    let init_str = String::from_utf8_lossy(&init_out.stdout);
    assert!(init_str.contains("[SUCCESS]"));
    assert!(init_str.contains("Initialized CRON package"));

    assert!(std::path::Path::new(test_pkg_dir).join("cron.toml").exists());
    assert!(std::path::Path::new(test_pkg_dir).join("src").join("lib.cl").exists());
    assert!(std::path::Path::new(test_pkg_dir).join("tests").join("test_kernel.cl").exists());
    assert!(std::path::Path::new(test_pkg_dir).join("cron.lock").exists());

    // 2. Build project
    let build_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .current_dir(test_pkg_dir)
        .args(["pkg", "build"])
        .output()
        .expect("Failed to run cron pkg build");

    assert!(build_out.status.success(), "cron pkg build must succeed");
    let build_str = String::from_utf8_lossy(&build_out.stdout);
    assert!(build_str.contains("[BUILD SUCCESS]"));
    assert!(build_str.contains("Compiled package 'my_silicon_kernel'"));

    // 3. Test project
    let test_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .current_dir(test_pkg_dir)
        .args(["pkg", "test"])
        .output()
        .expect("Failed to run cron pkg test");

    assert!(test_out.status.success(), "cron pkg test must succeed");
    let test_str = String::from_utf8_lossy(&test_out.stdout);
    assert!(test_str.contains("[TEST SUCCESS]"));
    assert!(test_str.contains("100% Passed"));

    // 4. Add dependency
    let add_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .current_dir(test_pkg_dir)
        .args(["pkg", "add", "cron/crypto"])
        .output()
        .expect("Failed to run cron pkg add");

    assert!(add_out.status.success(), "cron pkg add must succeed");
    let add_str = String::from_utf8_lossy(&add_out.stdout);
    assert!(add_str.contains("[SUCCESS]"));
    assert!(add_str.contains("Added 'cron/crypto' to cron.toml"));

    // 5. Verify integrity
    let verify_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .current_dir(test_pkg_dir)
        .args(["pkg", "verify"])
        .output()
        .expect("Failed to run cron pkg verify");

    assert!(verify_out.status.success(), "cron pkg verify must succeed");
    let verify_str = String::from_utf8_lossy(&verify_out.stdout);
    assert!(verify_str.contains("[VERIFY SUCCESS]"));

    // 6. Tree / List
    let tree_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .current_dir(test_pkg_dir)
        .args(["pkg", "tree"])
        .output()
        .expect("Failed to run cron pkg tree");

    assert!(tree_out.status.success(), "cron pkg tree must succeed");
    let tree_str = String::from_utf8_lossy(&tree_out.stdout);
    assert!(tree_str.contains("my_silicon_kernel"));
    assert!(tree_str.contains("cron/crypto"));

    // 7. Search
    let search_out = Command::new(env!("CARGO_BIN_EXE_cron"))
        .args(["pkg", "search", "math"])
        .output()
        .expect("Failed to run cron pkg search");

    assert!(search_out.status.success(), "cron pkg search must succeed");
    let search_str = String::from_utf8_lossy(&search_out.stdout);
    assert!(search_str.contains("cron/math"));

    // 8. Clean up
    let _ = fs::remove_dir_all(test_pkg_dir);
}
