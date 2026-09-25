// CRON Neocortical Micro-Circuit & Biological Neuromodulation Engine
// Module: cron.neuro.micro_circuit
// Brain 4: 6-Layer Mammalian Cortical Column with Homeostatic E/I Balance

.MODULE cron.neuro.micro_circuit

struct NeuromodulatorCocktail {
    dopamine: i32,
    acetylcholine: i32,
    serotonin: i32,
    noradrenaline: i32
}

struct CorticalLayerState {
    layer_id: i32,
    excitatory_count: i32,
    inhibitory_count: i32,
    membrane_tau_us: i32,
    spike_threshold_mv: i32,
    resting_potential_mv: i32,
    current_voltage_mv: i32,
    e_i_ratio_scaled: i32
}

struct CorticalColumn {
    column_id: i32,
    core_id: i32,
    torus_x: i32,
    torus_y: i32,
    torus_z: i32,
    torus_w: i32,
    neuromodulators: NeuromodulatorCocktail,
    layer_l23: CorticalLayerState,
    layer_l4: CorticalLayerState,
    layer_l5: CorticalLayerState,
    layer_l6: CorticalLayerState,
    is_active: bool
}

def create_default_neuromodulators() -> NeuromodulatorCocktail {
    return NeuromodulatorCocktail {
        dopamine: 500,
        acetylcholine: 700,
        serotonin: 600,
        noradrenaline: 300
    }
}

def create_layer_spec(layer_id: i32, exc: i32, inh: i32, tau_us: i32, threshold_mv: i32) -> CorticalLayerState {
    let ratio: i32 = (exc * 1000) / (if inh > 0 { inh } else { 1 })
    return CorticalLayerState {
        layer_id: layer_id,
        excitatory_count: exc,
        inhibitory_count: inh,
        membrane_tau_us: tau_us,
        spike_threshold_mv: threshold_mv,
        resting_potential_mv: -70,
        current_voltage_mv: -70,
        e_i_ratio_scaled: ratio
    }
}

def initialize_cortical_column(column_id: i32, core_id: i32, tx: i32, ty: i32, tz: i32, tw: i32) -> CorticalColumn {
    let cocktail: NeuromodulatorCocktail = create_default_neuromodulators()
    let l23: CorticalLayerState = create_layer_spec(2, 80, 20, 15000, -50)
    let l4: CorticalLayerState = create_layer_spec(4, 90, 10, 10000, -52)
    let l5: CorticalLayerState = create_layer_spec(5, 85, 15, 25000, -48)
    let l6: CorticalLayerState = create_layer_spec(6, 75, 25, 30000, -55)

    return CorticalColumn {
        column_id: column_id,
        core_id: core_id,
        torus_x: tx,
        torus_y: ty,
        torus_z: tz,
        torus_w: tw,
        neuromodulators: cocktail,
        layer_l23: l23,
        layer_l4: l4,
        layer_l5: l5,
        layer_l6: l6,
        is_active: true
    }
}

def inject_sensory_spike_to_layer4(column: CorticalColumn, spike_magnitude_mv: i32) -> CorticalColumn {
    let new_volt: i32 = column.layer_l4.current_voltage_mv + spike_magnitude_mv
    let fired: bool = new_volt >= column.layer_l4.spike_threshold_mv
    let updated_l4_volt: i32 = if fired { column.layer_l4.resting_potential_mv } else { new_volt }
    let updated_l23_volt: i32 = if fired {
        column.layer_l23.current_voltage_mv + (spike_magnitude_mv * 8 / 10)
    } else {
        column.layer_l23.current_voltage_mv
    }

    let updated_l4: CorticalLayerState = CorticalLayerState {
        layer_id: column.layer_l4.layer_id,
        excitatory_count: column.layer_l4.excitatory_count,
        inhibitory_count: column.layer_l4.inhibitory_count,
        membrane_tau_us: column.layer_l4.membrane_tau_us,
        spike_threshold_mv: column.layer_l4.spike_threshold_mv,
        resting_potential_mv: column.layer_l4.resting_potential_mv,
        current_voltage_mv: updated_l4_volt,
        e_i_ratio_scaled: column.layer_l4.e_i_ratio_scaled
    }

    let updated_l23: CorticalLayerState = CorticalLayerState {
        layer_id: column.layer_l23.layer_id,
        excitatory_count: column.layer_l23.excitatory_count,
        inhibitory_count: column.layer_l23.inhibitory_count,
        membrane_tau_us: column.layer_l23.membrane_tau_us,
        spike_threshold_mv: column.layer_l23.spike_threshold_mv,
        resting_potential_mv: column.layer_l23.resting_potential_mv,
        current_voltage_mv: updated_l23_volt,
        e_i_ratio_scaled: column.layer_l23.e_i_ratio_scaled
    }

    return CorticalColumn {
        column_id: column.column_id,
        core_id: column.core_id,
        torus_x: column.torus_x,
        torus_y: column.torus_y,
        torus_z: column.torus_z,
        torus_w: column.torus_w,
        neuromodulators: column.neuromodulators,
        layer_l23: updated_l23,
        layer_l4: updated_l4,
        layer_l5: column.layer_l5,
        layer_l6: column.layer_l6,
        is_active: column.is_active
    }
}

def modulate_dopamine_level(column: CorticalColumn, delta_dopamine: i32) -> CorticalColumn {
    let raw_da: i32 = column.neuromodulators.dopamine + delta_dopamine
    let new_da: i32 = if raw_da > 1000 { 1000 } else if raw_da < 0 { 0 } else { raw_da }
    let updated_cocktail: NeuromodulatorCocktail = NeuromodulatorCocktail {
        dopamine: new_da,
        acetylcholine: column.neuromodulators.acetylcholine,
        serotonin: column.neuromodulators.serotonin,
        noradrenaline: column.neuromodulators.noradrenaline
    }

    return CorticalColumn {
        column_id: column.column_id,
        core_id: column.core_id,
        torus_x: column.torus_x,
        torus_y: column.torus_y,
        torus_z: column.torus_z,
        torus_w: column.torus_w,
        neuromodulators: updated_cocktail,
        layer_l23: column.layer_l23,
        layer_l4: column.layer_l4,
        layer_l5: column.layer_l5,
        layer_l6: column.layer_l6,
        is_active: column.is_active
    }
}
