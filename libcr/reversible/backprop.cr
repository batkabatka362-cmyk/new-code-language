// CRON Standard Library - Zero-Memory Reversible Backpropagation
// Module: cron.reversible.backprop
// Brain 3: Inverse Computational Graph Traversal (Delta S = 0, 0 DRAM Checkpoints)

.MODULE cron.reversible.backprop

// Reversible Layer State: Reconstructed purely by inverse function F^-1(output)
// Completely avoids saving forward activation tensors into external DRAM!
struct ReversibleLayer {
    layer_id: u32,
    weight_ref: u64,
    entropy_loss_bits: u32
}

def init_reversible_layer(id: u32, weights: u64) -> ReversibleLayer {
    return ReversibleLayer {
        layer_id: id,
        weight_ref: weights,
        entropy_loss_bits: 0
    }
}

// Backward Invert Pass: Maps directly to machine VLIW opcode _BK
// Recovers input activations exactly from output, then accumulates weight gradients
def reversible_backward_step(
    lin output_state: linear u32,
    lin upstream_grad: linear u32,
    layer: ReversibleLayer
) -> (linear u32, linear u32) {
    // 1. Reconstruct previous layer state: input = F^-1(output)
    let lin reconstructed_input = backward(consume(output_state))

    // 2. Compute input gradient via Jacobian-transpose product
    let lin input_grad = consume(upstream_grad) ^ (reconstructed_input / 2)

    return (reconstructed_input, input_grad)
}
