// ============================================================================
// CRON Standard Library - Neuromorphic SNN & Surrogate Gradient Autograd
// Module: cron.neuro.snn_autograd
// Target: Brain 4 (Neuromorphic SNN) & Hardware Autograd Accelerators
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

.MODULE cron.neuro.snn_autograd

// Leaky Integrate-and-Fire (LIF) Neuron parameters and dynamic state
struct LIFNeuron {
    u_membrane: f32,
    u_threshold: f32,
    v_rest: f32,
    v_reset: f32,
    decay_beta: f32,
    spike_count: i32
}

// Synaptic Layer connecting inputs to an LIF neuron
struct SNNLayer {
    weights: vec8f,
    bias: f32,
    neuron: LIFNeuron,
    alpha_surrogate: f32
}

// Tape entry recording forward state for exact surrogate backpropagation
struct SNNTapeEntry {
    u_before_spike: f32,
    spike_emitted: f32,
    input_current: f32
}

// Step result for LIF neuron
struct LIFStepResult {
    neuron: LIFNeuron,
    spike: f32
}

// Forward result for SNN layer
struct SNNLayerOutput {
    layer: SNNLayer,
    spike: f32,
    tape: SNNTapeEntry
}

// Initialize LIF Neuron with standard biological parameters
def create_lif_neuron(threshold: f32, decay_beta: f32) -> LIFNeuron {
    return LIFNeuron {
        u_membrane: 0.0,
        u_threshold: threshold,
        v_rest: 0.0,
        v_reset: 0.0,
        decay_beta: decay_beta,
        spike_count: 0
    }
}

// FastSigmoid Surrogate Derivative: dS/dU = 1 / (1 + alpha * |U - U_th|)^2
def fast_sigmoid_surrogate_grad(u: f32, u_th: f32, alpha: f32) -> f32 {
    let diff: f32 = u - u_th
    let abs_diff: f32 = if diff < 0.0 { -diff } else { diff }
    let denom: f32 = 1.0 + (alpha * abs_diff)
    return 1.0 / (denom * denom)
}

// ArcTangent Surrogate Derivative: dS/dU = 1 / (1 + (alpha * (U - U_th))^2)
def atan_surrogate_grad(u: f32, u_th: f32, alpha: f32) -> f32 {
    let diff: f32 = u - u_th
    let scaled: f32 = alpha * diff
    let denom: f32 = 1.0 + (scaled * scaled)
    return 1.0 / denom
}

// Forward simulation step for a single LIF neuron
def lif_step_forward(neuron: LIFNeuron, input_current: f32) -> LIFStepResult {
    // Leaky integration: U_new = beta * U_old + I
    let u_decayed: f32 = neuron.u_membrane * neuron.decay_beta
    let u_next: f32 = u_decayed + input_current

    // Threshold spike evaluation: S = 1.0 if U >= U_th else 0.0
    let mut spike: f32 = 0.0
    let mut u_after: f32 = u_next
    let mut count: i32 = neuron.spike_count

    if u_next >= neuron.u_threshold {
        spike = 1.0
        u_after = neuron.v_reset
        count = count + 1
    }

    let updated_neuron = LIFNeuron {
        u_membrane: u_after,
        u_threshold: neuron.u_threshold,
        v_rest: neuron.v_rest,
        v_reset: neuron.v_reset,
        decay_beta: neuron.decay_beta,
        spike_count: count
    }

    return LIFStepResult {
        neuron: updated_neuron,
        spike: spike
    }
}

// Backward surrogate gradient step: computes dLoss / dInput
def lif_step_backward(grad_output: f32, u_before_spike: f32, u_th: f32, alpha: f32) -> f32 {
    let surrogate_deriv = fast_sigmoid_surrogate_grad(u_before_spike, u_th, alpha)
    return grad_output * surrogate_deriv
}

// Forward pass through a SIMD-accelerated SNN layer
def snn_layer_forward(layer: SNNLayer, inputs: vec8f) -> SNNLayerOutput {
    // 1. Synaptic dot product: I = W . X + bias
    let syn_current: f32 = simd_dot(layer.weights, inputs) + layer.bias

    // 2. LIF membrane integration
    let u_before = (layer.neuron.u_membrane * layer.neuron.decay_beta) + syn_current
    let step_res: LIFStepResult = lif_step_forward(layer.neuron, syn_current)

    let next_layer = SNNLayer {
        weights: layer.weights,
        bias: layer.bias,
        neuron: step_res.neuron,
        alpha_surrogate: layer.alpha_surrogate
    }

    let tape = SNNTapeEntry {
        u_before_spike: u_before,
        spike_emitted: step_res.spike,
        input_current: syn_current
    }

    return SNNLayerOutput {
        layer: next_layer,
        spike: step_res.spike,
        tape: tape
    }
}

// Backward pass computing weight gradient vector for the SNN layer
def snn_layer_backward(grad_loss: f32, tape: SNNTapeEntry, u_th: f32, alpha: f32, inputs: vec8f) -> vec8f {
    // 1. Surrogate gradient dL / dI = dL / dS * dS / dU
    let grad_i = lif_step_backward(grad_loss, tape.u_before_spike, u_th, alpha)

    // 2. Synaptic weight gradients: dL / dW = (dL / dI) * X
    let grad_i_vec: vec8f = simd_splat(grad_i)
    let grad_w: vec8f = inputs * grad_i_vec

    return grad_w
}
