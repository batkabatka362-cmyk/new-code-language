@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:     FeatureBatch[16×16:FP32]    @sram[0x0000]
    stats:  MeanStdDev[16×16:FP32]      @sram[0x0200]
    out:    Normalized[16×16:FP32]      @sram[0x0400]
}

§DataScalingBlock[16×16:FP32] {
    S_Factor: T[16×16:FP32], MinVal: T[16×16:FP32]

    FeatureBatch ↦ Normalized :
        Centered := FeatureBatch - MinVal
        Y_s := Centered * S_Factor
        Normed := N(Y_s)
        Normalized := silu(Normed)
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
!Δ { |Normalized| < 1e4, latency < 10ns }
