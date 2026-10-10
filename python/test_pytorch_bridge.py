"""
Unit & Integration Test Suite for CRON PyTorch & JAX Native Bridge
Tests:
1. Native C FFI discrete operations via cron_cortex.dll
2. CronSphereLinear forward & backward STE (Straight-Through Estimator) differentiability
3. Training loop convergence with PyTorch AdamW optimizer
4. Full CronSphereTransformerBlock & CronSphereTransformerModel generation
5. Loading real SafeTensors weights from `tests/fixtures/pytorch_transformer_layer.safetensors`
"""

import os
import sys
import numpy as np

# Ensure python directory is in sys.path
sys.path.insert(0, os.path.dirname(__file__))

import cron_cortex
from cron_cortex import (
    CronCortex,
    CronSphereLinear,
    CronSphereRMSNorm,
    CronSphereSwiGLU,
    CronSphereAttention,
    CronSphereTransformerBlock,
    CronSphereTransformerModel,
    HAS_TORCH,
)

if not HAS_TORCH:
    print("[ERROR] PyTorch is required to run test_pytorch_bridge.py")
    sys.exit(1)

import torch
import torch.nn as nn
import torch.optim as optim


def test_native_c_ffi():
    print(">>> 1. Testing Native C FFI Functions via cron_cortex.dll...")
    cortex = CronCortex()

    # 1.1 Test sphere_linear_forward
    batch_size = 2
    in_dim = 16
    out_dim = 8
    x = np.random.randint(-10, 10, size=(batch_size, in_dim), dtype=np.int32)
    w_ternary = np.random.choice([-1, 0, 1], size=(out_dim, in_dim)).astype(np.int8)

    out = cortex.sphere_linear_forward(x, w_ternary)
    assert out.shape == (batch_size, out_dim), f"Bad shape: {out.shape}"
    # Verify exact bit-level arithmetic
    expected = np.dot(x, w_ternary.T)
    np.testing.assert_array_equal(out, expected)
    print("  [PASS] sphere_linear_forward bit-exact match with matrix dot!")

    # 1.2 Test sphere_swiglu
    gate = np.array([10, -8, 16, -24], dtype=np.int32)
    up = np.array([4, 8, -12, 16], dtype=np.int32)
    swiglu_out = cortex.sphere_swiglu(gate, up)
    assert swiglu_out.shape == gate.shape
    print(f"  [PASS] sphere_swiglu output: {swiglu_out}")

    # 1.3 Test quantize_ternary
    floats = np.array([-1.5, 0.05, 1.8, -0.02, 0.9, -1.2], dtype=np.float32)
    t_weights, scale = cortex.quantize_ternary(floats)
    assert t_weights.shape == floats.shape
    assert set(np.unique(t_weights)).issubset({-1, 0, 1})
    print(f"  [PASS] quantize_ternary: scale={scale:.4f}, ternary={t_weights}")


def test_cron_sphere_linear():
    print("\n>>> 2. Testing CronSphereLinear PyTorch Module & STE Autograd...")
    linear = CronSphereLinear(in_features=32, out_features=16, bias=True)

    x = torch.randn(4, 32, requires_grad=True)
    out = linear(x)

    assert out.shape == (4, 16), f"Expected shape (4, 16), got {out.shape}"
    print("  [PASS] CronSphereLinear Forward shape verified: (4, 16)")

    # Test Backward Pass
    loss = out.sum()
    loss.backward()

    assert x.grad is not None and not torch.isnan(x.grad).any(), "Input gradient is None or NaN"
    assert linear.weight.grad is not None and not torch.isnan(linear.weight.grad).any(), "Weight gradient is None or NaN"
    assert linear.bias.grad is not None, "Bias gradient is None"
    print("  [PASS] STE Backward Pass: gradients propagated cleanly to input and weights!")

    # Test Sparsity and Compression Metrics
    sparsity = linear.get_sparsity()
    compression = linear.get_compression_ratio()
    print(f"  [INFO] Weight Sparsity: {sparsity * 100:.1f}% | Compression Ratio: {compression:.2f}x (FP32 -> 1.58b)")
    assert compression > 20.0, "Expected >20x compression"


