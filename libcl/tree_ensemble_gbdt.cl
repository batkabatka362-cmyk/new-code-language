@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    features:   FeatureBins[16×16:FP32]     @sram[0x0000]
    hessians:   HessianGrads[16×16:FP32]    @sram[0x0200]
    leaf_split: LeafOutputs[16×16:FP32]     @sram[0x0400]
}

§TreeBoostBlock[16×16:FP32] {
    RegularizationL2: T[16×16:FP32], LearningRateEta: T[16×16:FP32]

    FeatureBins ↦ LeafOutputs :
        GradScores := FeatureBins ⊗ HessianGrads
        Norm_G := N(GradScores)
        OptimalSplit := Norm_G + RegularizationL2
        LeafOutputs := silu(OptimalSplit)
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

!Δ { |LeafOutputs| < 1e5, gain_split > 0.01 }
