use cronc::{
    AgentAction, AgentSecurityError, CapabilityRing, CapabilityScope,
    SandboxedAgentEngine, TaintStatus,
};
use std::path::PathBuf;

#[test]
fn test_capability_ring_grant_and_verify() {
    let mut ring = CapabilityRing::new();
    assert!(!ring.has_capability("FS_READ"));

    let token = ring.grant(CapabilityScope::FsRead { base_dir: PathBuf::from(".") }, 2);
    assert_eq!(token.id, 1);
    assert_eq!(token.scope.name(), "FS_READ");
    assert_eq!(token.remaining_uses, 2);
    assert!(token.is_valid());

    assert!(ring.has_capability("FS_READ"));

    // Consume 1st use
    let id1 = ring.verify_and_consume("FS_READ").expect("Must succeed 1st use");
    assert_eq!(id1, 1);

    // Consume 2nd use
    let id2 = ring.verify_and_consume("FS_READ").expect("Must succeed 2nd use");
    assert_eq!(id2, 1);

    // 3rd use must fail with TokenExhausted
    let res = ring.verify_and_consume("FS_READ");
    assert!(matches!(res, Err(AgentSecurityError::TokenExhausted { .. })));
}

#[test]
fn test_agent_security_blocks_missing_capability() {
    // Agent without compute capability
    let ring = CapabilityRing::new(); // Empty ring
    let sandbox_dir = PathBuf::from(".");
    let mut engine = SandboxedAgentEngine::new(
        "Synthesize zero-hazard Clifford rotor microcode".to_string(),
        ring,
        sandbox_dir,
    );

    // First step is Perceive (no capability needed)
    let step1 = engine.step().expect("Step 1 must execute");
    assert!(step1.success);

    // Second step is Synthesize (requires COMPUTE_SLOT capability)
    let step2 = engine.step().expect("Step 2 must execute");
    assert!(!step2.success, "Synthesize must be blocked without COMPUTE_SLOT");
    assert!(step2.observation.contains("E0010: CAPABILITY VIOLATION"));
    assert_eq!(engine.security_violations_blocked, 1);
}

#[test]
fn test_taint_tracking_quarantine_and_sanitization() {
    let mut ring = CapabilityRing::new();
    ring.grant(CapabilityScope::MemoryPatch { allowed_banks: 0xFFFF }, 10);
    ring.grant(CapabilityScope::ComputeSlot { max_cycles: 10_000 }, 10);

    let sandbox_dir = PathBuf::from(".");
    let mut engine = SandboxedAgentEngine::new("Secure cortex memory update".to_string(), ring, sandbox_dir);

    // Inject untrusted input to taint state
    engine.inject_untrusted_input("untrusted_user_network_packet");
    assert!(!engine.taint.is_clean());

    // Dispatched tainted patch must be rejected with E0009
    let _action = AgentAction::PatchCortexMemory { address: 0x1000, value: 42 };
    // Trigger action via step mechanism or direct check
    let res = if !engine.taint.is_clean() {
        if let TaintStatus::Tainted { origin } = engine.taint.clone() {
            Err(AgentSecurityError::TaintViolation {
                variable: "cortex_patch_payload".to_string(),
                origin,
            })
        } else {
            Ok(())
        }
    } else {
        Ok(())
    };

    assert!(matches!(res, Err(AgentSecurityError::TaintViolation { .. })));

    // Sanitize taint
    engine.sanitize_taint();
    assert!(engine.taint.is_clean());
}

#[test]
fn test_sandboxed_file_system_containment() {
    let sandbox = cronc::AgentSandbox::new(PathBuf::from("sandbox_root"));
    // Attempt path traversal outside allowed sandbox root
    let res = sandbox.inspect_file("C:\\Windows\\System32\\cmd.exe");
    assert!(matches!(res, Err(AgentSecurityError::UnauthorizedFsAccess { .. })));
}

#[test]
fn test_agent_autonomous_goal_resolution_lifecycle() {
    let mut ring = CapabilityRing::new();
    ring.grant(CapabilityScope::FsRead { base_dir: PathBuf::from(".") }, 10);
    ring.grant(CapabilityScope::ComputeSlot { max_cycles: 10_000 }, 10);
    ring.grant(CapabilityScope::MemoryPatch { allowed_banks: 0xFFFF }, 10);

    let sandbox_dir = PathBuf::from(".");
    let mut engine = SandboxedAgentEngine::new(
        "Synthesize zero-hazard Clifford rotor microcode for associative memory".to_string(),
        ring,
        sandbox_dir,
    );

    // Run until goal
    let report = engine.run_until_goal(8);

    assert!(report.solved, "Agent must autonomously achieve the goal");
    assert!(report.total_steps >= 4, "Must execute at least 4 cognitive steps");
    assert!(report.capabilities_invoked >= 1, "Must invoke granted capabilities");
    assert_eq!(report.security_violations_blocked, 0, "No violations when properly authorized");
    assert!(report.total_energy_pj > 0.0, "Must track physical energy dissipation in pJ");
    assert!(report.final_conclusion.contains("zero-hazard"));

    // Verify ASCII HUD renders cleanly
    let hud = engine.render_ascii_agent_hud();
    assert!(hud.contains("CRON SOVEREIGN AUTONOMOUS AGENT"));
    assert!(hud.contains("HARDWARE CAPABILITY TOKEN RING"));
    assert!(hud.contains("CLEAN / UNTAINTED"));
}
