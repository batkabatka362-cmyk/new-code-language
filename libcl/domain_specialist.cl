@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:       DomainLatents[16×16:FP16]    @sram[0x0000]
    guidance: AutoGuidanceField[16×16:FP16] @sram[0x0200]
    out:      SynthesizedOut[16×16:FP16]   @sram[0x0400]
}

§DomainSpecialistBlock[16×16:FP16] {
    BezierSplines: T[16×16:FP16], RectifiedFlow: T[16×16:FP16]

    DomainLatents ↦ SynthesizedOut :
        GuidedFlow := DomainLatents <~> AutoGuidanceField
        Norm := rmsnorm(GuidedFlow)
        Curves := Norm * BezierSplines
        SynthesizedOut := silu(Curves)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |SynthesizedOut| < 1e4, latency < 14ns }
