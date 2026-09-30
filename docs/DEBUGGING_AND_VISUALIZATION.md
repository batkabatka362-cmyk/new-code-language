# CRON Debugging & Visualization Architecture (v1.3.0)

This document provides a comprehensive guide to debugging and hardware telemetry visualization in the **CRON** ecosystem, including the **Debug Adapter Protocol (DAP)** server, reverse time-travel execution, and the **VS Code 4D-Torus Mesh Visualizer**.

---

## 1. Overview & Architectural Design

The CRON debugging system bridges high-level cognitive programs (`.cr`), low-level VLIW microcode bundles (`.cl`), and physical silicon hardware simulation through standard language and debugging protocols:

```
+-----------------------------------------------------------------------+
|                         VS Code IDE (Client)                          |
|  [Run & Debug View]   [Breakpoints & Watches]   [4D-Torus Visualizer] |
+-----------------------------------+-----------------------------------+
                                    |
                  DAP (JSON-RPC over stdio / port)
                                    |
+-----------------------------------v-----------------------------------+
|               CRON DAP Server (`cron dap` / `cron-lsp`)               |
|  - Request Dispatcher & Protocol Serializer                           |
|  - Breakpoint Manager (Conditional, Opcode, Line)                     |
|  - Scope & Variable Inspector (Registers, Waves, Synapses, Telemetry) |
|  - Reverse Execution Controller (`stepBack` Snapshot Ring-Buffer)     |
+-----------------------------------+-----------------------------------+
                                    |
                             SimAPI / IPC
                                    |
+-----------------------------------v-----------------------------------+
|            CRON Virtual Machine (`cron-vm` / `ClDebugger`)            |
|  - 256-Core 4D-Torus Interconnect (VC0..VC3 Routers)                  |
|  - 16-Bank PGAS SRAM with GF(2^4) Conflict-Free Swizzling             |
|  - Photonic MZI Phase Grid & Thermodynamic Landauer Sinks              |
|  - Real-Time Fourier Heat Diffusion & Thermal Throttling Simulator    |
+-----------------------------------------------------------------------+
```

---

## 2. Debug Adapter Protocol (DAP) Standard Server

CRON includes a built-in DAP server compliant with the standard **Debug Adapter Protocol (v1.51+)**. The DAP server can be launched directly from the CLI or spawned transparently by the VS Code extension.

### 2.1 Starting the DAP Server

```bash
# Launch DAP server listening on standard I/O (default for VS Code integration)
cron dap

# Or test via cargo in the workspace
cargo run --bin cron -- dap
```

DAP messages use HTTP-style header framing:
```http
Content-Length: 124

{"seq":1,"type":"request","command":"initialize","arguments":{"clientID":"vscode","adapterID":"cron-debug"}}
```

### 2.2 Capabilities Negotiated

| DAP Capability | Supported | Description |
| :--- | :---: | :--- |
| `supportsConfigurationDoneRequest` | **Yes** | Signals when initial breakpoints and config have been sent |
| `supportsConditionalBreakpoints` | **Yes** | Evaluates expressions such as `R0 == 42` or `temp >= 105` |
| `supportsInstructionBreakpoints` | **Yes** | Allows breaking on specific opcodes (e.g. `_HL`, `_OP`, `_FA`) |
| `supportsStepBack` | **Yes** | **True Reverse Debugging**: step backward one machine cycle |
| `supportsSetVariable` | **Yes** | Live modification of registers and memory while paused |
| `supportsTerminateRequest` | **Yes** | Clean shutdown and resource teardown |

---

## 3. Supported DAP Commands & Inspection Scopes

### 3.1 Breakpoint Management (`setBreakpoints`)
- **Source Breakpoints**: Set by line number in `.cl` or `.cr` source files.
- **Conditional Breakpoints**: Evaluated dynamically on each cycle:
  - Register conditions: `R0 == 42`, `R1 != 0`, `R2 > 1000`
  - Telemetry conditions: `temp >= 105.0`, `power > 35.0`
  - Trap/Halt conditions: `trap`, `halt`
- **Opcode Breakpoints**: Trigger immediately whenever a target micro-op is decoded (e.g., `_HL`, `_FA`, `_OP`).

### 3.2 Stepping & Execution Flow
- `next` (**Step Over**): Advances the simulator by exactly 1 bundle / 1 cycle.
- `stepIn` (**Step Into**): Steps into nested subroutines or cognitive kernel invocations.
- `stepBack` (**Step Back / Reverse Debugging**):
  - Uses the `ClDebugger` cyclic state ring-buffer.
  - Restores the exact register state, program counter, and optical wave buffers of cycle $t-1$.
- `continue`: Resumes non-blocking execution until the next breakpoint, trap, or halt.
- `pause`: Asynchronously interrupts active execution and inspects the immediate core state.

