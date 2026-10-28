import os
import torch
from safetensors.torch import save_file

def main():
    os.makedirs("models/sovereign_1b", exist_ok=True)
    out_path = "models/sovereign_1b/model.safetensors"

    print("[1/2] Generating BitNet b1.58 ternary tensor weights for Sovereign-1B...")
    # Sample tensors with ternary values {-1, 0, +1}
    tensors = {
        "model.embed_tokens.weight": torch.randn(128, 64),
        "model.layers.0.input_layernorm.weight": torch.ones(64),
        "model.layers.0.self_attn.q_proj.weight": torch.randint(-1, 2, (64, 64)).float(),
        "model.layers.0.self_attn.k_proj.weight": torch.randint(-1, 2, (16, 64)).float(),
        "model.layers.0.self_attn.v_proj.weight": torch.randint(-1, 2, (16, 64)).float(),
        "model.layers.0.self_attn.o_proj.weight": torch.randint(-1, 2, (64, 64)).float(),
        "model.layers.0.post_attention_layernorm.weight": torch.ones(64),
        "model.layers.0.mlp.gate_proj.weight": torch.randint(-1, 2, (128, 64)).float(),
        "model.layers.0.mlp.up_proj.weight": torch.randint(-1, 2, (128, 64)).float(),
        "model.layers.0.mlp.down_proj.weight": torch.randint(-1, 2, (64, 128)).float(),
        "model.norm.weight": torch.ones(64),
        "lm_head.weight": torch.randint(-1, 2, (128, 64)).float(),
    }

    print(f"[2/2] Saving SafeTensors archive to '{out_path}'...")
    save_file(tensors, out_path)
    print(f"[SUCCESS] Exported {len(tensors)} tensors to '{out_path}'.")

if __name__ == "__main__":
    main()
