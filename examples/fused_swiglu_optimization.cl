X   = [[1.0, 2.0], [3.0, 4.0]]
W_g = [[0.5, 0.2], [0.1, 0.4]]
W_u = [[0.2, 0.3], [0.4, 0.1]]
W_d = [[0.6, 0.1], [0.2, 0.5]]
b   = [0.05, 0.10]

Y_s = (((X @ W_g) * σ(X @ W_g)) * (X @ W_u)) @ W_d
Y_r = ((X @ W_g) + b) > 0.0 ? ((X @ W_g) + b) : 0.0

P_1 = (X @ W_d) + b
P_2 = (X @ W_d) + [0.2, 0.3]

Y_s >> "fused_swiglu_output.safetensors:swiglu_out"
Y_r >> "fused_swiglu_output.safetensors:linear_relu_out"
P_1 >> "fused_swiglu_output.safetensors:proj1"
P_2 >> "fused_swiglu_output.safetensors:proj2"
