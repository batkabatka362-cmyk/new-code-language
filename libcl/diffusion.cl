@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:  Xt[16×16:FP32]    @sram[0x0000]
    vel: Vpred[16×16:FP32] @sram[0x0200]
    out: Xnext[16×16:FP32] @sram[0x0400]
}

§FlowMatchingBlock[16×16:FP32] {
    W: T[16×16:FP32], Noise: T[16×16:FP32]

    Xt ↦ Xnext :
        Step := .op.ode_dormand_prince(Vpred, dt=0.05)
        Mod := N(Xt + Step)
        Xnext := Mod ~*~ Noise
}

§ControlFlow {
    ⟳ (50) {
        #R1 ← @sram[0x0000]
        ? (#R1 > 0x3000) ⮞ {
            #R2 := #R1 * 0.98
            @sram[0x0400] ← #R2
        } : {
            ⮌
        }
    }
}
!Δ { |Xnext| < 1e4, latency < 15ns }
