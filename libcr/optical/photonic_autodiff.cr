// CRON Standard Library - Forward-Mode Optical Automatic Differentiation
// Module: cron.optical.photonic_autodiff
// Brain 2: Instantaneous Optical Autodiff Tap with 0 DRAM Memory Footprint

.MODULE cron.optical.photonic_autodiff

struct OpticalAutodiffTap {
    tap_ratio_percent: u8, // 1% light extraction for gradient accumulation
    polarization_angle: u16,
    spectral_bandwidth: u32
}

def init_autodiff_tap(tap_ratio: u8) -> OpticalAutodiffTap {
    return OpticalAutodiffTap {
        tap_ratio_percent: tap_ratio,
        polarization_angle: 0,
        spectral_bandwidth: 100
    }
}

// Forward Autodiff Tap: Splits optical wavefront into primary output and gradient tangent
// Maps directly to machine VLIW opcode _FA
def tap_forward_gradient(lin wave: linear wave_t, tap: OpticalAutodiffTap) -> (linear wave_t, linear wave_t) {
    let lin primal_wave = optical_gemm(wave=consume(wave))
    let lin tangent_wave = optical_autodiff_tap(primal_wave)
    return (primal_wave, tangent_wave)
}
