@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:     DataMatrix[16×16:FP32]      @sram[0x0000]
    proj:   EigenVectors[16×16:FP32]    @sram[0x0200]
    out:    ReducedEmbedding[16×16:FP32] @sram[0x0400]
}

§PcaSvdBlock[16×16:FP32] {
    Covariance: T[16×16:FP32], UMAPWeights: T[16×16:FP32]

    DataMatrix ↦ ReducedEmbedding :
        Covariance := DataMatrix ⊗ DataMatrix
        EigenVectors := svd(Covariance)
        Projected := DataMatrix ⊗ EigenVectors
        ReducedEmbedding := silu(Projected)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        #R4 := #R3 + #R2
        @sram[0x0400] ← #R4
    }
}

!Δ { |ReducedEmbedding| < 1e5, energy_preservation > 0.95 }
