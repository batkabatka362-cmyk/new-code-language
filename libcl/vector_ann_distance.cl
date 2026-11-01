@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    query:   Q[16×16:FP32]              @sram[0x0000]
    corpus:  K[16×16:FP32]              @sram[0x0200]
    metric:  Distances[16×16:FP32]      @sram[0x0400]
}

§VectorMetricBlock[16×16:FP32] {
    S: T[16×16:FP32]

    Q ↦ Distances :
        DotProd := Q ⊗ K
        NormQ := N(Q)
        NormK := N(K)
        CosineDist := DotProd * NormQ
        Distances := silu(CosineDist)
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

!Δ { |Distances| < 1e4, recall_at_10 > 0.99 }
