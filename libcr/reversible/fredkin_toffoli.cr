// CRON Standard Library - Universal Reversible Logic Gates
// Module: cron.reversible.fredkin_toffoli
// Brain 3: Landauer-Limit Zero Entropy Computing (Delta S = 0)

.MODULE cron.reversible.fredkin_toffoli

// Fredkin Gate (Controlled-SWAP): Universal 3-bit reversible gate
// Conserves number of 1s (conservative logic) -> zero heat dissipation
// Maps directly to machine VLIW opcode _RF
def fredkin_gate(control: bool, lin a: linear u32, lin b: linear u32) -> (linear u32, linear u32) {
    if control {
        // Swap inputs reversibly
        return (b, a)
    } else {
        // Pass through unchanged
        return (a, b)
    }
}

// Toffoli Gate (Controlled-Controlled-NOT): Universal reversible logic gate
// Maps directly to machine VLIW opcode _TO
def toffoli_gate(ctrl1: bool, ctrl2: bool, lin target: linear u32) -> linear u32 {
    let flipped = if ctrl1 && ctrl2 {
        target ^ 1
    } else {
        target
    }
    consume(target)
    return flipped
}

// Reversible Entanglement: Bijective state coupling with exact inverse
def reversible_entangle_states(lin state_a: linear u32, lin state_b: linear u32) -> (linear u32, linear u32) {
    let out_a = state_a ^ state_b
    let out_b = state_b
    consume(state_a)
    consume(state_b)
    return (out_a, out_b)
}

// Reversible Disentanglement (Inverse Operator F^-1): Zero entropy loss
def reversible_disentangle_states(lin out_a: linear u32, lin out_b: linear u32) -> (linear u32, linear u32) {
    let orig_a = out_a ^ out_b
    let orig_b = out_b
    consume(out_a)
    consume(out_b)
    return (orig_a, orig_b)
}
