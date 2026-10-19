// ============================================================================
// CRON CL v2.0 TEST SUITE: 5-Pillar VLIW Silicon Execution
// Verifies real execution of new Math opcodes (_MD, _DV, _FM, _TG, _RC)
// ============================================================================

use cronc::cl_v2_parser::parse_cl_v2;
use cronc::cl_jit::{ClJitCore, execute_cl_on_core};

#[test]
fn test_v2_vliw_math_execution() {
    let source = r#"
// ============================================================================
// [1] SILICON TARGET CONTRACT
// ============================================================================
@kernel cluster_allreduce_mamba2
.target silicon.distributed_swarm
.ipc_target 4.0
.cores 256
@mesh [4×4×4×4] : ⮑ (x+1, y, z, w)

// ============================================================================
// [2] I/O MEMORY BINDING
// ============================================================================
@io {
    in:  X[16×16:NVFP4]  @sram[0x0000]
    out: Y[16×16:NVFP4]  @sram[0x0400]
    wt:  W[16×16:NVFP4]  @dram[0x8000]
}

// ============================================================================
// [3] MATHEMATICAL TENSOR MORPHISM
// ============================================================================
§Mamba2Block[16×16:NVFP4] {
    H: SS[64×256], W: T[16×16:NVFP4]
    
    X ↦ silu( (rmsnorm(X) <~> ssd(X, H)) ~*~ ssd(X, H) )
}

// ============================================================================
// [4] CIPHERED CONTROL-FLOW
// ============================================================================
§ControlFlow {
    ⟳ (100) {
        #R1 ← @dmem[0x1000]
        ? (#R1 > 0x3C00) ⮞ {
            #R2 := #R1 * 0.95
            @dmem[0x1000] ← #R2
        } : {
            ⮌
        }
    }
}
!Δ { |Y| < 1e4, latency < 15ns }

// ============================================================================
// [5] 10/16-CHAR VLIW SILICON CIPHER
// ============================================================================
B0000: '==01#0A0K '==02#080V _MD03*120F _FM04+320A
B0001: _DV05/310P _RC06!500Z _TG07*AA0M _SB08-760U
B0002: _MZ09~450X _PO0A$980L ~RM0B^0A0C !HL00#000(
    "#;

    let doc = parse_cl_v2(source).expect("Should parse cleanly");
    assert_eq!(doc.vliw_stream.len(), 3);

    let mut core = ClJitCore::new();
    execute_cl_on_core(source, &mut core).expect("Execution on core must succeed");

    assert!(core.is_halted, "Core must halt on !HL");
    assert_eq!(core.cycle_count, 3, "Must execute exactly 3 cycles");

    // Cycle 0:
    // '==01#0A0 -> R1 = 0xA0 = 160
    // '==02#080 -> R2 = 0x80 = 128
    // _MD03*120 -> Multiply R3 = R1 * R2 = 160 * 128 = 20480
    // _FM04+320 -> FMA R4 = R3 * R2 + R4 = 20480 * 128 + 0 = 2621440
    assert_eq!(core.r[1], 160);
    assert_eq!(core.r[2], 128);
    assert_eq!(core.r[3], 20480);
    assert_eq!(core.r[4], 2621440);

    // Cycle 1:
    // _DV05/310 -> Divide R5 = R3 / R1 = 20480 / 160 = 128
    assert_eq!(core.r[5], 128);
    // _RC06!500 -> Reciprocal R6 = 1000000 / sqrt(128) = 1000000 / 11.3137 = 88388
    assert_eq!(core.r[6], 88388);
}
