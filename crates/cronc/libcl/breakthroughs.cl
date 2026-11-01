@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:     X[16×16:NVFP4]              @sram[0x0000]
    spline: BSplineWeights[16×16:NVFP4] @sram[0x0200]
    out:    Y[16×16:NVFP4]              @sram[0x0400]
}

§BreakthroughsBlock[16×16:NVFP4] {
    CliffordRotor: T[16×16:NVFP4], SsdState: T[16×16:NVFP4]

    X ↦ Y :
        KanSpline := X <~> BSplineWeights
        B_pop := KanSpline * BSplineWeights
        Norm := N(B_pop)
        Y := silu(Norm)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |Y| < 1e4, latency < 10ns }
