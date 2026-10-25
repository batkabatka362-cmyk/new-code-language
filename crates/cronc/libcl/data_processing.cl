@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:    Raw[16×16:FP32]        @sram[0x0000]
    scale: ScaleParam[16×16:FP32] @sram[0x0200]
    out:   Scaled[16×16:FP32]     @sram[0x0400]
}

§DataProcessingBlock[16×16:FP32] {
    Mean: T[16×16:FP32], SvdVec: T[16×16:FP32]

    Raw ↦ Scaled :
        Centered := Raw - Mean
        Norm := rmsnorm(Centered)
        Proj := Norm <~> SvdVec
        Scaled := silu(Proj)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |Scaled| < 1e4, latency < 10ns }
