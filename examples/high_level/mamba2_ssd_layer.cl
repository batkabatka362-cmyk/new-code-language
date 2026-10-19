// ============================================================================
// CRON HIGH-LEVEL CL ARCHITECTURE SPECIFICATION
// Example: Mamba-2 Structured State Space (SSD) Layer with Photonic MZI & Wave
// ============================================================================

module mamba2_ssd_layer;

.stage "SiliconTensorTile", tile_x = 16, tile_y = 16, memory = "sram";

arch Mamba2Block {
    state ssm_hidden: StateSpace[64, 256];
    state weights: RegTile[16, 16, NVFP4];

    fn step(x: Tensor[NVFP4, 16, 16]) -> Tensor[NVFP4, 16, 16] {
        let norm = .op.rmsnorm(x);
        let scan = .op.mamba2_ssd(norm);
        let coupled = norm <~> scan;
        let wave = coupled ~*~ scan;
        let gated = .op.silu(wave);
        return gated;
    }
}
