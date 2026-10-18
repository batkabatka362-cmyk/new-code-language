; ============================================================================
; CRON-Sovereign-1B: Modular Master Pipeline
; Uses .include directives to assemble the complete 1.14B AI model
; from independent, verified sub-modules with zero code duplication.
; ============================================================================

.stage "sovereign_1b_modular", params="1.14B", precision="1.58b_ternary"

; 1. Architecture specification & 4D-Torus hardware dimensions
.include "config.cl"

; 2. Ingress Token Embedding Layer
.include "embedding.cl"

; 3. Pre-Attention & Pre-FFN RMSNorm Layer
.include "rms_norm.cl"

; 4. Multi-Head Latent Attention & FlashAttention-3
.include "attention.cl"

; 5. SwiGLU Feed-Forward Network
.include "swiglu_mlp.cl"

; 6. Native SafeTensors Quantized Weight Bindings (Banks 0..11)
.include "weights.cl"

; 7. Autoregressive Generation Loop & Paged KV-Cache
.include "generation_loop.cl"

; 8. Trained On-Device CL-LoRA Adaptations
.include "sovereign_adapter.cl"

; 9. Multi-Modal Sensory Ingress: Retinal Vision & Cochlear Audio Streams
.include "vision_stream.cl"
.include "audio_stream.cl"


