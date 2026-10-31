<< "libcl/math/adam.cl"

L(w) = (w * 3.0 - 15.0)^2

η = 0.05
Δ(w) = w - η * ∇(L(w), w)

w_t = 0.0 -> Δ^40

w_t >> "checkpoints/linear_model.safetensors:weight"

L(w_t)
