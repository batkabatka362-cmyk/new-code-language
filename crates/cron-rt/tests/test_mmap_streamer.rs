// ============================================================================
// CRON Zero-VRAM Paged Weight Streamer Verification Test
// Tests bounded resident memory and zero-leak layer eviction across 32 layers.
// ============================================================================

use cron_rt::mmap_streamer::PagedWeightStreamer;

#[test]
fn test_bounded_layer_streaming_and_eviction() {
    let num_layers = 32;
    let layer_size_bytes = 2 * 1024 * 1024; // 2 MB per layer (64 MB total)
    let page_capacity_bytes = 4 * 1024 * 1024; // 4 MB bounded scratchpad limit

    let streamer = PagedWeightStreamer::new_synthetic(num_layers, layer_size_bytes, page_capacity_bytes);

    // Fixed-size reusable regional buffer in SRAM/RAM
    let mut scratch_buffer = vec![0u8; page_capacity_bytes];

    for l in 0..num_layers {
        // 1. Stream layer weights into scratchpad
        let bytes_read = streamer
            .stream_layer(l, &mut scratch_buffer)
            .expect("Streaming layer failed");

        assert_eq!(bytes_read, layer_size_bytes);

        // Verify deterministic pattern loaded
        let expected_byte = ((l * 17) & 0xFF) as u8;
        assert_eq!(scratch_buffer[0], expected_byte);
        assert_eq!(scratch_buffer[bytes_read - 1], expected_byte);

        // 2. Immediate 0-cycle buffer eviction
        streamer.evict_layer();
        assert_eq!(
            streamer.telemetry().active_memory_bytes,
            0,
            "Active memory must be 0 after eviction"
        );
    }

    let telemetry = streamer.telemetry();
    assert_eq!(
        telemetry.total_bytes_read,
        (num_layers * layer_size_bytes) as u64
    );
    assert_eq!(telemetry.layers_streamed, num_layers);
    assert_eq!(telemetry.evictions_count, num_layers);
    assert!(
        telemetry.peak_memory_bytes <= page_capacity_bytes,
        "Peak active memory ({}) must never exceed page capacity limit ({})",
        telemetry.peak_memory_bytes,
        page_capacity_bytes
    );

    println!(
        "Paged Streamer verified: Streamed {} MB across {} layers with peak memory strictly <= {} MB",
        telemetry.total_bytes_read / (1024 * 1024),
        num_layers,
        telemetry.peak_memory_bytes / (1024 * 1024)
    );
}
