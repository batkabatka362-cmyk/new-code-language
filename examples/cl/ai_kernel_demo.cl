.stage "bitnet_7b_flash_kernel", params="7.544B", precision="1.58b_ternary"
.tensor %Q: [32, 128], %K: [8, 128], %V: [8, 128]
.layout {TP=4, EP=4, CP=4, PP=4, dim="4x4x4x4"}

.kernel neural_attention_pipeline(%Q: trit16[32, 128], %K: trit16[8, 128]) -> (%O: fp16x2[32, 128]) {
    %q_norm = rms_norm(%Q)
    %attn = flash_attn(%q_norm, %K)
    %out = swiglu(%attn)
    emit %out -> Core[1, 0, 0, 0]
}
