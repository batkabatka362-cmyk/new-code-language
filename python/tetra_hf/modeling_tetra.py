import torch
import torch.nn as nn
import ctypes
import os
import sys
from typing import Optional, Tuple, Union, List
from transformers import PreTrainedModel, GenerationMixin
from transformers.modeling_outputs import CausalLMOutputWithPast
from .configuration_tetra import TetraConfig

class TetraEngineHandle(ctypes.Structure):
    pass

class TetraForCausalLM(PreTrainedModel, GenerationMixin):
    config_class = TetraConfig
    base_model_prefix = "model"
    _supports_flash_attn_2 = False
    _supports_sdpa = False

    def __init__(self, config: TetraConfig):
        super().__init__(config)
        self.config = config

        # Dummy param for HF validation
        self.dummy_param = nn.Parameter(torch.empty(0), requires_grad=False)
        self._init_c_backend()

    def _init_c_backend(self):
        lib_path = self.config.lib_path
        abs_lib_path = os.path.abspath(lib_path)
        if not os.path.exists(abs_lib_path):
            # Fallback check in current directory
            alt = os.path.abspath("libtetra_engine.dll" if sys.platform == "win32" else "libtetra_engine.so")
            if os.path.exists(alt):
                abs_lib_path = alt
            else:
                raise FileNotFoundError(f"[!] Динамик сан олдсонгүй: {abs_lib_path}")

        if sys.platform == "win32":
            os.add_dll_directory(os.path.dirname(abs_lib_path))

        self._lib = ctypes.CDLL(abs_lib_path)

        self._lib.tetra_create_engine.argtypes = [ctypes.c_char_p]
        self._lib.tetra_create_engine.restype = ctypes.POINTER(TetraEngineHandle)

        self._lib.tetra_step.argtypes = [ctypes.POINTER(TetraEngineHandle), ctypes.c_char]
        self._lib.tetra_step.restype = ctypes.c_char

        self._lib.tetra_destroy_engine.argtypes = [ctypes.POINTER(TetraEngineHandle)]
        self._lib.tetra_destroy_engine.restype = None

        model_path_bytes = self.config.model_bin_path.encode('utf-8')
        self._handle = self._lib.tetra_create_engine(model_path_bytes)
        if not self._handle:
            raise RuntimeError("[!] 1T Тетраэдр загварыг санах ойд холбоход алдаа гарлаа!")

    def get_input_embeddings(self):
        return None

    def set_input_embeddings(self, value):
        pass

    def get_output_embeddings(self):
        return None

    def forward(
        self,
        input_ids: torch.LongTensor = None,
        attention_mask: Optional[torch.Tensor] = None,
        position_ids: Optional[torch.LongTensor] = None,
        past_key_values: Optional[List[torch.FloatTensor]] = None,
        inputs_embeds: Optional[torch.FloatTensor] = None,
        labels: Optional[torch.LongTensor] = None,
        use_cache: Optional[bool] = True,
        output_attentions: Optional[bool] = False,
        output_hidden_states: Optional[bool] = False,
        return_dict: Optional[bool] = True,
    ) -> Union[Tuple, CausalLMOutputWithPast]:
        batch_size, seq_len = input_ids.shape
        device = input_ids.device

        last_chars = []
        for b in range(batch_size):
            tokens = input_ids[b].tolist()
            curr_char = ' '
            for t in tokens:
                char_in = chr(t % 94 + 33) if t != 0 else ' '
                curr_char = self._lib.tetra_step(self._handle, ctypes.c_char(char_in.encode('latin1', errors='ignore')))
            last_chars.append(ord(curr_char))

        logits = torch.full((batch_size, seq_len, self.config.vocab_size), -100.0, device=device)
        for b in range(batch_size):
            target_token = last_chars[b] % self.config.vocab_size
            logits[b, -1, target_token] = 10.0

        loss = None
        if labels is not None:
            loss_fct = nn.CrossEntropyLoss()
            loss = loss_fct(logits.view(-1, self.config.vocab_size), labels.view(-1))

        if not return_dict:
            output = (logits,)
            return ((loss,) + output) if loss is not None else output

        return CausalLMOutputWithPast(
            loss=loss,
            logits=logits,
            past_key_values=past_key_values,
            hidden_states=None,
            attentions=None,
        )

    def prepare_inputs_for_generation(
        self,
        input_ids,
        past_key_values=None,
        attention_mask=None,
        inputs_embeds=None,
        **kwargs
    ):
        if past_key_values is not None:
            input_ids = input_ids[:, -1:]

        return {
            "input_ids": input_ids,
            "past_key_values": past_key_values,
            "use_cache": kwargs.get("use_cache", True),
            "attention_mask": attention_mask,
        }

    def _reorder_cache(self, past_key_values, beam_idx):
        return past_key_values

    def __del__(self):
        if hasattr(self, '_handle') and self._handle:
            self._lib.tetra_destroy_engine(self._handle)
            self._handle = None
