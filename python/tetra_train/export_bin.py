import struct
import torch
import os
try:
    from tetra_model import TetraLM
    from tetra_shadow_layer import TetrahedralShadowLinear
except ImportError:
    from .tetra_model import TetraLM
    from .tetra_shadow_layer import TetrahedralShadowLinear

def export_model_to_bin(model: TetraLM, output_path: str):
    """
    Сургагдсан PyTorch TetraLM загварыг дискний шууд zero-copy mmap формат руу хөрвүүлэн хадгална.
    Формат: TetraModelHeader + [TetraBlockDisk ... ]
    """
    model.eval()
    device = next(model.parameters()).device
    
    # 1. Бүх Тетраэдр давхаргуудын жинг цуглуулах
    tetra_layers = []
    for layer in model.layers:
        sub_layers = [
            layer.q_proj, layer.k_proj, layer.v_proj, layer.out_proj,
            layer.mlp_gate, layer.mlp_down
        ]
        tetra_layers.extend(sub_layers)

    # Нийт параметр тооцох
    total_weights = sum(l.out_features * l.in_features for l in tetra_layers)
    total_blocks = total_weights // 32

    header_magic = b"TETRA1T\x00"
    version = 3
    total_params = total_weights
    d_model = model.d_model
    num_layers = len(model.layers)
    vocab_size = 131072
    header_size = 48 # sizeof(TetraModelHeader)
    weights_offset = header_size

    header_bytes = struct.pack(
        "<8sI4xQIII4xQ",
        header_magic,
        version,
        total_params,
        d_model,
        num_layers,
        vocab_size,
        weights_offset
    )

    print(f"[*] Загварыг экспортлож байна: {output_path}")
    print(f"    Нийт параметр: {total_params}, Блок тоо: {total_blocks}, Давхарга: {num_layers}")

    with open(output_path, "wb") as f:
        f.write(header_bytes)

        # Блок бүрийг TetraBlockDisk болгон савлах
        for l in tetra_layers:
            w = l.weight_shadow.detach().cpu()
            gain = l.micro_gain.detach().cpu().squeeze(-1) # (out_f, in_f // 32)
            out_f, in_f = w.shape

            w_reshaped = w.view(out_f, in_f // 32, 32) # (out_f, blocks_per_row, 32)
            
            for r in range(out_f):
                for b in range(in_f // 32):
                    block_weights = w_reshaped[r, b] # 32 тасралтгүй жин
                    g_val = int(torch.clamp(torch.round(gain[r, b]), 0.0, 15.0).item())

                    # 32 битийг sign-аар битмаск болгох
                    mask = 0
                    for bit_idx in range(32):
                        if block_weights[bit_idx] >= 0:
                            mask |= (1 << bit_idx)

                    # Тетраэдр 4 оройн фазын задрал (v0, v1, v2, v3)
                    v0 = mask
                    v1 = (mask ^ 0x55555555) & 0xFFFFFFFF
                    v2 = (mask ^ 0x33333333) & 0xFFFFFFFF
                    v3 = (mask ^ 0x0F0F0F0F) & 0xFFFFFFFF

                    # TetraBlockDisk (17 байт: 4xuint32 + 1xuint8)
                    block_bytes = struct.pack("<IIIIB", v0, v1, v2, v3, g_val)
                    f.write(block_bytes)

    file_size_mb = os.path.getsize(output_path) / (1024.0 * 1024.0)
    print(f"[+] Экспорт амжилттай дууслаа! Файлын хэмжээ: {file_size_mb:.2f} MB")
