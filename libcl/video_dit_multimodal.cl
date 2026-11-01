@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    in:       VideoPixels[16×16:FP16]   @sram[0x0000]
    velocity: FlowField[16×16:FP16]     @sram[0x0200]
    out:      LatentFrames[16×16:FP16]  @sram[0x0400]
}

§VideoDitBlock[16×16:FP16] {
    Causal3DVae: T[16×16:FP16], Tile_Ψ: T[16×16:FP16]

    VideoPixels ↦ LatentFrames :
        CausalEncoded := VideoPixels <~> Causal3DVae
        Norm := N(CausalEncoded)
        FlowStep := Norm * FlowField
        LatentFrames := silu(FlowStep)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        @sram[0x0400] ← #R3
    }
}
!Δ { |LatentFrames| < 1e4, latency < 15ns }
