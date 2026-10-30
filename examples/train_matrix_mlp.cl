// ═══════════════════════════════════════════════════════════════════════════
// CRON (CL) — Zero-Word 2D Matrix Neural Network Training & Broadcasting
// Full 2D Matrix Autodiff: ∇_W L(W) ∈ ℝ^(M×N) + Bias Broadcasting [B, D] + [D]
// ═══════════════════════════════════════════════════════════════════════════

// 1. Оролтын багц (Batch = 3, In_Dim = 2) болон зорилтот гаралт (Batch = 3, Out_Dim = 2)
X = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]]
Y = [[2.5, 0.5], [1.5, 3.5], [3.5, 3.5]]

// 2. Bias вектор b ∈ ℝ^2 ([3, 2] матриц дээр автоматаар broadcast хийгдэнэ)
b = [0.5, 0.5]

// 3. Шугаман давхарга ба Loss функц: L(W) = ∑ ((X @ W + b) - Y)²
pred(W) = X @ W + b
loss(W) = sum((pred(W) - Y)^2)

// 4. 2D Матрицын Градиент буулгалт (Matrix Gradient Descent):
// grad(loss(W), W) нь 2x2 хэмжээтэй бүтэн градиент матриц буцаана!
lr = 0.04
step(W) = W - lr * grad(loss(W), W)

// 5. Анхны тэг матрицаас (zeros(2, 2)) эхлэн 150 алхам сургах (Fixpoint iteration)
W_init = zeros(2, 2)
W_star = W_init -> step^150

// 6. Сургагдсан матрицын жинг Safetensors руу шууд хадгалах
W_star >> "trained_matrix_mlp.safetensors:W"

// Эцсийн Loss утгыг буцаах (~0.000000)
loss(W_star)

