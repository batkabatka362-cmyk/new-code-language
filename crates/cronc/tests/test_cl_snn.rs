use cronc::{
    simulate_snn, synthesize_snn_kernel, verify_cl_program, run_cl_jit,
    LifNeuronConfig, StdpConfig, SpikeEvent,
};

#[test]
fn test_lif_neuron_integration_and_firing() {
    let lif_cfg = LifNeuronConfig {
        decay_beta: 0.90,
        threshold: 1.0,
        reset_voltage: 0.0,
        refractory_cycles: 2,
    };
    let stdp_cfg = StdpConfig::default();

    // Two consecutive inputs to neuron 0 at cycle 0 and 1
    let input_events = vec![
        SpikeEvent { time_cycle: 0, neuron_id: 0 },
        SpikeEvent { time_cycle: 1, neuron_id: 0 },
    ];

    let result = simulate_snn(4, 10, &lif_cfg, &stdp_cfg, &input_events);

    assert_eq!(result.num_neurons, 4);
    assert_eq!(result.total_cycles, 10);
    assert!(result.total_spikes >= 1, "Neuron 0 should have fired a spike");
    assert!(result.spike_matrix[1][0] || result.spike_matrix[2][0], "Neuron 0 should fire at cycle 1 or 2");
    assert!(result.energy_picojoules > 0.0);
}

#[test]
fn test_stdp_weight_adaptation() {
    let lif_cfg = LifNeuronConfig {
        decay_beta: 0.80,
        threshold: 0.7,
        reset_voltage: 0.0,
        refractory_cycles: 1,
    };
    let stdp_cfg = StdpConfig {
        a_plus: 0.20,
        a_minus: 0.20,
        tau_plus: 10.0,
        tau_minus: 10.0,
        w_min: -2.0,
        w_max: 2.0,
        quantize_int2: true,
    };

    // Pre-spike on neuron 0 at t=0, Post-spike on neuron 1 at t=2
    let input_events = vec![
        SpikeEvent { time_cycle: 0, neuron_id: 0 },
        SpikeEvent { time_cycle: 2, neuron_id: 1 },
    ];

    let result = simulate_snn(2, 6, &lif_cfg, &stdp_cfg, &input_events);

    // Weights should be within the configured bounds
    for &w in &result.synaptic_weights {
        assert!(w >= stdp_cfg.w_min && w <= stdp_cfg.w_max);
    }
}

#[test]
fn test_snn_microcode_synthesis_and_jit() {
    let lif_cfg = LifNeuronConfig::default();
    let stdp_cfg = StdpConfig::default();

    let cl_code = synthesize_snn_kernel(8, &lif_cfg, &stdp_cfg);
    assert!(cl_code.contains("CRON BRAIN 4 NEUROMORPHIC SNN"));

    // Verify .cl format
    let cl_rep = verify_cl_program(&cl_code);
    assert!(cl_rep.is_ok(), "Synthesized SNN microcode must be valid .cl: {:?}", cl_rep.err());

    // Execute via JIT
    let jit_res = run_cl_jit(&cl_code);
    assert!(jit_res.is_ok(), "JIT execution failed: {:?}", jit_res.err());
}

#[test]
fn test_snn_ascii_raster_and_json() {
    let lif_cfg = LifNeuronConfig::default();
    let stdp_cfg = StdpConfig::default();
    let input_events = vec![
        SpikeEvent { time_cycle: 1, neuron_id: 0 },
        SpikeEvent { time_cycle: 2, neuron_id: 0 },
        SpikeEvent { time_cycle: 3, neuron_id: 1 },
    ];

    let result = simulate_snn(4, 8, &lif_cfg, &stdp_cfg, &input_events);

    // Test ASCII raster
    let ascii = result.render_ascii_raster();
    assert!(ascii.contains("CRON BRAIN 4 NEUROMORPHIC SPIKE RASTER"));
    assert!(ascii.contains("Terminal Spike Raster Plot"));
    assert!(ascii.contains("N00 │"));

    // Test JSON serialization
    let json = result.to_json();
    assert!(json.contains("\"num_neurons\": 4"));
    assert!(json.contains("\"total_cycles\": 8"));
    assert!(json.contains("\"final_membrane_potentials\":"));
    assert!(json.contains("\"sample_synaptic_weights\":"));
}
