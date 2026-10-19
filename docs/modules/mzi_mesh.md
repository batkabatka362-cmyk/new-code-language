# Module `mzi_mesh`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct MziPhaseAngle`

| Field | Type |
|---|---|
| `theta_rad` | `u16` |
| `phi_rad` | `u16` |

### `struct MziGrid16x16`

| Field | Type |
|---|---|
| `phases` | `[MziPhaseAngle; 120]` |
| `center_wavelength_nm` | `u32` |
| `insertion_loss_db` | `u16` |

## ⚡ Functions & Intrinsics

### `fn init_mzi_mesh() -> MziGrid16x16`

### `fn optical_gemm_forward(input_wave: linear wave_t, mesh: MziGrid16x16) -> linear wave_t`

### `fn program_mzi_phases(mesh: linear MziGrid16x16, idx: u32, theta: u16, phi: u16) -> linear MziGrid16x16`

