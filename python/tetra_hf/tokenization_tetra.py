from transformers import PreTrainedTokenizer
from typing import List, Optional, Dict

class TetraTokenizer(PreTrainedTokenizer):
    """
    DPSTA 94-тэмдэгт болон Блохын фазад зориулсан 0-overhead токенайзер.
    Нүсэр BPE json хүснэгт шаардахгүй, шууд тэмдэгтийн долгионы кодлол хийнэ.
    """
    def __init__(
        self,
        bos_token="<s>",
        eos_token="</s>",
        unk_token="<unk>",
        pad_token="<pad>",
        **kwargs
    ):
        self.id_to_token = {0: "<pad>", 1: "<s>", 2: "</s>", 3: "<unk>"}
        for c in range(33, 127):
            self.id_to_token[c] = chr(c)
        self.token_to_id = {v: k for k, v in self.id_to_token.items()}

        super().__init__(
            bos_token=bos_token,
            eos_token=eos_token,
            unk_token=unk_token,
            pad_token=pad_token,
            **kwargs
        )

    @property
    def vocab_size(self) -> int:
        return 131072 # Tetra-Tree 4-т модны хэмжээс

    def _tokenize(self, text: str) -> List[str]:
        return list(text)

    def _convert_token_to_id(self, token: str) -> int:
        if token in self.token_to_id:
            return self.token_to_id[token]
        if len(token) == 1:
            return ord(token) % 94 + 33
        return 3 # <unk>

    def _convert_id_to_token(self, index: int) -> str:
        if index in self.id_to_token:
            return self.id_to_token[index]
        return chr(index % 94 + 33)

    def get_vocab(self) -> Dict[str, int]:
        return dict(self.token_to_id)

    def save_vocabulary(self, save_directory: str, filename_prefix: Optional[str] = None):
        return ()
