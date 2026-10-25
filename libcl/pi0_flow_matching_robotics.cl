@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:     VisualLatent[16×16:FP32]  @sram[0x0000]
    action: JointState14D[16×16:FP32] @sram[0x0200]
    out:    VelocityFlow[16×16:FP32]  @sram[0x0400]
}

§Pi0FlowMatchingRoboticsBlock[16×16:FP32] {
    VlaProjection: T[16×16:FP32], ContinuousVectorFlow: T[16×16:FP32]

    VisualLatent ↦ VelocityFlow :
        ConditionedAction := VisualLatent <~> JointState14D
        TrajectoryTangent := ConditionedAction * VlaProjection
        NormedFlow := rmsnorm(TrajectoryTangent)
        VelocityFlow := silu(NormedFlow)
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
!Δ { |VelocityFlow| < 1e4, latency < 10ns }
