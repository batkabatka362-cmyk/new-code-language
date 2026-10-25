@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    signal:     SignalIn[16×16:FP32]        @sram[0x0000]
    filter:     TwiddleFactors[16×16:FP32]  @sram[0x0200]
    spectrum:   FilteredSpectrum[16×16:FP32] @sram[0x0400]
}

§FourierWaveletBlock[16×16:FP32] {
    AlphaEMA: T[16×16:FP32], WaveletScale: T[16×16:FP32]

    SignalIn ↦ FilteredSpectrum :
        EmaSmoothed := SignalIn * AlphaEMA
        FreqDomain := EmaSmoothed ⊗ TwiddleFactors
        NormalizedFreq := rmsnorm(FreqDomain)
        FilteredSpectrum := silu(NormalizedFreq)
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        #R4 := #R3 + #R2
        @sram[0x0400] ← #R4
    }
}

!Δ { |FilteredSpectrum| < 1e5, snr_db > 40.0 }
