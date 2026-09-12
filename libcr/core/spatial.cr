; libcr/core/spatial.cr — 4D Torus Spatial Communication Library
; 256-Core (4x4x4x4) Torus NoC Packet Routing, Spherical Waves, & Simplex Pipes

module Core_Spatial

; 4D Hyper-cube Spherical Wave Broadcast
export def inline broadcast_4d_sphere(lin payload : linear vec4_i8) {
    ; Broadcast to all 8 nearest neighbors (+-X, +-Y, +-Z, +-W) in 1 clock cycle
    spatial_broadcast(axis=HYPER_ALL, payload=consume(payload))
}

; Open a zero-latency simplex hardware pipe between two cores
export def inline open_simplex_pipe(target_axis: u32, channel_id: u32) -> (u32, u32) {
    let tx_handle = open_channel(to=target_axis, vc=channel_id)
    let rx_handle = open_channel(from=target_axis, vc=channel_id)
    return (tx_handle, rx_handle)
}

; Synchronize arrival of waves across neighborhood axes
export def inline sync_neighborhood(axis_mask: u32) {
    spatial_sync(axis=axis_mask)
}
