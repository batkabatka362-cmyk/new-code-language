@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    hidden_states:  H[16×16:FP32]           @sram[0x0000]
    router_weights: WGate[16×16:FP32]       @sram[0x0200]
    gate_probs:     ExpertWeights[16×16:FP32] @sram[0x0400]
}

§MoeGatingBlock[16×16:FP32] {
    ExpertBias: T[16×16:FP32], TopKCount: T[16×16:FP32]

    H ↦ ExpertWeights :
        Logits := H ⊗ WGate
        BiasedLogits := Logits + ExpertBias
        Norm_L := N(BiasedLogits)
        ExpertWeights := silu(Norm_L)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        #R4 := #R3 + #R1
        @sram[0x0400] ← #R4
    }
}

!Δ { |ExpertWeights| < 1e4, routing_entropy > 0.5 }
