//! Comprehensive Hidden Bug & Edge-Case Resilience Test Suite
//! Tests extreme input conditions, multi-byte UTF-8 handling, zero-norm edge cases,
//! and zero-overhead memory and parser robustness across .cl, VM, and LSP.

use cronc::cl_hyperdimensional::{HyperVector, HDC_DIM};
use cronc::cl_lsp::analyze_cl_source;
use cronc::cl_binary::{encode_slot_to_u32, decode_u32_to_slot};

#[test]
fn test_utf8_multibyte_safety_in_lsp() {
    // String with 2-byte and 3-byte UTF-8 characters ('ö', '€', '✓', 'Монгол')
    let unicode_cl_code = "\
; UTF-8 Comment: Төгс амьд AGI загвар ✓
B0000: _NO00$000> _NO00$000> _NO00$000> _NO00$000>
; Corrupted multi-byte tokens should be handled gracefully without panic
B0001: _OP01$28Fö _WD02$10A€ _NO00$000> _NO00$000>
";

    // Calling LSP linter on unicode source MUST NOT panic!
    let diags = analyze_cl_source(unicode_cl_code);
    assert!(!diags.is_empty(), "Should produce diagnostics for invalid slots without panicking");
}

#[test]
fn test_utf8_multibyte_safety_in_binary_encoder() {
    // Malformed/multi-byte unicode slot string
    let weird_slot = "'=00#FF€>";
    let encoded = encode_slot_to_u32(weird_slot);
    let decoded = decode_u32_to_slot(encoded);
    assert_eq!(decoded.len(), 10);
}

#[test]
fn test_hdc_zero_norm_division_safety() {
    let zero_vec_a = HyperVector::zero();
    let zero_vec_b = HyperVector::zero();
    
    // Similarity between two all-zero vectors must return 0.0 and NEVER NaN or panic!
    let sim = zero_vec_a.similarity(&zero_vec_b);
    assert_eq!(sim, 0.0);
    assert!(!sim.is_nan());
}

#[test]
fn test_hdc_extreme_permutation_wrapping() {
    let mut vec = HyperVector::zero();
    vec.set_trit(0, 1);
    
    // Shift by more than HDC_DIM (1024)
    let permuted = vec.permute(HDC_DIM * 5 + 3);
    assert_eq!(permuted.get_trit(3), 1);
}
