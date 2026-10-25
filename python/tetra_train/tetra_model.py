import torch
import torch.nn as nn
import torch.nn.functional as F
import math
try:
    from tetra_shadow_layer import TetrahedralShadowLinear
except ImportError:
    from .tetra_shadow_layer import TetrahedralShadowLinear

DPSTA_CHAR_BASE = 32
DPSTA_CHAR_COUNT = 95
DPSTA_HARMONIC_MULT = 0x02C9FB4D

class DPSTAWaveEmbedding(nn.Module):
    """
    DPSTA 95-Тэмдэгтийн Дискрет Фазын Долгион Үүсгэгч (Phase Wave Embedding).
    ASCII кодуудыг тоон шулуун дээр бус Блохын бөмбөрцгийн дискрет гармоник фаз руу буулгана.
    """
    def __init__(self, d_model: int):
        super().__init__()
        self.d_model = d_model
        # Суралцах боломжтой фазын төсөөллийн матриц
        self.phase_proj = nn.Embedding(DPSTA_CHAR_COUNT, d_model)
        self.reset_parameters()

    def reset_parameters(self):
        # Алтан харьцаа ба гармоник фазын шилжилтээр эхлүүлэх
        with torch.no_grad():
            angles = torch.arange(DPSTA_CHAR_COUNT, dtype=torch.float32) * (2.0 * math.pi / DPSTA_CHAR_COUNT)
            weights = torch.empty(DPSTA_CHAR_COUNT, self.d_model)
            for i in range(self.d_model):
                freq = float(i + 1)
                weights[:, i] = torch.sin(angles * freq) if (i % 2 == 0) else torch.cos(angles * freq)
            self.phase_proj.weight.copy_(weights)

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        # x: (batch_size, seq_len) in [32..126]
        clamped_x = torch.clamp(x - DPSTA_CHAR_BASE, 0, DPSTA_CHAR_COUNT - 1)
        return self.phase_proj(clamped_x)

class TetraRMSNorm(nn.Module):
    """
    Тетраэдр хурдан RMSNorm.
    """
    def __init__(self, d_model: int, eps: float = 1e-5):
        super().__init__()
        self.eps = eps
        self.weight = nn.Parameter(torch.ones(d_model))

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        variance = x.pow(2).mean(-1, keepdim=True)
        x_norm = x * torch.rsqrt(variance + self.eps)
        return self.weight * x_norm

class TetraBlock(nn.Module):
    """
    1.03-Бит Тетраэдр Симплекс Блок (Self-Attention + Tetrahedral MLP).
    """
    def __init__(self, d_model: int, n_heads: int = 8):
        super().__init__()
        self.d_model = d_model
        self.n_heads = n_heads
        self.head_dim = d_model // n_heads

        self.norm1 = TetraRMSNorm(d_model)
        self.q_proj = TetrahedralShadowLinear(d_model, d_model)
        self.k_proj = TetrahedralShadowLinear(d_model, d_model)
        self.v_proj = TetrahedralShadowLinear(d_model, d_model)
        self.out_proj = TetrahedralShadowLinear(d_model, d_model)

        self.norm2 = TetraRMSNorm(d_model)
        self.mlp_gate = TetrahedralShadowLinear(d_model, d_model * 2)
        self.mlp_down = TetrahedralShadowLinear(d_model * 2, d_model)

    def forward(self, x: torch.Tensor, mask: torch.Tensor = None) -> torch.Tensor:
        b, s, d = x.shape

        # Attention
        norm_x = self.norm1(x)
        q = self.q_proj(norm_x).view(b, s, self.n_heads, self.head_dim).transpose(1, 2)
        k = self.k_proj(norm_x).view(b, s, self.n_heads, self.head_dim).transpose(1, 2)
        v = self.v_proj(norm_x).view(b, s, self.n_heads, self.head_dim).transpose(1, 2)

        # Causal Attention Score
        scores = torch.matmul(q, k.transpose(-2, -1)) / math.sqrt(self.head_dim)
        if mask is not None:
            scores = scores + mask

        attn_weights = F.softmax(scores, dim=-1)
        attn_out = torch.matmul(attn_weights, v)
        attn_out = attn_out.transpose(1, 2).contiguous().view(b, s, d)
        x = x + self.out_proj(attn_out)

        # Tetrahedral MLP
        norm_x2 = self.norm2(x)
        gate = F.silu(self.mlp_gate(norm_x2))
        x = x + self.mlp_down(gate)

        return x

    def set_temperature(self, temp: float):
        """Давхаргын бүх Тетраэдр жингүүдийн температурыг тохируулах"""
        self.q_proj.set_temperature(temp)
        self.k_proj.set_temperature(temp)
        self.v_proj.set_temperature(temp)
        self.out_proj.set_temperature(temp)
        self.mlp_gate.set_temperature(temp)
        self.mlp_down.set_temperature(temp)

    def repulsion_loss(self) -> torch.Tensor:
        return (
            self.q_proj.tetrahedral_repulsion_loss() +
            self.k_proj.tetrahedral_repulsion_loss() +
            self.v_proj.tetrahedral_repulsion_loss() +
            self.out_proj.tetrahedral_repulsion_loss() +
            self.mlp_gate.tetrahedral_repulsion_loss() +
            self.mlp_down.tetrahedral_repulsion_loss()
        )

class TetraLM(nn.Module):
    """
    1.03-Бит Тетраэдр Causal Language Model.
    Бүрэн сургагдах боломжтой, дискрет 1.03-бит файл руу шууд хөрвөнө.
    """
    def __init__(self, d_model: int = 512, num_layers: int = 6, n_heads: int = 8, vocab_size: int = DPSTA_CHAR_COUNT):
        super().__init__()
        assert d_model % 32 == 0
        self.d_model = d_model
        self.num_layers = num_layers
        self.vocab_size = vocab_size

        self.tok_embed = DPSTAWaveEmbedding(d_model)
        self.layers = nn.ModuleList([TetraBlock(d_model, n_heads) for _ in range(num_layers)])
        self.norm = TetraRMSNorm(d_model)
        self.lm_head = nn.Linear(d_model, vocab_size, bias=False)

    def set_temperature(self, temp: float):
        """Бүх давхаргуудад температурыг дамжуулах (Annealing)"""
        for layer in self.layers:
            layer.set_temperature(temp)

    def forward(self, input_ids: torch.Tensor, targets: torch.Tensor = None):
        b, s = input_ids.shape
        x = self.tok_embed(input_ids)

        # Causal mask
        mask = torch.full((s, s), float('-inf'), device=input_ids.device)
        mask = torch.triu(mask, diagonal=1).unsqueeze(0).unsqueeze(0)

        for layer in self.layers:
            x = layer(x, mask)

        x = self.norm(x)
        logits = self.lm_head(x)

        loss = None
        if targets is not None:
            # Cross-Entropy Loss
            target_indices = torch.clamp(targets - DPSTA_CHAR_BASE, 0, DPSTA_CHAR_COUNT - 1)
            loss_ce = F.cross_entropy(logits.view(-1, self.vocab_size), target_indices.view(-1))

            # Тетраэдр түлхэлцлийн торгууль (109.47 градус хамгаалах)
            rep_loss = sum(layer.repulsion_loss() for layer in self.layers) / len(self.layers)
            loss = loss_ce + 0.05 * rep_loss

        return logits, loss
