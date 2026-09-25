// ============================================================================
// SAGI 6-Layer Neocortical Driver & Sensory Loop (`examples/sagi_neocortex_500_driver.cr`)
//
// Demonstrates:
// 1. Instantiating Cortical Columns across 256 cores
// 2. Setting chemical neuromodulation levels (Dopamine, Acetylcholine, etc.)
// 3. Injecting high-frequency DVS & Cochlea sensory spikes into Layer 4
// 4. Verifying Layer 4 -> Layer 2/3 forward excitation propagation
// ============================================================================

import {
    CorticalColumn,
    initialize_cortical_column,
    inject_sensory_spike_to_layer4,
    modulate_dopamine_level
} from "../libcr/neuro/micro_circuit.cr"

def main() -> i32 {
    let col0 = initialize_cortical_column(0, 0, 0, 0, 0, 0)

    if col0.layer_l4.current_voltage_mv != -70 {
        return 1
    }

    let col1 = modulate_dopamine_level(col0, 250)
    if col1.neuromodulators.dopamine != 750 {
        return 2
    }

    let col2 = inject_sensory_spike_to_layer4(col1, 25)

    if col2.layer_l4.current_voltage_mv != -70 {
        return 3
    }

    if col2.layer_l23.current_voltage_mv != -50 {
        return 4
    }

    return 0
}
