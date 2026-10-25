@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:     Tokens[16×16:NVFP4]       @sram[0x0000]
    expert: ExpertRouter[16×16:NVFP4] @sram[0x0200]
    out:    Hidden[16×16:NVFP4]       @sram[0x0400]
}

§FrontierModelsBlock[16×16:NVFP4] {
    W: T[16×16:NVFP4], GrpoAdvantage: T[16×16:NVFP4]

    Tokens ↦ Hidden :
        Latent := Tokens <~> ExpertRouter
        Norm := rmsnorm(Latent)
        Routed := .op.nvfp4_gemm(Norm, W)
        Hidden := silu(Routed)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |Hidden| < 1e4, latency < 10ns }
