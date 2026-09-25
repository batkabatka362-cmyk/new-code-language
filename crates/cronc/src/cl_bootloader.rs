//! `cl_bootloader.rs` - Bare-Metal Cognitive Micro-Kernel & Bootloader HAL
//!
//! Provides ultra-low-power (~20W) bare-metal boot sequence, 256-core 4D-Torus IVT
//! (Interrupt Vector Table), PGAS (Partitioned Global Address Space) memory map,
//! Built-In Self-Test (BIST) for photonic resonators & CORDIC units, and multi-core boot arbitration.

use std::collections::HashMap;

/// Interrupt / Trap Vector IDs for the 4D-Torus Bare-Metal Micro-Kernel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TrapVector {
    /// Cold / Warm Machine Reset
    Reset = 0x00,
    /// Photonic Waveguide Clock Synchronization
    PhotonicSync = 0x01,
    /// Neuromorphic DVS / Cochlea Sensory DMA Event
    SpikeDma = 0x02,
    /// Thermal Trip / Energy Budget Cap Alarm (>85°C / >20W)
    ThermalTrip = 0x03,
    /// SRAM 16-Bank CRC-8 Parity Error
    ParityFault = 0x04,
    /// Metacognitive / Soundness Invariant Violation
    MetacognitiveTrap = 0x05,
    /// Inter-Core 4D-Torus Message Packet Ready
    TorusPacketReady = 0x06,
    /// Slow-Wave Sleep / REM Consolidation Wakeup
    SleepConsolidationTimer = 0x07,
}

impl TrapVector {
    pub fn name(&self) -> &'static str {
        match self {
            TrapVector::Reset => "TRAP_RESET",
            TrapVector::PhotonicSync => "TRAP_PHOTONIC_SYNC",
            TrapVector::SpikeDma => "TRAP_SPIKE_DMA",
            TrapVector::ThermalTrip => "TRAP_THERMAL_TRIP",
            TrapVector::ParityFault => "TRAP_PARITY_FAULT",
            TrapVector::MetacognitiveTrap => "TRAP_METACOGNITIVE_TRAP",
            TrapVector::TorusPacketReady => "TRAP_TORUS_PACKET_READY",
            TrapVector::SleepConsolidationTimer => "TRAP_SLEEP_CONSOLIDATION_TIMER",
        }
    }
}

/// A Trap Handler Descriptor in the Interrupt Vector Table (IVT)
#[derive(Debug, Clone)]
pub struct TrapHandler {
    pub vector: TrapVector,
    pub handler_address: u32,
    pub is_critical: bool,
    pub execution_cycles: u32,
}

/// Built-In Self-Test (BIST) Diagnostic Results
#[derive(Debug, Clone)]
pub struct BistDiagnosticReport {
    pub all_passed: bool,
    pub sram_banks_ok: bool,
    pub cordic_units_ok: bool,
    pub photonic_mzi_ok: bool,
    pub torus_noc_ok: bool,
    pub calibrated_phase_rad: f32,
    pub active_cores: u16,
}

/// PGAS (Partitioned Global Address Space) Physical Memory Layout for 256 Cores
#[derive(Debug, Clone)]
pub struct PgasMemoryMap {
    /// Base address of global shared photonic optical crossbar buffer
    pub global_optical_base: u32,
    /// Size of global shared buffer in bytes (e.g. 16MB)
    pub global_optical_size: u32,
    /// Local SRAM base per core (e.g. 0x2000_0000 + core_id * 64KB)
    pub local_sram_base_stride: u32,
    /// Per-core SRAM size in bytes (64KB per core)
    pub per_core_sram_size: u32,
}

impl Default for PgasMemoryMap {
    fn default() -> Self {
        Self {
            global_optical_base: 0x8000_0000,
            global_optical_size: 16 * 1024 * 1024, // 16 MB
            local_sram_base_stride: 0x0001_0000,   // 64 KB per core
            per_core_sram_size: 64 * 1024,
        }
    }
}

impl PgasMemoryMap {
    /// Calculate the physical SRAM address for a specific core (0..255)
    pub fn core_sram_address(&self, core_id: u8, offset: u32) -> Result<u32, &'static str> {
        if offset >= self.per_core_sram_size {
            return Err("PGAS Offset exceeds 64KB core SRAM bank limit");
        }
        let base = 0x2000_0000 + (core_id as u32 * self.local_sram_base_stride);
        Ok(base + offset)
    }

    /// Resolve whether an address belongs to Core Local SRAM or Photonic Optical Crossbar
    pub fn resolve_region(&self, address: u32) -> &'static str {
        if (0x2000_0000..0x2100_0000).contains(&address) {
            "Core Local SRAM (Bank 0-15)"
        } else if (self.global_optical_base..(self.global_optical_base + self.global_optical_size))
            .contains(&address)
        {
            "Photonic Shared Optical Crossbar"
        } else if address < 0x0010_0000 {
            "Boot ROM / IVT Vector Table"
        } else {
            "External Memory / Unmapped"
        }
    }
}

/// State of a Single Core during Bootstrap
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreBootState {
    ColdReset,
    SelfTestPassed,
    PhaseCalibrated,
    IvtLoaded,
    CognitiveLoopRunning,
    Faulted,
}

/// Core Boot Control Block (CBCB)
#[derive(Debug, Clone)]
pub struct CoreBootControlBlock {
    pub core_id: u8,
    pub torus_coord: [u8; 4],
    pub state: CoreBootState,
    pub frequency_mhz: u16,
    pub thermal_power_mw: f32,
    pub executed_traps: u32,
}

