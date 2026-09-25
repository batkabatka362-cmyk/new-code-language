// ============================================================================
// CRON .cl Predictive Coding Engine (Zero-Backpropagation Brain Paradigm)
// Local Dendritic Error Learning & Free-Energy Minimization (Rao-Ballard Model)
// Target: Zero Global Backprop Buffers, Zero Landauer Dissipation, O(1) Local Plasticity
// ============================================================================


/// A dendritic compartment representation with apical top-down and basal bottom-up inputs.
#[derive(Debug, Clone, PartialEq)]
pub struct DendriticNeuron {
    pub id: usize,
    /// Somatic potential (internal state representation r_i)
    pub state: f64,
    /// Basal drive (sensory / bottom-up input)
    pub basal_input: f64,
    /// Apical drive (contextual / top-down prediction)
    pub apical_prediction: f64,
    /// Local dendritic prediction error: e_i = basal - apical
    pub prediction_error: f64,
    /// Local spiking threshold
    pub threshold: f64,
    /// Spike flag
    pub fired: bool,
}

impl DendriticNeuron {
    pub fn new(id: usize, threshold: f64) -> Self {
        Self {
            id,
            state: 0.0,
            basal_input: 0.0,
            apical_prediction: 0.0,
            prediction_error: 0.0,
            threshold,
            fired: false,
        }
    }

    /// Computes local error and updates somatic spike output.
    pub fn compute_local_error(&mut self) -> f64 {
        self.prediction_error = self.basal_input - self.apical_prediction;
        self.state = self.apical_prediction + 0.1 * self.prediction_error;
        self.fired = self.prediction_error.abs() > self.threshold;
        self.prediction_error
    }
}

/// A hierarchical Predictive Coding Layer executing local inference and learning.
#[derive(Debug, Clone, PartialEq)]
pub struct PredictiveCodingLayer {
    pub layer_id: usize,
    pub num_neurons: usize,
    pub neurons: Vec<DendriticNeuron>,
    /// Forward generative weights: predictions = W * higher_states
    /// Dimensions: [num_neurons x higher_layer_size]
    pub weights: Vec<Vec<f64>>,
    /// Learning rate for local synaptic update (eta)
    pub learning_rate: f64,
    /// State inference rate (alpha)
    pub inference_rate: f64,
    /// Total free-energy (prediction error variance / precision-weighted sum)
    pub free_energy: f64,
}

impl PredictiveCodingLayer {
    pub fn new(layer_id: usize, num_neurons: usize, higher_dim: usize, lr: f64) -> Self {
        let mut neurons = Vec::with_capacity(num_neurons);
        for i in 0..num_neurons {
            neurons.push(DendriticNeuron::new(i, 0.25));
        }

        // Initialize weights with deterministic normalized fan-in
        let mut weights = Vec::with_capacity(num_neurons);
        let scale = if higher_dim > 0 { 1.0 / (higher_dim as f64).sqrt() } else { 1.0 };
        for i in 0..num_neurons {
            let mut row = Vec::with_capacity(higher_dim);
            for j in 0..higher_dim {
                // Deterministic pseudo-random seed
                let w = ((i * 31 + j * 17) % 100) as f64 / 100.0 * 2.0 - 1.0;
                row.push(w * scale);
            }
            weights.push(row);
        }

        Self {
            layer_id,
            num_neurons,
            neurons,
            weights,
            learning_rate: lr,
            inference_rate: 0.15,
            free_energy: 0.0,
        }
    }

