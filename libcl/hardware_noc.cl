@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:   Flit[16×16:FP32]   @sram[0x0000]
    dest: Dst[16×16:FP32]    @sram[0x0200]
    out:  Routed[16×16:FP32] @sram[0x0400]
}

§TorusNoCBlock[16×16:FP32] {
    Phase: T[16×16:FP32], Map: T[16×16:FP32]

    Flit ↦ Routed :
        Route := Flit ^ Dst
        Mzi := Route <~> Phase
        Norm := N(Mzi)
        Routed := silu(Norm)
}

§ControlFlow {
    ⟳ (512) {
        #R1 ← @sram[0x0000]
        ? (#R1 > 0x1000) ⮞ {
            #R2 := #R1 * 0.99
            @sram[0x0400] ← #R2
        } : {
            @sram[0x0400] ← #R1
        }
    }
}
!Δ { |Routed| < 1e4, latency < 5ns }
