@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:       GpuTokens[16×16:NVFP4]    @sram[0x0000]
    streamed: CpuAmxWeight[16×16:NVFP4] @sram[0x0200]
    out:      FusedHidden[16×16:NVFP4]  @sram[0x0400]
}

§HeterogeneousOffloadBlock[16×16:NVFP4] {
    PciePipeline: T[16×16:NVFP4], BlockSparseKv: T[16×16:NVFP4]

    GpuTokens ↦ FusedHidden :
        Overlapped := GpuTokens <~> CpuAmxWeight
        Norm := rmsnorm(Overlapped)
        ExpertVal := Norm * BlockSparseKv
        FusedHidden := silu(ExpertVal)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |FusedHidden| < 1e4, latency < 12ns }
