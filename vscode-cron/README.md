# CRON Language Support for Visual Studio Code & Antigravity IDE

Official language extension for **CRON (`.cr`)** and **CRON Low-Level Machine Code (`.cl`)**, targeted at the 256-Core 4D-Torus Neuromorphic Photonic Processor.

## Features

- **Rich Syntax Highlighting**:
  - Full support for `.cr` high-level cognitive DSLs (Linear types, Brain primitives, Regions, Superposition).
  - VLIW bundle & slot syntax highlighting for `.cl` machine instructions.
- **Language Server Protocol (LSP 3.17)**:
  - Powered by native Rust `cron lsp`.
  - Real-time compiler diagnostics:
    - Linear ownership leaks (`$E0002$`)
    - Capability-based security violations (`$E0010$`)
    - Taint tracking memory safety (`$E0009$`)
    - Immutable assignment errors (`$E0005$`)
  - Hover tooltips with full Markdown documentation for hardware intrinsics.
  - Autocompletion for keywords, standard library modules, and brain constructs.
- **Integrated High-Performance Toolchain**:
  - `CRON: Compile to C23`
  - `CRON: Compile to Native Executable (GCC/Clang)`

## Installation

### From VSIX Package
In VS Code or Antigravity IDE:
1. Open the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`).
2. Run `Extensions: Install from VSIX...`
3. Select `vscode-cron.vsix` or link the `vscode-cron/` folder into your `.vscode/extensions/` directory.

### Manual Link
```bash
# Windows PowerShell:
Copy-Item -Recurse -Force "vscode-cron" "$env:USERPROFILE\.vscode\extensions\cron-lang"
```
