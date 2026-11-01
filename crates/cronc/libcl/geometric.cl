@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:  V[16×16:FP32]    @sram[0x0000]
    rot: R[16×16:FP32]    @sram[0x0200]
    out: Vrot[16×16:FP32] @sram[0x0400]
}

§CliffordGeometricBlock[16×16:FP32] {
    Psi: T[16×16:FP32], Rotor: T[16×16:FP32]

    V ↦ Vrot :
        Half := R ^ V
        Full := Half ^ ~R
        Norm := N(Full)
        Vrot := silu(Norm)
}

§ControlFlow {
    ⟳ (64) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |Vrot| < 5e3, latency < 8ns }
