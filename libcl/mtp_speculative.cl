@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:       BaseHidden[16×16:NVFP4]     @sram[0x0000]
    drafts:   MtpLookahead[16×16:NVFP4]   @sram[0x0200]
    verified: AcceptedTokens[16×16:NVFP4] @sram[0x0400]
}

§MtpSpeculativeBlock[16×16:NVFP4] {
    DraftVerifier: T[16×16:NVFP4], SpecScore: T[16×16:NVFP4]

    BaseHidden ↦ AcceptedTokens :
        FuturePred := BaseHidden <~> MtpLookahead
        Norm := rmsnorm(FuturePred)
        Scored := Norm * SpecScore
        AcceptedTokens := silu(Scored)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |AcceptedTokens| < 1e4, latency < 8ns }
