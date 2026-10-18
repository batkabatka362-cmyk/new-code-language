//! CRON CLI Modular Command Subsystems (crates/cron-cli/src/commands/)
//!
//! Provides clean, decoupled command handlers:
//! - `cmd_sphere`: 4D Sphere LLM, Geodesic Curvature, and Clifford operations
//! - `cmd_cl`: Low-level .cl compilation, assembly, formatting, and linting
//! - `cmd_sim`: 256-Core 4D-Torus simulation, FPGA & ASIC tapeout flows
//! - `cmd_pkg`: Silicon package manager & module registry
//! - `cmd_ai`: Inference, MLA, Titans, Liquid, and Fine-Tuning pipelines
//! - `cmd_transpile`: Multi-target transpilation (C23, LLVM, Verilog, WASM)
//! - `cmd_rl`: TinyZero GRPO pure reinforcement learning and verification

pub mod cmd_sphere;
pub mod cmd_cl;
pub mod cmd_sim;
pub mod cmd_pkg;
pub mod cmd_ai;
pub mod cmd_transpile;
pub mod cmd_rl;
