import unittest
import torch
import torch.nn as nn
import numpy as np
import os
import sys

# Ensure repository root is in python path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

import python.cron as cron
from python.cron.torch_bridge import (
    CronSparse24BitLinear,
    CronSubByteRingTapeAttention,
    CronMoERouter,
    CronNeuromorphicPlasticity,
    pack_sparse_24_ternary_weights,
    quantize_subbyte_kv_4bit,
    moe_route_topk,
    hebbian_oja_step,
    load_cl,
    load_clb,
)

class TestCronAiSiliconTorch(unittest.TestCase):
    def test_sparse_24_bitlinear(self):
        in_features = 64
        out_features = 128
        batch_size = 4
        
        linear = nn.Linear(in_features, out_features, bias=True)
        sparse_layer = CronSparse24BitLinear.from_linear(linear)
        
        x = torch.randn(batch_size, in_features, dtype=torch.float32)
        out = sparse_layer(x)
        
        self.assertEqual(out.shape, (batch_size, out_features))
        self.assertFalse(torch.isnan(out).any())
        self.assertFalse(torch.isinf(out).any())
        
        # Verify 2:4 sparsity packing compression:
        # 128 rows * (64 / 4) bytes = 128 * 16 = 2048 bytes
        expected_bytes = out_features * ((in_features + 3) // 4)
        self.assertEqual(sparse_layer.weight_packed.numel(), expected_bytes)

    def test_subbyte_kv_attention(self):
        dim = 64
        n_heads = 4
        capacity = 128
        seq_len = 8
        batch_size = 2
        
        attn = CronSubByteRingTapeAttention(dim=dim, n_heads=n_heads, capacity=capacity)
        x = torch.randn(batch_size, seq_len, dim, dtype=torch.float32)
        
        out = attn(x)
        self.assertEqual(out.shape, (batch_size, seq_len, dim))
        self.assertFalse(torch.isnan(out).any())
        
        # Verify 8x compressed KV tape storage:
        # head_dim = 16. In 4-bit, half_hd = 8 bytes per head per token.
        half_hd = (dim // n_heads + 1) // 2
        self.assertEqual(attn.k_tape_packed.shape, (batch_size, n_heads, capacity, half_hd))

    def test_moe_router(self):
        d_model = 64
        num_experts = 8
        top_k = 2
        
        router = CronMoERouter(d_model=d_model, num_experts=num_experts, top_k=top_k)
        x = torch.randn(d_model, dtype=torch.float32)
        
        routes, power_savings = router(x)
        self.assertEqual(len(routes), top_k)
        
        total_weight = sum(w for _, _, w in routes)
        self.assertAlmostEqual(total_weight, 1.0, places=3)
        
        # 8 experts, top 2 active -> 6/8 = 75% power savings
        self.assertGreater(power_savings, 50.0)
        self.assertLessEqual(power_savings, 100.0)

    def test_neuromorphic_plasticity(self):
        d_in = 32
        d_out = 16
        
        plasticity = CronNeuromorphicPlasticity(in_features=d_in, out_features=d_out, learning_rate=0.01)
        pre_syn = torch.randn(d_in, dtype=torch.float32)
        
        w_before = plasticity.weight.clone()
        plasticity.adapt(pre_syn)
        w_after = plasticity.weight
        
        # Weight must have adapted via Oja's rule
        diff = torch.norm(w_after - w_before).item()
        self.assertGreater(diff, 1e-6)

    def test_load_cl_and_clb(self):
        # Test loading .cl microcode
        cl_path = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "examples", "cl", "cron_bitnet_7b_foundation.cl"))
        if os.path.exists(cl_path):
            code = load_cl(cl_path)
            self.assertIn("B0000:", code)
            
        # Test loading .clb binary
        clb_path = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "examples", "cl", "cron_bitnet_7b_foundation.clb"))
        if os.path.exists(clb_path):
            binary = load_clb(clb_path)
            self.assertGreater(len(binary), 12)
            self.assertEqual(binary[:4], b"CLB1")
            # Binary payload: 12-byte header + N * 20-byte bundle records
            self.assertEqual((len(binary) - 12) % 20, 0)

if __name__ == "__main__":
    unittest.main()
