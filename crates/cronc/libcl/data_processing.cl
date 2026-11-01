@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:    Raw[16×16:FP32]        @sram[0x0000]
    s:     S_Param[16×16:FP32] @sram[0x0200]
    out:   Y_s[16×16:FP32]     @sram[0x0400]
}

§DataProcessingBlock[16×16:FP32] {
    Mean: T[16×16:FP32], SvdVec: T[16×16:FP32]

    Raw ↦ Y_s :
        Centered := Raw - Mean
        Norm := N(Centered)
        Proj := Norm <~> SvdVec
        Y_s := silu(Proj)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |Y_s| < 1e4, latency < 10ns }
