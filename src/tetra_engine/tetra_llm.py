import ctypes
import os
import sys

if hasattr(sys.stdout, 'reconfigure'):
    try:
        sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    except Exception:
        pass

# Windows дээр .dll, Linux дээр .so хайх
if sys.platform == "win32":
    LIB_NAME = "./libtetra_engine.dll"
else:
    LIB_NAME = "./libtetra_engine.so"

class TetraEngineHandle(ctypes.Structure):
    pass

class TetraLLM:
    """
    1.03-Бит Тетраэдр 1T загварын Python Ctypes интерфэйс.
    Дискнээс zero-copy mmap хийж OpenCL/SIMD INT32 цөмөөр ажиллана.
    """
    def __init__(self, model_path: str = "tetra_1t_foundation.bin", lib_path: str = LIB_NAME):
        abs_lib_path = os.path.abspath(lib_path)
        if not os.path.exists(abs_lib_path):
            raise FileNotFoundError(f"[!] Динамик сан олдсонгүй: {abs_lib_path}. Эхлээд санг компилно уу.")

        if sys.platform == "win32":
            os.add_dll_directory(os.path.dirname(abs_lib_path))

        self._lib = ctypes.CDLL(abs_lib_path)
        self._setup_bindings()

        model_path_bytes = model_path.encode('utf-8')
        print(f"[*] 1T Тетраэдр загварыг холбож байна: {model_path}")
        self._handle = self._lib.tetra_create_engine(model_path_bytes)

        if not self._handle:
            raise RuntimeError("[!] Загварыг холбоход алдаа гарлаа!")

    def _setup_bindings(self):
        # tetra_create_engine
        self._lib.tetra_create_engine.argtypes = [ctypes.c_char_p]
        self._lib.tetra_create_engine.restype = ctypes.POINTER(TetraEngineHandle)

        # tetra_step
        self._lib.tetra_step.argtypes = [ctypes.POINTER(TetraEngineHandle), ctypes.c_char]
        self._lib.tetra_step.restype = ctypes.c_char

        # tetra_generate_text
        self._lib.tetra_generate_text.argtypes = [
            ctypes.POINTER(TetraEngineHandle),
            ctypes.c_char_p,
            ctypes.c_size_t,
            ctypes.c_char_p,
            ctypes.c_size_t
        ]
        self._lib.tetra_generate_text.restype = ctypes.c_int

        # tetra_destroy_engine
        self._lib.tetra_destroy_engine.argtypes = [ctypes.POINTER(TetraEngineHandle)]
        self._lib.tetra_destroy_engine.restype = None

        # tetra_set_temperature
        self._lib.tetra_set_temperature.argtypes = [ctypes.POINTER(TetraEngineHandle), ctypes.c_float]
        self._lib.tetra_set_temperature.restype = None

        # tetra_set_seed
        self._lib.tetra_set_seed.argtypes = [ctypes.POINTER(TetraEngineHandle), ctypes.c_uint32]
        self._lib.tetra_set_seed.restype = None

    def set_temperature(self, temp: float):
        if hasattr(self, '_handle') and self._handle:
            self._lib.tetra_set_temperature(self._handle, ctypes.c_float(temp))

    def set_seed(self, seed: int):
        if hasattr(self, '_handle') and self._handle:
            self._lib.tetra_set_seed(self._handle, ctypes.c_uint32(seed))

    def generate(self, prompt: str, max_new_tokens: int = 40, temperature: float = 0.7) -> str:
        self.set_temperature(temperature)
        prompt_bytes = prompt.encode('utf-8')
        buffer_size = max_new_tokens + 1024
        out_buf = ctypes.create_string_buffer(buffer_size)

        ret = self._lib.tetra_generate_text(
            self._handle,
            prompt_bytes,
            ctypes.c_size_t(max_new_tokens),
            out_buf,
            ctypes.c_size_t(buffer_size)
        )

        if ret < 0:
            raise RuntimeError("[!] Генерацийн явцад алдаа гарлаа!")

        return out_buf.value.decode('utf-8', errors='ignore')

    def stream_generate(self, prompt: str, max_new_tokens: int = 40, temperature: float = 0.7):
        self.set_temperature(temperature)
        curr = b' '
        for ch in prompt.encode('utf-8'):
            curr = self._lib.tetra_step(self._handle, ctypes.c_char(ch))

        for _ in range(max_new_tokens):
            curr = self._lib.tetra_step(self._handle, curr)
            yield curr.decode('utf-8', errors='ignore')

    def __del__(self):
        if hasattr(self, '_handle') and self._handle:
            self._lib.tetra_destroy_engine(self._handle)
            self._handle = None

if __name__ == "__main__":
    llm = TetraLLM()
    prompt = "Mongol AGI:"
    res = llm.generate(prompt, 30)
    print(f"[Үр дүн]: '{prompt}' -> '{res}'")
