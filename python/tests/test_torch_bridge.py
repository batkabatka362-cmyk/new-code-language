# ============================================================================
# Unit Tests for CRON PyTorch Bridge (test_torch_bridge.py)
# ============================================================================

import unittest
import sys
import os
import torch
import torch.nn as nn
import numpy as np

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))
import cron

class TestCronTorchBridge(unittest.TestCase):

    def test_native_library_loading(self):
        lib_path = cron.find_native_library()
        self.assertTrue(os.path.exists(lib_path), f"Native library '{lib_path}' must exist")
        ver = cron.get_native_version()
        self.assertIn("CRON-Native-Accelerator", ver)
        self.assertIn("BitNet-b1.58", ver)

    def test_pack_ternary_weights(self):
        weight = torch.randn(64, 128)
        packed_w, scale = cron.pack_ternary_weights(weight)
        
        expected_bytes = (64 * 128 + 3) // 4
        self.assertEqual(packed_w.shape[0], expected_bytes)
        self.assertEqual(packed_w.dtype, torch.uint8)
        self.assertGreater(scale, 0.0)

    def test_cron_bit_linear_forward(self):
        batch_size = 4
        seq_len = 16
        in_dim = 64
        out_dim = 128
        
        linear = nn.Linear(in_dim, out_dim, bias=True)
        bit_linear = cron.CronBitLinear.from_linear(linear)
        
        x = torch.randn(batch_size, seq_len, in_dim)
        y = bit_linear(x)
        
        self.assertEqual(y.shape, (batch_size, seq_len, out_dim))
        self.assertFalse(torch.isnan(y).any(), "Output must not contain NaNs")
        self.assertFalse(torch.isinf(y).any(), "Output must not contain Infs")

    def test_model_layer_replacement(self):
        class MiniMLP(nn.Module):
            def __init__(self):
                super().__init__()
                self.fc1 = nn.Linear(32, 64)
                self.act = nn.ReLU()
                self.fc2 = nn.Linear(64, 16)
                
            def forward(self, x):
                return self.fc2(self.act(self.fc1(x)))
                
        model = MiniMLP()
        x = torch.randn(2, 32)
        
        # Replace layers with CRON BitLinear
        cron.replace_torch_layers(model)
        
        self.assertIsInstance(model.fc1, cron.CronBitLinear)
        self.assertIsInstance(model.fc2, cron.CronBitLinear)
        
        y = model(x)
        self.assertEqual(y.shape, (2, 16))

    def test_cron_rmsnorm(self):
        dim = 128
        norm = cron.CronRMSNorm(dim, eps=1e-6)
        x = torch.randn(8, 32, dim)

        # PyTorch Reference RMSNorm
        rms = torch.sqrt(torch.mean(x ** 2, dim=-1, keepdim=True) + 1e-6)
        ref_y = (x / rms) * norm.weight

        y = norm(x)
        self.assertEqual(y.shape, (8, 32, dim))
        max_diff = torch.max(torch.abs(y - ref_y)).item()
        self.assertLess(max_diff, 1e-4, f"RMSNorm output difference ({max_diff}) exceeds tolerance")

    def test_cron_ringtape_attention(self):
        batch = 2
        seq_len = 8
        dim = 64
        n_heads = 4
        attn = cron.CronRingTapeAttention(dim, n_heads, capacity=512)

        x = torch.randn(batch, seq_len, dim)
        out = attn(x)

        self.assertEqual(out.shape, (batch, seq_len, dim))
        self.assertFalse(torch.isnan(out).any())
        self.assertFalse(torch.isinf(out).any())

    def test_cron_transformer_block(self):
        batch = 2
        seq_len = 16
        dim = 128
        n_heads = 4
        mlp_dim = 256

        block = cron.CronTransformerBlock(dim, n_heads, mlp_dim, capacity=1024)
        x = torch.randn(batch, seq_len, dim)
        out = block(x)

        self.assertEqual(out.shape, (batch, seq_len, dim))
        self.assertFalse(torch.isnan(out).any())
        self.assertFalse(torch.isinf(out).any())

if __name__ == "__main__":
    unittest.main()
