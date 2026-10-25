@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:  X[16×16:FP32]    @sram[0x0000]
    wt:  W[16×16:FP32]    @sram[0x0200]
    out: Pred[16×16:FP32] @sram[0x0400]
}

§ClassicalMLBlock[16×16:FP32] {
    Bias: T[16×16:FP32], TargetY: T[16×16:FP32]

    X ↦ Pred :
        Grad := X <~> W
        Hess := Grad * Grad
        Residual := TargetY - Grad
        Norm := rmsnorm(Residual)
        Pred := silu(Norm)
}

§ControlFlow {
    ⟳ (100) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |Pred| < 1e4, latency < 8ns }
