@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:  X[16×16:BF16]       @sram[0x0000]
    dt:  DeltaT[16×16:BF16]  @sram[0x0200]
    out: Y[16×16:BF16]       @sram[0x0400]
}

§RecurrentBlock[16×16:BF16] {
    H: SS[64×256], W: T[16×16:BF16]

    X ↦ Y :
        S := ssd(X >>= DeltaT, H)
        N := N(S)
        Y := silu((N <~> S) ~*~ S)
}

§ControlFlow {
    ⟳ (128) {
        #R1 ← @sram[0x0000]
        ? (#R1 > 0x2000) ⮞ {
            #R2 := #R1 * 0.95
            @sram[0x0400] ← #R2
        } : {
            @sram[0x0400] ← #R1
        }
    }
}
!Δ { |Y| < 1e4, latency < 12ns }
