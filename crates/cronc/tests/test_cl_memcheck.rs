use cronc::cl_memcheck::{
    compute_linear_bank, compute_swizzled_bank, prove_strided_conflict_freedom,
    verify_cl_memory_access, MemcheckOptions, NUM_BANKS,
};

#[test]
fn test_cl_memcheck_galois_field_swizzle_mathematical_proof() {
    // Mathematical Proof of Zero Bank Conflicts on GF(2^4):
    // For stride = 16 words (64 bytes), sequential rows r = 0..15:
    // Linear addressing puts every access into Bank 0 (16-way conflict!).
    // XOR Swizzling permutes every row into a unique bank in 0..15.
    let mut swizzled_banks = std::collections::HashSet::new();
    let mut linear_banks = std::collections::HashSet::new();

    let stride_bytes = 64; // 16 32-bit words
    for row in 0..16 {
        let addr = (row * stride_bytes) as u32;
        let s_bank = compute_swizzled_bank(addr);
        let l_bank = compute_linear_bank(addr);

        swizzled_banks.insert(s_bank);
        linear_banks.insert(l_bank);
    }

    // Linear addressing collides 100% on a single bank
    assert_eq!(linear_banks.len(), 1, "Linear addressing must collapse onto a single bank");

    // XOR swizzled addressing visits all 16 banks orthogonally
    assert_eq!(swizzled_banks.len(), NUM_BANKS, "Swizzled addressing must be a strict bijection on all 16 banks");

    // Formally verify using helper
    assert!(prove_strided_conflict_freedom(stride_bytes, 16, true));
    assert!(!prove_strided_conflict_freedom(stride_bytes, 16, false));
}

#[test]
fn test_cl_memcheck_linear_unswizzled_bank_conflict_detection() {
    // Program where multiple slots in B0001 access memory with identical linear banks
    // Slot 0 (R1=0) accesses addr 0 -> Linear Bank 0, Swizzled Bank 0
    // Slot 1 (R2=0x30) accesses addr 48 + 16 = 64 -> Linear Bank 0 (Collision!), Swizzled Bank 1 (Orthogonal!)
    let cl_code = r#"
    B0000: '==01#000> '==02#030> _NO00#000> _NO00#000>
    B0001: _TT04$100> _TT05$200> _NO00#000> _NO00#000>
    B0002: _HL00$008! _NO00#000> _NO00#000> _NO00#000>
    "#;

    // 1. Linear addressing must detect the 2-way bank collision
    let opts_linear = MemcheckOptions {
        enable_swizzling: false,
        ..Default::default()
    };
    let rep_linear = verify_cl_memory_access(cl_code, &opts_linear).expect("Verification should succeed");

    assert!(rep_linear.total_memory_accesses >= 2);
    assert_eq!(rep_linear.conflicted_cycles, 1, "Linear addressing must detect a cycle bank conflict");
    assert!(rep_linear.total_stall_penalty_cycles >= 1);
    assert!(!rep_linear.is_provably_conflict_free);

    // 2. Swizzled addressing must eliminate the collision completely!
    let opts_swizzled = MemcheckOptions {
        enable_swizzling: true,
        ..Default::default()
    };
    let rep_swizzled = verify_cl_memory_access(cl_code, &opts_swizzled).expect("Swizzled verification");
    assert!(rep_swizzled.is_provably_conflict_free, "Swizzled addressing must be conflict-free");
    assert_eq!(rep_swizzled.conflicted_cycles, 0);
}

#[test]
fn test_cl_memcheck_swizzled_conflict_free_verification() {
    let cl_code = r#"
    B0000: '==01#000> '==02#004> _NO00#000> _NO00#000>
    B0001: _TT04$100> _MD05*200> _NO00#000> _NO00#000>
    B0002: _HL00$008! _NO00#000> _NO00#000> _NO00#000>
    "#;

    let opts_swizzled = MemcheckOptions {
        enable_swizzling: true,
        ..Default::default()
    };
    let rep_swizzled = verify_cl_memory_access(cl_code, &opts_swizzled).expect("Verification should succeed");

    assert!(rep_swizzled.total_memory_accesses >= 2);
    assert!(rep_swizzled.is_provably_conflict_free, "Swizzled access must be provably conflict-free");
    assert_eq!(rep_swizzled.conflicted_cycles, 0);
    assert_eq!(rep_swizzled.total_stall_penalty_cycles, 0);
}

#[test]
fn test_cl_memcheck_multi_core_pgas_partitioning() {
    let cl_multicore = r#"
    .core [0, 0, 0, 0]:
    B0000: '==01#000> _NO00#000> _NO00#000> _NO00#000>
    B0001: _TT04$100> _NO00#000> _NO00#000> _HL00$008!

    .core [1, 2, 0, 0]:
    B0000: '==01#010> _NO00#000> _NO00#000> _NO00#000>
    B0001: _TT04$100> _NO00#000> _NO00#000> _HL00$008!
    "#;

    let report = verify_cl_memory_access(cl_multicore, &MemcheckOptions::default()).expect("Verify multicore");
    assert!(report.total_memory_accesses >= 2);
    assert!(report.is_provably_conflict_free);
    assert!(report.out_of_bounds_accesses.is_empty());
}

#[test]
fn test_cl_memcheck_json_serialization_validity() {
    let cl_code = r#"
    B0000: '==01#004> _NO00#000> _NO00#000> _NO00#000>
    B0001: _TT04$100> _NO00#000> _NO00#000> _HL00$008!
    "#;

    let report = verify_cl_memory_access(cl_code, &MemcheckOptions::default()).expect("Verify");
    let json = report.to_json();

    assert!(json.contains("\"total_memory_accesses\": 1"));
    assert!(json.contains("\"is_provably_conflict_free\": true"));
    assert!(json.contains("\"bank_access_histogram\": ["));
    assert!(json.contains("\"conflict_details\": []"));
    assert!(json.contains("\"out_of_bounds_accesses\": []"));
}

#[test]
fn test_cl_memcheck_ascii_report_format() {
    let cl_code = r#"
    B0000: '==01#004> _NO00#000> _NO00#000> _NO00#000>
    B0001: _TT04$100> _NO00#000> _NO00#000> _HL00$008!
    "#;

    let report = verify_cl_memory_access(cl_code, &MemcheckOptions::default()).expect("Verify");
    let trace = report.format_ascii_report("test_kernel.cl");

    assert!(trace.contains("CRON 256-CORE 4D-TORUS PGAS 16-BANK MEMORY CONFLICT VERIFIER"));
    assert!(trace.contains("PROVABLY CONFLICT-FREE"));
    assert!(trace.contains("Physical 16-Bank Distribution Histogram"));
    assert!(trace.contains("Bank 00:"));
}
