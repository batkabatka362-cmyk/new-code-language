// ============================================================================
// CRON SOVEREIGN N-D TENSOR CALCULUS: W^T, Reshape, Outer, and Slicing
// Zero-Word High-Performance Matrix Engine
// ============================================================================

// 1. Reshape 1D vector into 2D Weight Matrix [2, 3]
raw_weights = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
W = reshape(raw_weights, [2, 3])

// 2. Matrix Transpose using standard mathematical notation W^T:
// W is [2, 3] -> W^T is [3, 2]
Wt = W^T

// 3. Matrix-Vector product with transposed matrix
x = [10.0, 20.0]
// W^T @ x -> [3, 2] @ [2] -> [3]
// [1*10 + 4*20, 2*10 + 5*20, 3*10 + 6*20] = [90.0, 120.0, 150.0]
y = Wt @ x

// 4. Outer Product between y and another vector:
// outer([90, 120, 150], [1, 2]) -> [3, 2] matrix
grad_w = outer(y, [1.0, 2.0])

// 5. Slice matrix row 0 and return its sum:
// Row 0 of grad_w is [90*1, 90*2] = [90, 180] -> sum = 270
r0 = slice(grad_w, 0, 1)
sum(r0)