### 3.3 Scopes & Variable Tree
The variables panel organizes machine state into 4 hierarchical scopes:

```
Variables
├── [Scope 1: Hardware Registers]
│   ├── R0: 0x000000000000002A (42)
│   ├── R1: 0x0000000000000001 (1)
│   ├── ...
│   ├── PC: 3 (Cycle: 4)
│   └── Status: RUNNING
├── [Scope 2: Photonic Waves]
│   ├── W0: Phase = 0.000 rad | Magnitude = 1.000 | Wavelength = 1550 nm
│   └── W1: Phase = 1.571 rad | Magnitude = 0.707 | Wavelength = 1310 nm
├── [Scope 3: Neuromorphic Synapses]
│   ├── Synapse_0_to_1: Weight = 0.854 | Trace = 0.120
│   └── Membrane_V: -65.2 mV (Threshold: -50.0 mV)
└── [Scope 4: Silicon Telemetry]
    ├── Core_Temp: 34.2 °C (Throttling: FALSE)
    ├── Frequency: 2400 MHz
    ├── Bank_Conflicts: 0
    └── Landauer_Entropy_Dissipation: 3.675e-19 J
```

---

## 4. VS Code Extension (v1.3.0) Integration

The official VS Code extension (`cron-lang-1.3.0.vsix`) provides out-of-the-box integration for both syntax highlighting, LSP 3.17 diagnostics, and DAP debugging.

### 4.1 Installing the Extension

```bash
code --install-extension vscode-cron/cron-lang-1.3.0.vsix
```

### 4.2 Debug Configurations (`.vscode/launch.json`)

To debug a `.cl` microcode kernel or `.cr` cognitive script, create a `.vscode/launch.json` file in your workspace:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "CRON: Debug Current Microcode (.cl)",
            "type": "cron",
            "request": "launch",
            "program": "${file}",
            "stopOnEntry": true,
            "trace": true
        },
        {
            "name": "CRON: Simulate 256-Core Torus (.cr)",
            "type": "cron",
            "request": "launch",
            "program": "${file}",
            "stopOnEntry": false,
            "trace": false
        }
    ]
}
```

Press **F5** in VS Code to launch the debugger, set breakpoints, step through instructions, and inspect live memory.

---

## 5. Interactive 4D-Torus Mesh Visualizer

CRON v1.3.0 introduces an interactive **4D-Torus Silicon Mesh Visualizer** embedded directly in VS Code.

### 5.1 Opening the Visualizer

1. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS) in VS Code.
2. Type **`CRON: Open 4D-Torus Mesh Visualizer`** and press **Enter**.
3. A live interactive panel opens displaying the wafer matrix.

### 5.2 Visualizer Capabilities

| Feature | Visual Presentation | Description |
| :--- | :--- | :--- |
| **16x16 Wafer Matrix** | 256-cell active grid | Represents physical compute tiles ($4 \times 4 \times 4 \times 4$ Torus topology) |
| **Thermal Heatmap** | Color gradient (Blue -> Green -> Orange -> Red) | Ambient $25^\circ\text{C}$ (Blue) up to Critical $105^\circ\text{C}+$ (Crimson Red) |
| **Hotspot Injection** | `[Inject Hotspot (125°C)]` | Simulates a thermal runaway event and triggers live 2D Fourier heat diffusion |
| **Packet Traffic Animation** | Glowing route indicators | Animates Dimension-Order Routing (DOR) between source and destination tiles |
| **Virtual Channels (VC0..VC3)** | Multi-track buffers | Visualizes deadlock-free routing across `VC0_Direct`, `VC1_Wraparound`, `VC2_Detour`, `VC3_Priority` |
| **Resilient Deflection** | Curved bypass routes | Demonstrates how the NoC router deflects packets around throttled tiles ($T \ge 105^\circ\text{C}$) without dropping data |

---

## 6. Real-World Debugging Workflow Example

1. **Open Kernel**: Open `libcl/flash_attn_v3.cl` in VS Code.
2. **Set Breakpoint**: Click next to line 8 (`B0001: _SM04$030s ...`) to set a red breakpoint dot.
3. **Add Condition**: Right-click the breakpoint -> **Edit Breakpoint...** -> Enter `R0 == 42`.
4. **Launch**: Press **F5**. The simulator runs to cycle 2 and halts.
5. **Inspect State**: In the left pane, expand **Hardware Registers** to view `R0` through `R15`.
6. **Step Backward**: Click the **Step Back** icon on the debug toolbar to rewind to cycle 1 and inspect register differences.
7. **Monitor Thermals**: Open the **Mesh Visualizer** tab alongside the editor to view the real-time core temperature during execution.
