@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:   Nodes[16×16:FP32]  @sram[0x0000]
    mask: DoMask[16×16:FP32] @sram[0x0200]
    out:  Effect[16×16:FP32] @sram[0x0400]
}

§CausalDoBlock[16×16:FP32] {
    Adj: T[16×16:FP32], W: T[16×16:FP32]

    Nodes ↦ Effect :
        Intervened := (Nodes * DoMask) + W
        Mess := Intervened <~> Adj
        Norm := N(Mess)
        Effect := silu(Norm)
}

§ControlFlow {
    ⟳ (128) {
        #R1 ← @sram[0x0000]
        ? (#R1 > 0x1000) ⮞ {
            #R2 := #R1 * 0.95
            @sram[0x0400] ← #R2
        } : {
            @sram[0x0400] ← #R1
        }
    }
}
!Δ { |Effect| < 1e4, latency < 12ns }