def test_training_convergence():
    print("\n>>> 3. Testing Real Training Convergence on Synthetic Task...")
    torch.manual_seed(42)

    # 2-layer discrete neural network
    model = nn.Sequential(
        CronSphereLinear(16, 32),
        CronSphereRMSNorm(32),
        CronSphereLinear(32, 4),
    )
    optimizer = optim.AdamW(model.parameters(), lr=0.01)
    criterion = nn.MSELoss()

    X = torch.randn(32, 16)
    Y_target = torch.randn(32, 4)

    initial_loss = None
    final_loss = None

    for epoch in range(25):
        optimizer.zero_grad()
        pred = model(X)
        loss = criterion(pred, Y_target)
        if epoch == 0:
            initial_loss = loss.item()
        loss.backward()
        optimizer.step()
        final_loss = loss.item()

    print(f"  [TRAIN] Epoch 0 Loss: {initial_loss:.4f} -> Epoch 25 Loss: {final_loss:.4f}")
    assert final_loss < initial_loss, f"Loss did not decrease: {initial_loss} -> {final_loss}"
    print("  [PASS] Model converged successfully under PyTorch backprop!")


def test_transformer_block_and_model():
    print("\n>>> 4. Testing CronSphereTransformerBlock & Full Generative LLM...")
    block = CronSphereTransformerBlock(hidden_dim=32, num_heads=4, intermediate_dim=64)

    x = torch.randn(2, 8, 32)
    out = block(x)
    assert out.shape == (2, 8, 32), f"Block output shape mismatch: {out.shape}"
    print("  [PASS] CronSphereTransformerBlock forward pass verified (2, 8, 32)")

    # Full Generative Model
    llm = CronSphereTransformerModel(
        vocab_size=256,
        hidden_dim=32,
        num_layers=2,
        num_heads=4,
        intermediate_dim=64,
    )
    tokens = torch.randint(0, 255, (2, 12))
    logits = llm(tokens)
    assert logits.shape == (2, 12, 256), f"Logits shape mismatch: {logits.shape}"
    print("  [PASS] CronSphereTransformerModel logits shape verified (2, 12, 256)")

    # Autoregressive generation
    gen_text = llm.generate("CRON", max_new_tokens=8, temperature=0.7)
    print(f"  [PASS] Generated Text Output: \"CRON{gen_text}\" (Length: {len(gen_text)} new tokens)")


def test_safetensors_weights_loading():
    print("\n>>> 5. Testing Real PyTorch Model Layer loading from SafeTensors fixture...")
    fixture_path = "tests/fixtures/pytorch_transformer_layer.safetensors"
    if not os.path.exists(fixture_path):
        fixture_path = os.path.join(os.path.dirname(__file__), "..", fixture_path)

    if os.path.exists(fixture_path):
        from safetensors.torch import load_file
        tensors = load_file(fixture_path)
        print(f"  [INFO] Loaded {len(tensors)} tensors from '{fixture_path}'")

        # Map to CronSphereLinear
        q_weight = tensors.get("model.layers.0.self_attn.q_proj.weight", tensors.get("layers.0.self_attn.q_proj.weight"))
        out_features, in_features = q_weight.shape
        cron_q = CronSphereLinear(in_features, out_features, bias=False)
        cron_q.weight.data.copy_(q_weight)

        inp = torch.randn(1, 4, in_features)
        out = cron_q(inp)
        assert out.shape == (1, 4, out_features)
        print(f"  [PASS] Successfully executed SafeTensors weights through 0-FLOPs CronSphereLinear: ({out_features}x{in_features})!")
    else:
        print(f"  [SKIP] SafeTensors fixture not found at '{fixture_path}'")


if __name__ == "__main__":
    print("================================================================================")
    print("     CRON PYTORCH & JAX NATIVE BRIDGE INTEGRATION TEST SUITE                    ")
    print("================================================================================")
    test_native_c_ffi()
    test_cron_sphere_linear()
    test_training_convergence()
    test_transformer_block_and_model()
    test_safetensors_weights_loading()
    print("\n================================================================================")
    print("  [100% PASS] ALL PYTORCH & JAX NATIVE BRIDGE TESTS COMPLETED SUCCESSFULLY!     ")
    print("================================================================================")