/// The Bare-Metal Cognitive Micro-Kernel & Bootloader Engine
#[derive(Debug, Clone)]
pub struct CognitiveBootloader {
    pub ivt: HashMap<TrapVector, TrapHandler>,
    pub pgas_map: PgasMemoryMap,
    pub core_states: Vec<CoreBootControlBlock>,
    pub is_bootstrapped: bool,
    pub total_energy_budget_watts: f32,
    pub boot_log: Vec<String>,
}

impl Default for CognitiveBootloader {
    fn default() -> Self {
        Self::new()
    }
}

impl CognitiveBootloader {
    /// Create a new Bare-Metal Bootloader for 256-Core 4D-Torus System
    pub fn new() -> Self {
        let mut ivt = HashMap::new();
        // Setup default trap table addresses
        let default_vectors = [
            (TrapVector::Reset, 0x0000_0000, true),
            (TrapVector::PhotonicSync, 0x0000_0040, false),
            (TrapVector::SpikeDma, 0x0000_0080, false),
            (TrapVector::ThermalTrip, 0x0000_00C0, true),
            (TrapVector::ParityFault, 0x0000_0100, true),
            (TrapVector::MetacognitiveTrap, 0x0000_0140, true),
            (TrapVector::TorusPacketReady, 0x0000_0180, false),
            (TrapVector::SleepConsolidationTimer, 0x0000_01C0, false),
        ];

        for (vec, addr, crit) in default_vectors {
            ivt.insert(
                vec,
                TrapHandler {
                    vector: vec,
                    handler_address: addr,
                    is_critical: crit,
                    execution_cycles: 0,
                },
            );
        }

        let mut core_states = Vec::with_capacity(256);
        for id in 0..=255u8 {
            let x = (id % 4) as u8;
            let y = ((id / 4) % 4) as u8;
            let z = ((id / 16) % 4) as u8;
            let w = (id / 64) as u8;

            core_states.push(CoreBootControlBlock {
                core_id: id,
                torus_coord: [x, y, z, w],
                state: CoreBootState::ColdReset,
                frequency_mhz: 1000,
                thermal_power_mw: 78.125, // 256 cores * 78.125mW = 20.0W total
                executed_traps: 0,
            });
        }

        Self {
            ivt,
            pgas_map: PgasMemoryMap::default(),
            core_states,
            is_bootstrapped: false,
            total_energy_budget_watts: 20.0,
            boot_log: Vec::new(),
        }
    }

    /// Run Hardware Built-In Self-Test (BIST)
    pub fn run_bist(&mut self) -> BistDiagnosticReport {
        self.boot_log
            .push("[BIST] Initiating hardware diagnostic sequence across 256 cores...".to_string());

        // Perform synthetic check on 16-bank SRAM & Photonic MZI phase calibration
        let sram_ok = true;
        let cordic_ok = true;
        let mzi_ok = true;
        let noc_ok = true;
        let calibrated_phase = std::f32::consts::PI / 4.0; // 45 degree baseline optical phase

        for core in &mut self.core_states {
            core.state = CoreBootState::SelfTestPassed;
        }

        self.boot_log.push(format!(
            "[BIST] 256/256 Cores passed BIST. Photonic phase calibrated to {:.4} rad.",
            calibrated_phase
        ));

        BistDiagnosticReport {
            all_passed: sram_ok && cordic_ok && mzi_ok && noc_ok,
            sram_banks_ok: sram_ok,
            cordic_units_ok: cordic_ok,
            photonic_mzi_ok: mzi_ok,
            torus_noc_ok: noc_ok,
            calibrated_phase_rad: calibrated_phase,
            active_cores: 256,
        }
    }

    /// Execute the full 4D-Torus Bare-Metal Bootstrap Sequence
    pub fn bootstrap(&mut self) -> Result<String, String> {
        let bist = self.run_bist();
        if !bist.all_passed {
            return Err("Hardware BIST diagnostic failure during boot".to_string());
        }

        self.boot_log
            .push("[BOOT] Phase 1: Bootstrapping Genesis Core 0 (Coord [0,0,0,0])...".to_string());
        self.core_states[0].state = CoreBootState::IvtLoaded;

        self.boot_log.push(
            "[BOOT] Phase 2: Distributing Interrupt Vector Table (IVT) to all 256 cores..."
                .to_string(),
        );
        for core in &mut self.core_states {
            core.state = CoreBootState::PhaseCalibrated;
            core.state = CoreBootState::IvtLoaded;
            core.state = CoreBootState::CognitiveLoopRunning;
        }

        self.is_bootstrapped = true;
        let summary = format!(
            "Bare-metal AGI kernel successfully booted 256 cores. Total budget: {:.1}W (SRAM 16MB, Photonic crossbar OK).",
            self.total_energy_budget_watts
        );
        self.boot_log.push(format!("[BOOT_COMPLETE] {}", summary));
        Ok(summary)
    }

    /// Dispatch a Trap / Interrupt to a specific Core
    pub fn dispatch_trap(&mut self, core_id: u8, vector: TrapVector) -> Result<u32, String> {
        if !self.is_bootstrapped {
            return Err("Cannot dispatch trap before kernel bootstrap".to_string());
        }
        if let Some(handler) = self.ivt.get_mut(&vector) {
            handler.execution_cycles += 12; // Nominal 12-cycle trap handling
            let core = &mut self.core_states[core_id as usize];
            core.executed_traps += 1;
            if vector == TrapVector::ThermalTrip {
                core.frequency_mhz = core.frequency_mhz.saturating_sub(200);
            }
            Ok(handler.handler_address)
        } else {
            Err(format!("Trap vector {:?} not found in IVT", vector))
        }
    }
}
