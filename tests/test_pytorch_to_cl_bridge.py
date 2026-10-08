# ============================================================================
# CRON & PyTorch End-to-End Integration Verification
# Tests conversion of real PyTorch / SafeTensors neural networks into
# 4D Spherical Curvature Machine Code (.cl) and BitNet b1.58 Silicon Tensors
# ============================================================================

import os
import sys
import torch
import torch.nn as nn
from safetensors.torch import save_file

# Add python path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "python")))
from cron.sphere_converter import SphereConverter, SphereLinear, export_tensor_to_sphere_cl, quantize_to_bitnet_b158

def test_pytorch_bridge():
    fixtures_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "fixtures"))
    os.makedirs(fixtures_dir, exist_ok=True)
    
    st_path = os.path.join(fixtures_dir, "pytorch_transformer_layer.safetensors")
    cl_out_path = os.path.join(fixtures_dir, "pytorch_transformer_sphere")
    
    # 1. Build canonical PyTorch Transformer Feed-Forward / Attention block
    model_tensors = {
        "model.layers.0.self_attn.q_proj.weight": torch.randn(64, 64, dtype=torch.float32) * 0.05,
        "model.layers.0.self_attn.k_proj.weight": torch.randn(64, 64, dtype=torch.float32) * 0.05,
        "model.layers.0.self_attn.v_proj.weight": torch.randn(64, 64, dtype=torch.float32) * 0.05,
        "model.layers.0.mlp.gate_proj.weight": torch.randn(128, 64, dtype=torch.float32) * 0.04,
        "model.layers.0.mlp.down_proj.weight": torch.randn(64, 128, dtype=torch.float32) * 0.04,
    }
    save_file(model_tensors, st_path)
    print(f"[+] Saved PyTorch SafeTensors model to: {st_path}")
    
    # 2. Invoke CRON Universal Converter CLI
    print("[*] Running CRON cl-convert on PyTorch model...")
    report = SphereConverter.convert_file(st_path, cl_out_path, json_mode=True)
    
    print("[+] Model Conversion Report Received:")
    print(f"    - Converted Tensors: {report['total_tensors_converted']}")
    print(f"    - Total Parameters: {report['total_parameters']:,} params")
    print(f"    - FP32 Footprint: {report['original_fp32_bytes'] / 1024:.2f} KB")
    print(f"    - Ternary Footprint: {report['quantized_ternary_bytes'] / 1024:.2f} KB")
    print(f"    - Memory Compression: {report['overall_compression_ratio']:.2f}x")
    print(f"    - Zero-FLOP Guarantee: {report['zero_flop_guarantee']}")
    print(f"    - Synthesized CL Bundles: {report['total_cl_bundles']:,}")
    print(f"    - 4D Geodesic Ops: {report['total_geodesic_operations']:,}")
    
    assert report['total_tensors_converted'] == 5
    assert report['zero_flop_guarantee'] is True
    assert report['overall_compression_ratio'] >= 16.0
    
    # 3. Check emitted artifacts
    cl_file = f"{cl_out_path}.cl"
    sphere_file = f"{cl_out_path}.sphere"
    assert os.path.exists(cl_file), f"Missing {cl_file}"
    assert os.path.exists(sphere_file), f"Missing {sphere_file}"
    
    with open(cl_file, "r", encoding="utf-8") as f:
        cl_head = "".join([f.readline() for _ in range(25)])
    print(f"[+] Emitted CL Machine Code Sample:\n{cl_head}")
    
    # 4. Test SphereLinear PyTorch module
    print("[*] Testing SphereLinear Drop-in Module...")
    layer = SphereLinear(in_features=32, out_features=16)
    x = torch.randn(4, 32)
    y = layer(x)
    assert y.shape == (4, 16)
    print(f"[+] SphereLinear forward pass successful: input {x.shape} -> output {y.shape}")
    
    exported_cl = layer.export_cl("custom_sphere_linear")
    assert "@kernel custom_sphere_linear" in exported_cl
    assert ".target silicon.sphere_isa" in exported_cl
    print("[+] Direct layer.export_cl() verified.")
    
    print("\n=======================================================")
    print("ALL PYTORCH -> CRON 4D SPHERICAL CONVERSION TESTS PASSED!")
    print("=======================================================")

if __name__ == "__main__":
    test_pytorch_bridge()
