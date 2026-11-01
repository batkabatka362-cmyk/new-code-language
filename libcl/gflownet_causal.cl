@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:     ParentFlow[16×16:FP32] @sram[0x0000]
    reward: RewardState[16×16:FP32] @sram[0x0200]
    out:    ChildFlow[16×16:FP32]  @sram[0x0400]
}

§GflownetCausalBlock[16×16:FP32] {
    ForwardPolicy: T[16×16:FP32], BackwardPolicy: T[16×16:FP32]

    ParentFlow ↦ ChildFlow :
        DagCoupled := ParentFlow <~> RewardState
        TrajectoryBalance := DagCoupled * ForwardPolicy
        Norm_F := N(TrajectoryBalance)
        ChildFlow := silu(Norm_F)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        #R4 := #R3 + #R2
        @sram[0x0400] ← #R4
    }
}
!Δ { |ChildFlow| < 1e4, latency < 10ns }
