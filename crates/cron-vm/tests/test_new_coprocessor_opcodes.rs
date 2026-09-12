use cron_vm::core_engine::CoreEngine;

#[test]
fn test_hyper_edge_associator_he() {
    let mut core = CoreEngine::new(0);
    core.registers[1] = 0x12;
    core.registers[2] = 0x34;
    // _HE01$200>: dest=R1, src=R2
    core.execute_slot("_HE01$200>");
    assert_eq!(core.registers[1] & 0xFFFF_0000, 0xCAFE_0000, "Must have hyper-edge concept tag");
    assert_eq!((core.registers[1] >> 8) & 0xFF, 0x34, "Must embed source concept");
    assert_eq!(core.registers[1] & 0xFF, 0x12, "Must preserve dest concept");
}

#[test]
fn test_neuromorphic_lif_spike() {
    let mut core = CoreEngine::new(0);
    // Sub-threshold input
    core.registers[1] = 10; // initial membrane
    core.registers[2] = 15; // current
    core.execute_slot("_LI01$205>");
    assert_eq!(core.registers[1], 0, "Sub-threshold should not emit spike");

    // Super-threshold input (current = 60, threshold = 50)
    core.registers[1] = 20;
    core.registers[2] = 60;
    core.execute_slot("_LI01$205>");
    assert_eq!(core.registers[1], 1, "Super-threshold must emit spike");
}

#[test]
fn test_chaos_diffusion_od() {
    let mut core = CoreEngine::new(0);
    core.registers[1] = 42;
    core.execute_slot("_OD01$100>");
    assert_ne!(core.registers[1], 42, "Lorenz attractor must mutate state deterministically");
    let state1 = core.registers[1];
    core.execute_slot("_OD01$100>");
    assert_ne!(core.registers[1], state1, "Consecutive cycles must advance chaos trajectory");
}

#[test]
fn test_cross_attention_gate_ca() {
    let mut core = CoreEngine::new(0);
    core.registers[1] = 256; // activation
    core.registers[2] = 128; // gate = 128/256 = 0.5
    core.execute_slot("_CA01$200>");
    assert_eq!(core.registers[1], 128, "Gate must scale activation by 50%");
}

#[test]
fn test_arbiter_weight_update_aw() {
    let mut core = CoreEngine::new(0);
    core.registers[1] = 50;
    core.registers[2] = 5;
    core.execute_slot("_AW01$200>");
    assert_eq!(core.registers[1], 55, "Arbiter weight must increment by delta");
}

#[test]
fn test_cordic_sincos_cd() {
    let mut core = CoreEngine::new(0);
    core.registers[1] = 90; // 90 degrees: sin=1 (32767), cos=0
    core.execute_slot("_CD01$100>");
    let sin_val = (core.registers[1] & 0xFFFF) as u16;
    let cos_val = ((core.registers[1] >> 16) & 0xFFFF) as u16;
    assert!(sin_val > 32000, "sin(90 deg) must approach 32767: was {}", sin_val);
    assert!(cos_val < 50, "cos(90 deg) must approach 0: was {}", cos_val);
}

#[test]
fn test_atomic_cas_cs() {
    let mut core = CoreEngine::new(0);
    core.registers[1] = 100; // memory word
    core.registers[2] = 100; // expected value
    // _CS01$204>: dest=R1, src=R2, imm=4
    core.execute_slot("_CS01$204>");
    assert_eq!(core.registers[0], 1, "CAS must succeed when expected matches");
    assert_eq!(core.registers[1], 4, "Destination must be updated with imm");

    // Second CAS with mismatched expected value
    core.registers[2] = 999;
    core.execute_slot("_CS01$207>");
    assert_eq!(core.registers[0], 0, "CAS must fail when expected does not match");
    assert_eq!(core.registers[1], 4, "Destination must remain unchanged on failure");
}

#[test]
fn test_parallel_prefix_sum_ps() {
    let mut core = CoreEngine::new(0);
    // 4 bytes: 1, 2, 3, 4
    core.registers[1] = 0x04030201;
    core.execute_slot("_PS01$100>");
    let s0 = core.registers[1] & 0xFF;
    let s1 = (core.registers[1] >> 8) & 0xFF;
    let s2 = (core.registers[1] >> 16) & 0xFF;
    let s3 = (core.registers[1] >> 24) & 0xFF;
    assert_eq!(s0, 1);
    assert_eq!(s1, 1 + 2);
    assert_eq!(s2, 1 + 2 + 3);
    assert_eq!(s3, 1 + 2 + 3 + 4);
}

#[test]
fn test_tensor_transpose_tt() {
    let mut core = CoreEngine::new(0);
    core.registers[1] = 0x1234_5678;
    core.execute_slot("_TT02$100>");
    assert_ne!(core.registers[2], 0, "Transpose result must be non-zero");
    // Transpose of transpose must recover original
    core.execute_slot("_TT03$200>");
    assert_eq!(core.registers[3], core.registers[1], "Double transpose must be identity");
}

#[test]
fn test_noc_wormhole_tunnel_wh() {
    let mut core = CoreEngine::new(0);
    core.registers[2] = 0x0011_2233;
    core.execute_slot("_WH01$200>");
    assert_eq!(core.registers[1] & 0xFF00_0000, 0x5500_0000, "Wormhole flit header must be 0x55");
    assert_eq!(core.registers[1] & 0x00FF_FFFF, 0x0011_2233, "Payload must be preserved");
}

#[test]
fn test_spatial_broadcast_sb() {
    let mut core = CoreEngine::new(0);
    core.registers[4] = 0xABCD_1234;
    let broadcast_payload = core.execute_slot("_SB04$0105>");
    assert_eq!(core.spatial_broadcast_count, 1, "Broadcast counter must increment");
    assert_eq!(broadcast_payload, Some(0xABCD_1234), "Must broadcast dest register payload");
}
