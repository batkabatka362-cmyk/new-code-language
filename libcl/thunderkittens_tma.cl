@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:     TileA[16×16:FP32]   @sram[0x0000]
    tile_b: TileB[16×16:FP32]   @sram[0x0200]
    out:    TileAccum[16×16:FP32] @sram[0x0400]
}

§ThunderKittensTmaBlock[16×16:FP32] {
    TmaScale: T[16×16:FP32], SwizzleMask: T[16×16:FP32]

    TileA ↦ TileAccum :
        AsyncTileFused := TileA <~> TileB
        SaturatedCore := AsyncTileFused * TmaScale
        NormedAccum := rmsnorm(SaturatedCore)
        TileAccum := silu(NormedAccum)
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
!Δ { |TileAccum| < 1e4, latency < 10ns }