    /// Top-Down Generative Pass: Compute top-down predictions from higher-level representations.
    pub fn generate_predictions(&mut self, higher_states: &[f64]) {
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            let mut pred = 0.0;
            if !self.weights.is_empty() && i < self.weights.len() {
                for (j, &h_state) in higher_states.iter().enumerate() {
                    if j < self.weights[i].len() {
                        pred += self.weights[i][j] * h_state;
                    }
                }
            }
            neuron.apical_prediction = pred;
        }
    }

    /// Bottom-Up Error Evaluation: Ingest sensory / lower-layer inputs and compute prediction errors.
    pub fn evaluate_sensory_input(&mut self, sensory_inputs: &[f64]) -> f64 {
        let mut total_error_sq = 0.0;
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            if i < sensory_inputs.len() {
                neuron.basal_input = sensory_inputs[i];
            }
            let err = neuron.compute_local_error();
            total_error_sq += err * err;
        }
        self.free_energy = total_error_sq * 0.5;
        self.free_energy
    }

    /// Local Synaptic Update: W += eta * (error * higher_state^T)
    /// NO BACKPROPAGATION REQUIRED — completely local to the synapse!
    pub fn update_synaptic_weights(&mut self, higher_states: &[f64]) -> f64 {
        let mut max_dw = 0.0f64;
        for (i, neuron) in self.neurons.iter().enumerate() {
            if i < self.weights.len() {
                let err = neuron.prediction_error;
                for (j, &h_state) in higher_states.iter().enumerate() {
                    if j < self.weights[i].len() {
                        let dw = self.learning_rate * err * h_state;
                        self.weights[i][j] += dw;
                        if dw.abs() > max_dw {
                            max_dw = dw.abs();
                        }
                    }
                }
            }
        }
        max_dw
    }

    /// Returns current layer state vector (representations)
    pub fn get_states(&self) -> Vec<f64> {
        self.neurons.iter().map(|n| n.state).collect()
    }

    /// Returns prediction error vector
    pub fn get_errors(&self) -> Vec<f64> {
        self.neurons.iter().map(|n| n.prediction_error).collect()
    }
}

/// Multi-Level Hierarchical Predictive Coding Network (e.g. 3-level Cortical Column).
#[derive(Debug, Clone, PartialEq)]
pub struct HierarchicalPredictiveNetwork {
    pub layers: Vec<PredictiveCodingLayer>,
    pub top_representation: Vec<f64>,
    pub global_free_energy: f64,
}

impl HierarchicalPredictiveNetwork {
    /// Creates a 3-layer predictive hierarchy: e.g. Sensory (64) <- Intermediate (32) <- High-Level Concept (16)
    pub fn new(layer_sizes: &[usize], lr: f64) -> Self {
        let mut layers = Vec::new();
        for i in 0..layer_sizes.len() - 1 {
            let num_neurons = layer_sizes[i];
            let higher_dim = layer_sizes[i + 1];
            layers.push(PredictiveCodingLayer::new(i, num_neurons, higher_dim, lr));
        }

        let top_size = *layer_sizes.last().unwrap_or(&16);
        let top_representation = vec![0.1; top_size];

        Self {
            layers,
            top_representation,
            global_free_energy: 0.0,
        }
    }

    /// Full predictive cycle on a sensory input:
    /// 1. Top-down generation
    /// 2. Bottom-up error calculation
    /// 3. Local weight updates
    /// Returns total network free energy.
    pub fn step_cycle(&mut self, sensory_input: &[f64]) -> f64 {
        let n = self.layers.len();
        if n == 0 {
            return 0.0;
        }

        // 1. Top-Down generative predictions
        for l in (0..n).rev() {
            let higher_rep = if l == n - 1 {
                self.top_representation.clone()
            } else {
                self.layers[l + 1].get_states()
            };
            self.layers[l].generate_predictions(&higher_rep);
        }

        // 2. Bottom-Up error evaluation
        let mut current_input = sensory_input.to_vec();
        let mut total_fe = 0.0;
        for l in 0..n {
            total_fe += self.layers[l].evaluate_sensory_input(&current_input);
            current_input = self.layers[l].get_states();
        }
        self.global_free_energy = total_fe;

        // 3. Local Hebbian / Error weight updates (Zero-backprop)
        for l in 0..n {
            let higher_rep = if l == n - 1 {
                self.top_representation.clone()
            } else {
                self.layers[l + 1].get_states()
            };
            self.layers[l].update_synaptic_weights(&higher_rep);
        }

        total_fe
    }
}
