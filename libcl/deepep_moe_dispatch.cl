@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:     LocalTokensFP8[16×16:FP32]    @sram[0x0000]
    expert: ExpertGateMatrix[16×16:FP32]  @sram[0x0200]
    out:    DispatchedMoE[16×16:FP32]     @sram[0x0400]
}

§DeepEpMoeDispatchBlock[16×16:FP32] {
    AsymInterconnect: T[16×16:FP32], AccumWeights: T[16×16:FP32]

    LocalTokensFP8 ↦ DispatchedMoE :
        RoutedGated := LocalTokensFP8 <~> ExpertGateMatrix
        ForwardedStream := RoutedGated * AsymInterconnect
        NormedAccum := rmsnorm(ForwardedStream)
        DispatchedMoE := silu(NormedAccum)
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
!Δ { |DispatchedMoE| < 1e4, latency < 10ns }
