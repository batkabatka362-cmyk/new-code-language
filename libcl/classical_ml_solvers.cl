@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    features:   X[16×16:FP32]           @sram[0x0000]
    weights:    W[16×16:FP32]           @sram[0x0200]
    residuals:  Grad[16×16:FP32]        @sram[0x0400]
}

§ConvexSolverBlock[16×16:FP32] {
    LambdaL1: T[16×16:FP32], AlphaL2: T[16×16:FP32]

    X ↦ Grad :
        Pred := X ⊗ W
        Residual := Pred - X
        GradL2 := Residual + (AlphaL2 * W)
        Shrunk := relu(GradL2 - LambdaL1)
        Grad := silu(Shrunk)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        #R4 := #R3 - #R1
        @sram[0x0400] ← #R4
    }
}

!Δ { |Grad| < 1e4, duality_gap < 1e-5 }
