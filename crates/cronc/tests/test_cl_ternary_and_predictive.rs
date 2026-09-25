use cronc::cl_ternary_simd::*;
use cronc::cl_predictive_coding::*;

#[test]
fn test_trit_packing_and_unpacking() {
    let mut trits = [0i8; 64];
    trits[0] = 1;
    trits[1] = -1;
    trits[2] = 0;
    trits[31] = 1;
    trits[32] = -1;
    trits[63] = 1;

    let packed = TritWord128::pack(&trits);
    let unpacked = packed.unpack();

    assert_eq!(unpacked[0], 1);
    assert_eq!(unpacked[1], -1);
    assert_eq!(unpacked[2], 0);
    assert_eq!(unpacked[31], 1);
    assert_eq!(unpacked[32], -1);
    assert_eq!(unpacked[63], 1);

    assert_eq!(packed.get_trit(0), TernaryTrit::Plus);
    assert_eq!(packed.get_trit(1), TernaryTrit::Minus);
    assert_eq!(packed.get_trit(2), TernaryTrit::Zero);
    assert_eq!(packed.non_zero_count(), 5);
}

#[test]
fn test_trit_simd_zero_mul_dot_product() {
    let mut trits = [0i8; 64];
    let mut acts = [0i16; 64];

    // +1 * 100 + (-1) * 40 + (+1) * 25 = 100 - 40 + 25 = 85
    trits[0] = 1;
    acts[0] = 100;
    trits[1] = -1;
    acts[1] = 40;
    trits[35] = 1;
    acts[35] = 25;

    let packed = TritWord128::pack(&trits);
    let result = TritSimdEngine::dot_product_i16(&packed, &acts);
    assert_eq!(result, 85);

    let (fp16_e, trit_e, red) = TritSimdEngine::energy_metric(1000, 0.7);
    assert!(red > 10.0, "Trit SIMD must achieve >10x energy reduction vs FP16 MAC");
    assert!(trit_e < fp16_e);
}

#[test]
fn test_predictive_coding_local_error_and_learning() {
    let mut net = HierarchicalPredictiveNetwork::new(&[4, 2], 0.05);

    let sensory_input = vec![1.0, 0.5, -0.5, 0.0];
    let initial_fe = net.step_cycle(&sensory_input);
    assert!(initial_fe >= 0.0);

    // After multiple local Hebbian error updates, free energy should minimize
    let mut last_fe = initial_fe;
    for _ in 0..50 {
        last_fe = net.step_cycle(&sensory_input);
    }

    assert!(
        last_fe < initial_fe,
        "Local dendritic error updates must decrease free-energy: initial={}, final={}",
        initial_fe,
        last_fe
    );
}

#[test]
fn test_dendritic_neuron_error_computation() {
    let mut neuron = DendriticNeuron::new(0, 0.2);
    neuron.basal_input = 0.8;
    neuron.apical_prediction = 0.3;

    let err = neuron.compute_local_error();
    assert!((err - 0.5).abs() < 1e-6);
    assert!(neuron.fired);
}
