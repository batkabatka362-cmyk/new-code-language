@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:  NodeFeatures[16×16:FP32] @sram[0x0000]
    adj: AdjMatrix[16×16:FP32]    @sram[0x0200]
    out: UpdatedNodes[16×16:FP32] @sram[0x0400]
}

§GraphAIEngineBlock[16×16:FP32] {
    Weight: T[16×16:FP32], MctsTree: T[16×16:FP32]

    NodeFeatures ↦ UpdatedNodes :
        Mess := NodeFeatures <~> AdjMatrix
        Norm := rmsnorm(Mess)
        Gated := Norm * Weight
        UpdatedNodes := silu(Gated)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |UpdatedNodes| < 1e4, latency < 10ns }
