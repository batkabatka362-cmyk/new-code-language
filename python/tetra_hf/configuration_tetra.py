from transformers import PretrainedConfig

class TetraConfig(PretrainedConfig):
    model_type = "tetra_llm"

    def __init__(
        self,
        vocab_size: int = 131072,         # Tetra-Tree 9-түвшний 4-т модны навчны тоо
        d_model: int = 8192,              # Нууцлагдмал хэмжээс (64 ширхэг 128-бит блок)
        num_hidden_layers: int = 80,      # 1T загварын 80 давхарга
        bits_per_weight: float = 1.03125, # 128-бит жин + 4-бит микро-масштаб
        micro_gain_bits: int = 4,         # Shared Micro-Gain gamma
        dpsta_mode: str = "bloch_3d",     # 94-тэмдэгт болон Монгол Кирилл фазын горим
        max_position_embeddings: int = 4096,
        lib_path: str = "./libtetra_engine.dll",
        model_bin_path: str = "tetra_1t_foundation.bin",
        pad_token_id: int = 0,
        bos_token_id: int = 1,
        eos_token_id: int = 2,
        **kwargs,
    ):
        super().__init__(
            pad_token_id=pad_token_id,
            bos_token_id=bos_token_id,
            eos_token_id=eos_token_id,
            **kwargs,
        )
        self.vocab_size = vocab_size
        self.d_model = d_model
        self.num_hidden_layers = num_hidden_layers
        self.bits_per_weight = bits_per_weight
        self.micro_gain_bits = micro_gain_bits
        self.dpsta_mode = dpsta_mode
        self.max_position_embeddings = max_position_embeddings
        self.lib_path = lib_path
        self.model_bin_path = model_bin_path
