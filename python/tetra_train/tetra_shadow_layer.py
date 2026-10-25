import torch
import torch.nn as nn
import torch.nn.functional as F
import math

class TetraSimplexSTE(torch.autograd.Function):
    """
    1.03125-Бит Тетраэдр Симплексийн Straight-Through Estimator (STE).
    Температурын зөөлөн хөлдөөлт (Temperature Annealing Soft-to-Hard):
    - Сургалтын эхэнд: T=1.0 орчимд tanh зөөлөн градиент урсаж, санамсаргүй локал минимумд гацахгүй.
    - Сургалтын төгсгөлд: T->0.01 болоход цэвэр {-1, +1} хатуу дискрет тетраэдр төлөвт шилжинэ.
    """
    @staticmethod
    def forward(ctx, weight_shadow, micro_gain, temperature=1.0):
        temp = max(float(temperature), 0.01)
        hard = torch.sign(weight_shadow)
        hard[hard == 0] = 1.0

        if temp > 0.05:
            # Зөөлөн ба хатуу төлөвийн холимог интерполяци
            soft = torch.tanh(weight_shadow / temp)
            alpha = max(0.0, min(1.0, 1.0 - temp))
            w_sign = alpha * hard + (1.0 - alpha) * soft
        else:
            w_sign = hard

        # Микро-масштаб gamma in [0..15]
        gamma = torch.clamp(torch.round(micro_gain), 0.0, 15.0)
        scale = 1.0 / (2.0 ** (15.0 - gamma))

        ctx.save_for_backward(weight_shadow, w_sign)
        ctx.temp = temp
        return w_sign * scale

    @staticmethod
    def backward(ctx, grad_output):
        weight_shadow, w_sign = ctx.saved_tensors
        temp = ctx.temp
        grad_weight = grad_output.clone()

        # Дасан зохицох хайчилбар (Adaptive Bound)
        bound = 1.5 if temp <= 0.1 else 2.5
        grad_weight[weight_shadow.abs() > bound] = 0.0

        # micro_gain-ийн градиент
        grad_gain = (grad_output * w_sign).sum(dim=-1, keepdim=True) * 0.01
        return grad_weight, grad_gain, None

class TetrahedralShadowLinear(nn.Module):
    """
    Тетраэдр 1.03125-Бит Сүүдэрт Градиентийн Давхарга (Shadow Manifold Layer).
    - 32 жин тутамд 4-битийн хуваалцсан бичил масштаб (Shared Micro-Gain).
    - Төлөв нуралтаас хамгаалах Тетраэдр түлхэлцлийн торгууль (Repulsion Penalty).
    """
    def __init__(self, in_features: int, out_features: int, bias: bool = False):
        super().__init__()
        assert in_features % 32 == 0, "in_features must be multiple of 32 (Tetra group size)"
        self.in_features = in_features
        self.out_features = out_features
        self.num_blocks = (out_features * in_features) // 32
        self.temperature = 1.0

        # Сүүдэрт олон төрөлт дээрх тасралтгүй жин (Continuous Shadow Weights)
        self.weight_shadow = nn.Parameter(torch.empty(out_features, in_features))
        # 32 жин тутмын хуваалцсан бичил масштаб (Micro-Gain gamma in [0..15])
        self.micro_gain = nn.Parameter(torch.full((out_features, in_features // 32, 1), 8.0))

        if bias:
            self.bias = nn.Parameter(torch.zeros(out_features))
        else:
            self.register_parameter('bias', None)

        self.reset_parameters()

    def set_temperature(self, temp: float):
        """Сургалтын температурыг тохируулах (T in [0.01..1.0])"""
        self.temperature = max(0.01, float(temp))

    def reset_parameters(self):
        # sp3 Тетраэдрын тэнцвэрт байрлалаар эхлүүлэх
        nn.init.kaiming_uniform_(self.weight_shadow, a=math.sqrt(5))
        with torch.no_grad():
            self.micro_gain.fill_(8.0)

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        out_f, in_f = self.out_features, self.in_features
        w_reshaped = self.weight_shadow.view(out_f, in_f // 32, 32)

        # 1.03-Бит Тетраэдр STE квантчлал (Температурын зөөлөн хөлдөөлттэй)
        w_quant = TetraSimplexSTE.apply(w_reshaped, self.micro_gain, self.temperature)
        w_eff = w_quant.view(out_f, in_f)

        return F.linear(x, w_eff, self.bias)

    def tetrahedral_repulsion_loss(self) -> torch.Tensor:
        """
        Цагаан номын 4.2 бүлэг: Тетраэдр Түлхэлцлийн Тогтворжуулалт (Tetrahedral Repulsion Field)
        L_tetra = sum_{i != j} (w_i . w_j + 1/3)^2
        109.4712 градусын өнцгийг барьж, орон зайн мэдээллийн эзлэхүүнийг нуралтаас хамгаална.
        """
        # Тетраэдр 4 оройн хоорондын косинус скаляр үржвэр ~ -1/3 байх ёстой
        # 4 оройт блокуудаар хэмжих
        w_4 = self.weight_shadow.view(-1, 4, 8) # 4 оройн бүлгүүд
        w_norm = F.normalize(w_4, p=2, dim=-1) # (N, 4, 8)
        
        # 4x4 грам матриц
        gram = torch.bmm(w_norm, w_norm.transpose(1, 2)) # (N, 4, 4)
        
        # Диагональ бус гишүүд (i != j) нь -1/3 (-0.33333) байх ёстой
        mask = ~torch.eye(4, dtype=torch.bool, device=self.weight_shadow.device).unsqueeze(0)
        off_diag = gram[mask.expand_as(gram)].view(-1, 12)
        
        loss_repulsion = torch.mean((off_diag + (1.0 / 3.0)) ** 2)
        return loss_repulsion
