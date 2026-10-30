// ═══════════════════════════════════════════════════════════════════════════
// CRON (CL) — Zero-Word Language Model Training with Cross-Entropy Loss
// Full Pipeline: Token Embedding -> Linear Projection -> Cross-Entropy -> Autodiff
// ═══════════════════════════════════════════════════════════════════════════

// 1. Токен эмбеддинг хүснэгт E ∈ ℝ^(Vocab×Dim) (Vocab = 4, Dim = 3)
E = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0, 1.0]]

// 2. Оролтын токен ID-ууд (Tokens = [0, 2]) болон зорилтот дараагийн токенууд (Targets = [1, 0])
tokens = [0.0, 2.0]
targets = [1.0, 0.0]

// 3. Эмбеддинг хайлт: H ∈ ℝ^(2×3) матриц үүснэ
H = embed(E, tokens)

// 4. Шугаман проекц болон Cross-Entropy Loss: L(W) = cross_entropy(H @ W, targets)
// H @ W нь [2, 4] logits матриц, targets нь [2] вектор байна
loss(W) = cross_entropy(H @ W, targets)

// 5. 2D Матрицын жингийн автомат градиент буулгалт
lr = 0.25
step(W) = W - lr * grad(loss(W), W)

// 6. Анхны тэг матрицаас (zeros(3, 4)) эхлэн 60 алхам сургах
W_init = zeros(3, 4)
W_trained = W_init -> step^60

// 7. Сургагдсан жинг Safetensors руу экспортлох
W_trained >> "trained_lm_head.safetensors:W"

// Эцсийн Loss утгыг хэвлэх (анх ~1.386 байснаас буурч < 0.15 болно)
loss(W_trained)

