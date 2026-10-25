@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:  X[16×16:NVFP4]  @sram[0x0000]
    wt:  W[16×16:NVFP4]  @sram[0x0200]
    out: Y[16×16:NVFP4]  @sram[0x0400]
}

§SiliconTensorBlock[16×16:NVFP4] {
    H: T[16×16:NVFP4], Acc: T[16×16:FP32]

    X ↦ Y :
        R := X <~> H
        Q := .op.nvfp4_gemm(R, W)
        N := rmsnorm(Q)
        Y := silu(N)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |Y| < 1e4, latency < 10ns }
