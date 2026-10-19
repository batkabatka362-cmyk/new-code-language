# CRON Hardware Architecture: GDSII Silicon Floorplan & Layer Stacking Specification

**Target Process**: TSMC N5 (5nm FinFET / 3D Heterogeneous Hybrid Bonding)  
**Chip Dimension**: $14.8\text{ mm} \times 14.8\text{ mm}$ ($219.04\text{ mm}^2$)  
**Clock Target**: 2.0 GHz Resonant H-Tree Clock  
**Thermal Design Power (TDP)**: 45W – 120W (Adaptive Sentry Throttle)  
**Topology**: 256 Heterogeneous Cognitive Cores in a 4D-Torus Mesh ($4 \times 4 \times 4 \times 4$)

---

## 1. 3D Metal & Silicon Photonic Layer Stack (BEOL)

The processor integrates a 15-metal electrical layer stack directly bonded atop an Electronic-Photonic Heterogeneous Substrate (EPDA / PIC Layer):

```
┌────────────────────────────────────────────────────────────────────────┐
│ M13 - M14: Ultra-Thick Metal (Global Power Grid VDD/VSS, C4 Bumps)     │ ◄── 0.75V VDD
├────────────────────────────────────────────────────────────────────────┤
│ M11 - M12: Thick Metal (Resonant H-Tree Clock 2.0 GHz, Global Reset)   │ ◄── Clock Tree
├────────────────────────────────────────────────────────────────────────┤
│ M9  - M10: Semi-Global Metal (4D-Torus NoC & Wormhole Bypass Tunnels)  │ ◄── 8-Port Mesh
├────────────────────────────────────────────────────────────────────────┤
│ M7  - M8 : Intermediate Metal (176-bit Cross-Brain Interconnect Bus)   │ ◄── Brain Bus
├────────────────────────────────────────────────────────────────────────┤
│ M3  - M6 : Intermediate Metal (VLIW Pipeline, Register Banks, SRAM)    │ ◄── Core Logic
├────────────────────────────────────────────────────────────────────────┤
│ M0  - M2 : Local Interconnect (Transistor Gates, Standard Cell Wiring) │ ◄── 5nm FinFET
├────────────────────────────────────────────────────────────────────────┤
│ PIC Layer: Integrated Silicon Photonics (MZI Phase Mesh, 1550nm Wave)  │ ◄── Light Guide
└────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Core Floorplan Partitioning

Each of the 256 cores occupies approximately $0.72\text{ mm}^2$, subdivided into dedicated cognitive co-processor domains:

```
┌────────────────────────────────────────┬────────────────────────────────────────┐
│            BRAIN 1 & 6 DOMAIN          │           BRAIN 2 & OPTICAL DOMAIN     │
│  - Knowledge Graph Unification Unit    │  - Photonic Waveguide Phase Modulators │
│  - Hyper-Edge Associative Memory       │  - BitNet 1.58b Ternary Matrix Multiply│
│  - Sentry Sentinel & Thermal Telemetry │  - Optical ADC/DAC Wave Integrators    │
├────────────────────────────────────────┼────────────────────────────────────────┤
│            BRAIN 4 & 5 DOMAIN          │           BRAIN 3 & MEMORY DOMAIN      │
│  - STDP Plasticity Synaptic Memory     │  - Reversible Thermodynamic Push/Pop   │
│  - Leaky Integrate-and-Fire (LIF) SNN  │  - Fredkin / Toffoli Entropy-Free Gates│
│  - Chaos Diffusion / Lorenz Generator  │  - 16-Entry Hardware Shadow Bank       │
├────────────────────────────────────────┴────────────────────────────────────────┤
│                       4D-TORUS ROUTER & CROSSBAR HUB                            │
│  - 8-Way Bi-directional NoC Ports (+X, -X, +Y, -Y, +Z, -Z, +W, -W)              │
│  - Virtual Channel Credit Flow Allocator & Deflection Fallback Sentry           │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Power, Performance & Area (PPA) Summary

| Cognitive Subsystem | Area Per Core | Active Power @ 2 GHz | Peak Throughput |
|:---|:---:|:---:|:---|
| **Photonic GEMM (Brain 2)** | $0.21\text{ mm}^2$ | 95 mW | 16 TOPs/s (Ternary 1.58b) |
| **STDP Plasticity SNN (Brain 4)** | $0.14\text{ mm}^2$ | 42 mW | 128M Synaptic Updates/s |
| **Reversible Stack (Brain 3)** | $0.09\text{ mm}^2$ | 18 mW | 0 Landauer Dissipation |
| **Chaos Attractor (Brain 5)** | $0.08\text{ mm}^2$ | 24 mW | 4G Coordinate Steps/s |
| **Symbolic Graph (Brain 1)** | $0.07\text{ mm}^2$ | 31 mW | 2G Unifications/s |
| **VLIW 4-Slot Engine + Shadow** | $0.08\text{ mm}^2$ | 40 mW | 8 Giga-Instructions/s |
| **4D-Torus NoC Router** | $0.05\text{ mm}^2$ | 35 mW | 512 Gbps Bisection Bandwidth |
| **Total Per Core** | **$0.72\text{ mm}^2$** | **285 mW** | — |
| **Chip Total (256 Cores)** | **$184.3\text{ mm}^2$** | **72.9W** (Nominal) | **4.096 Peta-Ops / s** |
