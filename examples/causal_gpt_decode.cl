// ═══════════════════════════════════════════════════════════════════════════
// CRON (CL) — Zero-Word Causal GPT Self-Attention & Autoregressive Decoding
// Causal Masking (S_{i,j} = -∞ for j > i) + RoPE + RMSNorm + Argmax + Concat
// ═══════════════════════════════════════════════════════════════════════════

// 1. Оролтын 3 токены эмбеддинг (Seq_Len = 3, Dim = 4)
X = [[1.0, 0.0, 1.0, 0.0], [0.0, 1.0, 0.0, 1.0], [1.0, 1.0, 0.0, 0.0]]

// 2. Pre-RMSNorm + Query/Key/Value проекц
X_norm = rmsnorm(X)
W_q = eye(4)
W_k = eye(4)
W_v = [[2.0, 0.0, 0.0, 0.0], [0.0, 3.0, 0.0, 0.0], [0.0, 0.0, 4.0, 0.0], [0.0, 0.0, 0.0, 5.0]]

Q = X_norm @ W_q
K = X_norm @ W_k
V = X_norm @ W_v

// 3. Causal Self-Attention (Ирээдүйн токенуудыг автоматаар масклана)
H = X + causal_attn(Q, K, V)

// 4. Сүүлийн токены гаралтыг авч Vocabulary (Vocab_Size = 5) руу проекцлох
h_last = H[2]
W_vocab = [[0.1, 0.2, 0.9, 0.1, 0.3], [0.2, 0.1, 0.8, 0.2, 0.1], [0.5, 0.3, 1.2, 0.0, 0.2], [0.1, 0.4, 0.7, 0.3, 0.1]]
logits = h_last @ W_vocab

// 5. Хамгийн өндөр магадлалтай дараагийн токены ID-г сонгох (Argmax) ба дараалалд залгах (Concat)
next_token = argmax(logits)
prompt_tokens = [101.0, 2054.0, 2003.0]
generated_seq = concat(prompt_tokens, [next_token])

// Сонгогдсон токены индекс (2)
next_token

