@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    bytes_in:   RawBytes[16×16:FP32]        @sram[0x0000]
    vocab_trie: MergeTrie[16×16:FP32]       @sram[0x0200]
    tokens_out: TokenIds[16×16:FP32]        @sram[0x0400]
}

§BytePairMergeBlock[16×16:FP32] {
    VocabMask: T[16×16:FP32], FreqWeight: T[16×16:FP32]

    RawBytes ↦ TokenIds :
        MergedPairs := RawBytes ⊗ MergeTrie
        ScoredTokens := MergedPairs * FreqWeight
        Norm_Tok := N(ScoredTokens)
        TokenIds := silu(Norm_Tok)
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

!Δ { |TokenIds| < 1e5, compression_ratio > 1.5 }
