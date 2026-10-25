@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:   Query[16×16:FP32]      @sram[0x0000]
    cand: Candidates[16×16:FP32] @sram[0x0200]
    out:  SimScores[16×16:FP32]  @sram[0x0400]
}

§VectorANNBlock[16×16:FP32] {
    HnswGraph: T[16×16:FP32], QuantCode: T[16×16:FP32]

    Query ↦ SimScores :
        NormQ := rmsnorm(Query)
        Dot := NormQ <~> Candidates
        Score := Dot * QuantCode
        SimScores := silu(Score)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |SimScores| < 1e4, latency < 6ns }
