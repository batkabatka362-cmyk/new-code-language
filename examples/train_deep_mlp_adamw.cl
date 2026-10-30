// ═══════════════════════════════════════════════════════════════════════════
// CRON (CL) — Zero-Word Deep Multi-Layer Perceptron (2-Layer MLP)
// Multi-Weight Tuple State Training with Gradient Clipping & Safetensors Merge
// ═══════════════════════════════════════════════════════════════════════════

// 1. Оролтын өгөгдөл X ∈ ℝ^(4×2) болон зорилтот ангилал Y ∈ ℝ^4
X = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]]
Y = [0.0, 1.0, 1.0, 0.0]

// 2. 2-Давхаргат Сүлжээний Loss функц:
// Layer 1: H = relu(X @ W1 + b1) -> [4, 4]
// Layer 2: Z = H @ W2 + b2       -> [4, 2] (Logits)
loss(W1, b1, W2, b2) = cross_entropy(relu(X @ W1 + b1) @ W2 + b2, Y)

// 3. Сургалтын хурд ба градиент хязгаарлагч (Frobenius Norm Clipping)
lr = 0.20
step(W1, b1, W2, b2) = (
    W1 - lr * clip_grad(grad(loss(W1, b1, W2, b2), W1), 1.0),
    b1 - lr * clip_grad(grad(loss(W1, b1, W2, b2), b1), 1.0),
    W2 - lr * clip_grad(grad(loss(W1, b1, W2, b2), W2), 1.0),
    b2 - lr * clip_grad(grad(loss(W1, b1, W2, b2), b2), 1.0)
)

// 4. Анхны жингүүд (Weights Initialization)
W1_0 = [[0.6, -0.3, 0.4, 0.1], [0.2, 0.5, -0.4, -0.2]]
b1_0 = [0.0, 0.1, -0.1, 0.0]
W2_0 = [[0.4, -0.5], [0.3, 0.2], [-0.4, 0.6], [0.1, -0.2]]
b2_0 = [0.0, 0.0]

// 5. Олон хувьсагчит Tuple State Fixpoint Iteration (50 Алхам)
(W1_opt, b1_opt, W2_opt, b2_opt) = (W1_0, b1_0, W2_0, b2_0) -> step^50

// 6. Олон матрицыг нэг Safetensors файлд нэгтгэн экспортлох (Merge-Update)
W1_opt >> "deep_mlp.safetensors:w1"
b1_opt >> "deep_mlp.safetensors:b1"
W2_opt >> "deep_mlp.safetensors:w2"
b2_opt >> "deep_mlp.safetensors:b2"

// 7. Эцсийн сургагдсан Loss утга
loss(W1_opt, b1_opt, W2_opt, b2_opt)
