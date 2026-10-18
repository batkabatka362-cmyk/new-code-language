; ============================================================================
; CRON-Sovereign-1B: Token Embedding Layer Module
; Converts incoming token ID to 2048-dim latent activation vector
; Precision: FP16 Activation, Partitioned SRAM Banking
; Target: Core[0, 0, 0, 0] (Primary Ingress Node)
; ============================================================================

.stage "sovereign_1b_embedding", params="65.5M", precision="fp16"
.tensor %token_id: [1], %W_emb: fp16x2[32000, 2048]
.layout {TP=4, EP=4, CP=4, PP=4, dim="4x4x4x4"}

; Pre-bound vocabulary embedding weights at Bank 0
.weights bank=0 offset=0 [0x3C00, 0x3800, 0x3400, 0x3000]

.kernel embedding_forward(%token_id: int32, %W_emb: fp16x2[32000, 2048]) -> (%hidden: fp16x2[2048]) {
    ; Receive input token from host IO
    %tok = receive()

    ; Lookup embedding table in Scratchpad SRAM Bank 0
    %emb = embedding_lookup(%tok, %W_emb)

    ; Scale embedding by sqrt(d_model)
    %hidden = rms_norm(%emb)

    ; Stream downstream to Layer 0 on Core[1, 0, 0, 0]
    emit %hidden -> Core[1, 0, 0, 0]
}
