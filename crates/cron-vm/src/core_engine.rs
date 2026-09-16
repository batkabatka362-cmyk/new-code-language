// ============================================================================
// CRON Core Engine — Single Core Execution Engine for 4D Torus Mesh
// Simulates 128-bit VLIW instructions: Photonic, Reversible, STDP, Symbolic
// Target: 256-Core 4D-Torus Neuromorphic Photonic Hardware
// ============================================================================

#[derive(Debug, Clone)]
pub struct PhotonicWave {
    pub amplitudes: [u8; 4],
    pub phases: [u8; 4],
}

#[derive(Debug, Clone)]
pub struct CoreEngine {
    pub id: usize,
    pub registers: [u32; 16],
    pub wave_reg: PhotonicWave,
    pub reversible_stack: Vec<u32>,
    pub stdp_weights: [i8; 16],
    pub thermal_level: u32,
    pub thermal_threshold: u32,
    pub fiber1_active: bool,
    pub is_halted: bool,
    pub trace_enabled: bool,

    // Performance telemetry counters
    pub cycle_count: usize,
    pub optical_gemm_count: usize,
    pub reversible_ops_count: usize,
    pub stdp_updates_count: usize,
    pub spatial_broadcast_count: usize,

    // === Hardware Trap & Fault Handling (Milestone #000, #181) ===
    /// Machine Cause Register: identifies the type of hardware exception
    pub mcause: u32,
    /// Machine Exception Program Counter: faulting cycle address
    pub mepc: usize,
    /// Trap handler ISR vector address (fixed at 0x0004)
    pub trap_handler_addr: usize,
    /// Whether the core is currently servicing a trap
    pub in_trap: bool,
    /// Total trap count for telemetry
    pub trap_count: usize,

    // === 32-bit Galois LFSR PRNG (Milestone #042) ===
    /// Hardware LFSR state: polynomial x^32 + x^31 + x^29 + x + 1
    pub lfsr_state: u32,

    // === Non-Blocking NoC Poll (Milestone #042) ===
    /// Network-on-Chip receive FIFO queue
    pub noc_rx_fifo: Vec<u32>,

    // === Hardware Performance CSRs (Milestone #180) ===
    /// CSR 0: Total elapsed cycles
    pub csr_cycle_cnt: u32,
    /// CSR 1: Memory bank / pipeline stall cycles
    pub csr_stall_cnt: u32,
    /// CSR 2: Executed predicated operations count
    pub csr_pred_exec_cnt: u32,
    /// CSR 3: Bundles with 100% 4-slot saturation
    pub csr_vec_burst_cnt: u32,

    // === Stackless ABI Shadow Checkpoint Bank (Milestone #053) ===
    /// 16-entry shadow register checkpoint bank for 1-cycle save/restore
    pub shadow_bank: Vec<[u32; 16]>,

    // === Banked Registers & 4D Hyperspace Clusters ===
    /// 16 Bank Register Files (16 banks x 16 registers each)
    /// Bank 0: Local Core Registers (mirrored with registers)
    /// Banks 1..15: 4D-Torus Hyperspace Clusters ($W$-axis scatter/gather)
    pub bank_registers: [[u32; 16]; 16],
    /// Chip-wide global barrier synchronization state
    pub in_barrier: bool,
    pub barrier_count: usize,

    // === Extended Homopolymer & Twin-Token Macro Telemetry ===
    pub cache_invalidations: usize, // CC: Chip-wide I/D Cache Invalidation
    pub dma_transfers: usize,       // DD: Direct NoC DMA transfer burst
    pub dvfs_energy_state: u32,     // EE: DVFS Power State (0=Nominal, 1=Eco, 2=Turbo)
    pub energy_saved_uw: u64,       // EE: Energy Saved in micro-Watts
    pub photonic_pumps: usize,      // 11: Photonic Laser Pump Strobe
    pub arena_resets: usize,        // 88: Region Arena 0-cycle Reset
    pub sentry_trips: usize,        // 99: Global Hardware Sentry Watchdog Trip
}

impl CoreEngine {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            registers: [0; 16],
            wave_reg: PhotonicWave {
                amplitudes: [0; 4],
                phases: [0; 4],
            },
            reversible_stack: Vec::new(),
            stdp_weights: [10; 16], // initial synaptic weights
            thermal_level: 25,      // 25°C baseline
            thermal_threshold: 180, // Sentry threshold
            fiber1_active: false,
            is_halted: false,
            trace_enabled: false,
            cycle_count: 0,
            optical_gemm_count: 0,
            reversible_ops_count: 0,
            stdp_updates_count: 0,
            spatial_broadcast_count: 0,
            // Hardware Trap state
            mcause: 0,
            mepc: 0,
            trap_handler_addr: 0x0004,
            in_trap: false,
            trap_count: 0,
            // Galois LFSR PRNG (non-zero seed)
            lfsr_state: 0xACE1_u32,
            // NoC receive FIFO
            noc_rx_fifo: Vec::new(),
            // Hardware CSR Performance Counters
            csr_cycle_cnt: 0,
            csr_stall_cnt: 0,
            csr_pred_exec_cnt: 0,
            csr_vec_burst_cnt: 0,
            // Shadow checkpoint bank
            shadow_bank: Vec::new(),
            // Banked Registers & 4D Hyperspace Clusters
            bank_registers: [[0; 16]; 16],
            in_barrier: false,
            barrier_count: 0,
            // Extended Homopolymer Telemetry
            cache_invalidations: 0,
            dma_transfers: 0,
            dvfs_energy_state: 0,
            energy_saved_uw: 0,
            photonic_pumps: 0,
            arena_resets: 0,
            sentry_trips: 0,
        }
    }

    fn parse_slot_metadata(&self, slot: &str) -> (String, usize, usize, usize, usize, char) {
        if slot.len() < 3 {
            return (String::new(), 0, 0, 0, 0, '$');
        }

        let op = slot[1..3].to_string();

        // Parse destination bank (high nibble) and register (low nibble)
        // If immediate load with 1-char opcode (e.g. '=00#0A04>, '=06#0064>, index 4 is '#'):
        //   bank is index 2, reg is index 3
        // If standard 2-char opcode (e.g. '==04#000A>, _PO06G400>, _PO0A$000>, _POA0$000>):
        //   bank is index 3, reg is index 4
        let (dest_bank, dest_reg) = if slot.len() >= 5 && slot.chars().nth(4) == Some('#') {
            let h = slot[2..3].chars().next().and_then(|c| c.to_digit(16)).unwrap_or(0) as usize;
            let l = slot[3..4].chars().next().and_then(|c| c.to_digit(16)).unwrap_or(0) as usize;
            (h, l)
        } else if slot.len() >= 5 {
            let h = slot[3..4].chars().next().and_then(|c| c.to_digit(16)).unwrap_or(0) as usize;
            let l = slot[4..5].chars().next().and_then(|c| c.to_digit(16)).unwrap_or(0) as usize;
            (h, l)
        } else {
            (0, 0)
        };

        // Parse mode (index 5)
        let mode = slot.chars().nth(5).unwrap_or('$');

        // Parse source register (index 6)
        let src = if slot.len() >= 7 {
            slot[6..7]
                .chars()
                .next()
                .and_then(|c| c.to_digit(16))
                .unwrap_or(0) as usize
        } else {
            0
        };

        // Parse immediate parameter (index 8)
        let imm = if slot.len() >= 9 {
            slot[8..9]
                .chars()
                .next()
                .and_then(|c| c.to_digit(16))
                .unwrap_or(0) as usize
        } else {
            0
        };

        (op, dest_bank, dest_reg, src, imm, mode)
    }

    pub fn execute_slot(&mut self, slot: &str) -> Option<u32> {
        if self.is_halted || slot.len() < 3 {
            return None;
        }

        let (op, dest_bank, dest_reg, src, imm, mode) = self.parse_slot_metadata(slot);

        // Check for Halt
        if op == "HL" || slot.starts_with("_HL") || slot.starts_with("_HLT") {
            self.is_halted = true;
            return None;
        }

        // Check for Global 256-Core Chip-Wide Hardware Barrier (bb) (Pages 88-95)
        if op == "bb" || (slot.len() >= 3 && &slot[1..3] == "bb") || (slot.len() >= 5 && &slot[3..5] == "bb") {
            if !self.in_barrier {
                self.in_barrier = true;
                self.barrier_count += 1;
            }
        }

        let dest = dest_reg.min(15);
        let saved_local_reg = self.registers[dest];
        if dest_bank > 0 {
            // Load current banked value into execution slot
            self.registers[dest] = self.bank_registers[dest_bank.min(15)][dest];
        }

        match op.as_str() {
            op if op.starts_with('=') || slot.starts_with("'=") => {
                // Immediate load: e.g. '=00#0A04>, '=06#0064>, or '==04#000A>
                let reg_idx = dest;
                if reg_idx < 16 {
                    if let Some(hash_pos) = slot.find('#') {
                        let hex_part: String = slot[hash_pos + 1..]
                            .chars()
                            .take_while(|c| c.is_ascii_hexdigit())
                            .collect();
                        if let Ok(val) = u32::from_str_radix(&hex_part, 16) {
                            self.registers[reg_idx] = val;
                        } else {
                            self.registers[reg_idx] = imm as u32;
                        }
                    } else {
                        let imm_str: String = slot.chars().skip(3).filter(|c| c.is_ascii_hexdigit()).collect();
                        if let Ok(val) = u32::from_str_radix(&imm_str, 16) {
                            self.registers[reg_idx] = val;
                        } else {
                            self.registers[reg_idx] = imm as u32;
                        }
                    }
                }
            }
            "SH" => {
                // Self-healing sentry configuration (Brain 6)
                self.thermal_threshold = 180;
            }
            "SP" => {
                // Spawn Fiber 1
                self.fiber1_active = true;
            }
            "FJ" => {
                // Fiber 1 Join
                self.fiber1_active = false;
            }
            "PK" => {
                // Pack sub-byte ternary representation
                let d = if dest > 0 { dest } else { 2 };
                self.registers[d] = 0x5555_AAAA;
            }
            "TL" => {
                // 4D torus tile calculation
                let d = if dest > 0 { dest } else { 3 };
                self.registers[d] = self.id as u32;
            }
            "ST" => {
                // STDP synapse update or prefetch stage (Brain 4)
                self.stdp_updates_count += 1;
                for w in self.stdp_weights.iter_mut() {
                    *w = (*w + 2).min(127);
                }
                let d = if dest > 0 { dest } else { 12 };
                self.registers[d] = 0x0000_0084;
            }
            "YD" => {
                // Coroutine Fiber Yield / Spatial Gather
            }
            "OP" | "WD" => {
                // Photonic MZI Optical GEMM (Brain 2)
                self.optical_gemm_count += 1;
                let d = if dest > 0 { dest } else { 4 };
                self.registers[d] = 0x00FF_AA55;
            }
            "FA" => {
                // Forward Autodiff Tap into reversible stack (Brain 2)
                let s = if src > 0 { src } else { 4 };
                let val = self.registers[s];
                self.reversible_stack.push(val);
                let d = if dest > 0 { dest } else { 5 };
                self.registers[d] = val;
            }
            "PO" | "P0" | "P1" => {
                // Predicated SIMD ALU (Pages 88-95: Polymorphic Mode Delimiters)
                let d = if dest > 0 { dest } else { 6 };
                let s = if src > 0 { src } else { 4 };
                let val_d = self.registers[d];
                let val_s = self.registers[s];

                self.registers[d] = match mode {
                    '+' => val_d.wrapping_add(val_s),
                    '-' => val_d.wrapping_sub(val_s),
                    '*' => val_d.wrapping_mul(val_s),
                    '/' => {
                        if val_s != 0 {
                            val_d / val_s
                        } else {
                            // Hardware Trap: Division by Zero (Milestone #181)
                            self.trigger_trap(0x0001);
                            0
                        }
                    }
                    '%' => {
                        if val_s != 0 {
                            val_d % val_s
                        } else {
                            self.trigger_trap(0x0001);
                            0
                        }
                    }
                    '&' => val_d & val_s,
                    '|' => val_d | val_s,
                    '^' => val_d ^ val_s,
                    '~' => !val_s,
                    '=' => if val_d == val_s { 1 } else { 0 },
                    '<' => if val_d < val_s { 1 } else { 0 },
                    '>' => if val_d > val_s { 1 } else { 0 },
                    '?' => if val_s != 0 { val_d } else { 0 },
                    '!' => {
                        if val_s != 0 {
                            self.trigger_trap(0x0002);
                        }
                        val_d
                    }
                    'G' => if val_d >= val_s { 1 } else { 0 },
                    _ => match imm {
                        1 => val_d.wrapping_add(val_s),
                        2 => val_d.wrapping_sub(val_s),
                        3 => val_d.wrapping_mul(val_s),
                        4 => {
                            if val_s != 0 {
                                val_d / val_s
                            } else {
                                // Hardware Trap: Division by Zero (Milestone #181)
                                self.trigger_trap(0x0001);
                                0
                            }
                        }
                        5 => {
                            if val_s != 0 {
                                val_d % val_s
                            } else {
                                self.trigger_trap(0x0001);
                                0
                            }
                        }
                        6 => val_d & val_s,
                        7 => val_d | val_s,
                        8 => val_d ^ val_s,
                        9 => val_d << (val_s & 31),
                        0xA => val_d >> (val_s & 31),
                        0xB => if val_d == val_s { 1 } else { 0 },
                        0xC => if val_d != val_s { 1 } else { 0 },
                        0xD => if val_d < val_s { 1 } else { 0 },
                        0xE => if val_d <= val_s { 1 } else { 0 },
                        0xF => if val_d > val_s { 1 } else { 0 },
                        0 => if val_s > 0 && d != s { val_s } else { val_d & 0x0000_0003 },
                        _ => val_d & 0x0000_0003,
                    },
                };
                self.csr_pred_exec_cnt += 1;
            }
            "MD" => {
                // 16x 2-bit ternary Dot Product MAC / Multiply (Pages 88-95)
                let d = if dest > 0 { dest } else { 8 };
                let s = if src > 0 { src } else { 0 };
                if mode == '*' || imm == 3 {
                    self.registers[d] = self.registers[d].wrapping_mul(self.registers[s]);
                } else if mode == '/' || imm == 4 {
                    if self.registers[s] != 0 {
                        self.registers[d] /= self.registers[s];
                    } else {
                        // Hardware Trap: Division by Zero (Milestone #181)
                        self.trigger_trap(0x0001);
                    }
                } else {
                    self.registers[d] = 0x0012_3456;
                }
            }
            "BK" => {
                // Reversible Backward Invert (Brain 3)
                self.reversible_ops_count += 1;
                let val = self.reversible_stack.pop().unwrap_or(0);
                let d = if dest > 0 { dest } else { 9 };
                self.registers[d] = val ^ 0xFFFF_FFFF; // Inverted mapping
            }
            "BL" => {
                // Blend staged halo
                let d = if dest > 0 { dest } else { 10 };
                let s = if src > 0 { src } else { 3 };
                self.registers[d] = self.registers[s] | 0x8000;
            }
            "RF" => {
                // Reversible Fredkin Gate Swap (Brain 3)
                self.reversible_ops_count += 1;
                let s1 = if dest > 0 { dest } else { 10 };
                let s2 = if src > 0 { src } else { (s1 + 1).min(15) };
                let tmp = self.registers[s1];
                self.registers[s1] = self.registers[s2];
                self.registers[s2] = tmp;
            }
            "TO" => {
                // Toffoli 3-wire Reversible Gate (Brain 3)
                self.reversible_ops_count += 1;
                let c1 = (self.registers[10] & 1) != 0;
                let c2 = (self.registers[11] & 1) != 0;
                let d = if dest > 0 { dest } else { 9 };
                if c1 && c2 {
                    self.registers[d] ^= 1;
                }
            }
            "GU" => {
                // Gradient Step Update: W = W - eta * grad
                let d = if dest > 0 { dest } else { 4 };
                let s = if src > 0 { src } else { 9 };
                self.registers[d] = self.registers[d].wrapping_sub(self.registers[s] / 2);
            }
            "SY" => {
                // Symbolic Grounding (Brain 1)
                let d = if dest > 0 { dest } else { 13 };
                self.registers[d] = 0xCAFE_BABE;
            }
            "KG" => {
                // Knowledge Graph Query / Assert (Brain 1)
                let d = if dest > 0 { dest } else { 13 };
                self.registers[d] = 1;
            }
            "PT" => {
                // Parity Telemetry Verification
            }
            "DW" => {
                // DMA Write-back / Scatter to High-Bandwidth Memory (HBM3)
            }
            "RS" => {
                // Region Arena Instant Reset (Brain 6 Memory reclamation in 0 cycles)
                self.registers[1] = 0;
            }
            "SW" => {
                // Superposition Wave Predication
                let s = if src > 0 { src } else { 14 };
                let cond = self.registers[s] != 0;
                if !cond {
                    return None;
                }
            }
            "RC" => {
                // Resilient Compute Fallback / CSR Read / Trap Return
                if imm == 0 && mode == '!' {
                    // Shadow checkpoint bank SAVE: _RC00#000.!>
                    self.shadow_bank.push(self.registers);
                } else if imm == 1 && mode == '!' {
                    // Shadow checkpoint bank RESTORE: _RC00#001.!>
                    if let Some(saved) = self.shadow_bank.pop() {
                        self.registers = saved;
                    }
                } else if mode == 'C' {
                    // CSR Read: _RC<dest><CSR_ID>C...>
                    let d = if dest > 0 { dest } else { 0 };
                    let csr_id = src;
                    self.registers[d] = match csr_id {
                        0 => self.csr_cycle_cnt,
                        1 => self.csr_stall_cnt,
                        2 => self.csr_pred_exec_cnt,
                        3 => self.csr_vec_burst_cnt,
                        _ => 0,
                    };
                }
            }
            "RT" => {
                // Return from trap (MRET): restore PC to mepc + 1 and clear trap state
                self.in_trap = false;
                self.mcause = 0;
            }
            "RN" => {
                // 32-bit Galois LFSR PRNG (Milestone #042)
                // Polynomial: x^32 + x^31 + x^29 + x + 1 = 0xA000_0003
                let feedback = self.lfsr_state & 1;
                self.lfsr_state >>= 1;
                if feedback == 1 {
                    self.lfsr_state ^= 0xA000_0003;
                }
                let d = if dest > 0 { dest } else { 0 };
                self.registers[d] = self.lfsr_state;
            }
            "PL" => {
                // Non-blocking NoC FIFO Poll (Milestone #042)
                // Writes 1 if rx_fifo_not_empty, 0 if empty
                let d = if dest > 0 { dest } else { 0 };
                self.registers[d] = if self.noc_rx_fifo.is_empty() { 0 } else { 1 };
            }
            "SB" => {
                // Spatial Broadcast across 4D Torus mesh
                self.spatial_broadcast_count += 1;
            }
            "HE" => {
                // Brain 1 Hyper-Edge Associator (Opcode 7'd134)
                let d = if dest > 0 { dest } else { 13 };
                let s = if src > 0 { src } else { 0 };
                let mask = 0xCAFE_0000 | ((self.registers[s] & 0xFF) << 8) | (self.registers[d] & 0xFF);
                self.registers[d] = mask;
            }
            "LI" => {
                // Brain 4 Neuromorphic LIF Neuron Step (Opcode 6'd75)
                let d = if dest > 0 { dest } else { 12 };
                let s = if src > 0 { src } else { 0 };
                let current = self.registers[s] & 0xFF;
                let membrane = (self.registers[d] * 7 / 8).wrapping_add(current);
                let thresh = if imm > 0 { (imm as u32) * 10 } else { 50 };
                if membrane >= thresh {
                    self.registers[d] = 1; // Spike emitted
                } else {
                    self.registers[d] = 0; // Sub-threshold
                }
                self.stdp_updates_count += 1;
            }
            "OD" => {
                // Brain 5 Chaos Diffusion / Lorenz Attractor (Opcode 7'd128)
                let d = if dest > 0 { dest } else { 14 };
                let s = if src > 0 { src } else { d };
                let val = self.registers[s];
                self.registers[d] = val.wrapping_mul(11).wrapping_add(7) ^ (val >> 3);
            }
            "CA" => {
                // Brain 5 Cross-Attention Gating (Opcode 7'd132)
                let d = if dest > 0 { dest } else { 6 };
                let s = if src > 0 { src } else { 4 };
                let gate = (self.registers[s] & 0xFF) as u32;
                let act = self.registers[d];
                self.registers[d] = (act.wrapping_mul(gate)) >> 8;
            }
            "AW" => {
                // Brain 6 Dynamic Arbiter Weight Update (Opcode 7'd126)
                let d = if dest > 0 { dest } else { 1 };
                let s = if src > 0 { src } else { 0 };
                let delta = (self.registers[s] & 0xF) as u32;
                self.registers[d] = (self.registers[d] + delta).min(255);
            }
            "CD" => {
                // Hardware CORDIC Sin/Cos (Opcode 6'd63)
                let d = if dest > 0 { dest } else { 5 };
                let s = if src > 0 { src } else { d };
                let angle_deg = (self.registers[s] % 360) as f64;
                let rad = angle_deg.to_radians();
                let sin_fx = ((rad.sin() * 32767.0) as i16 as u16) as u32;
                let cos_fx = ((rad.cos() * 32767.0) as i16 as u16) as u32;
                self.registers[d] = (cos_fx << 16) | sin_fx;
            }
            "CS" => {
                // Hardware Atomic Compare-and-Swap (Opcode 7'd105)
                let d = if dest > 0 { dest } else { 1 };
                let s = if src > 0 { src } else { 0 };
                if self.registers[d] == self.registers[s] {
                    self.registers[d] = imm as u32;
                    self.registers[0] = 1;
                } else {
                    self.registers[0] = 0;
                }
            }
            "PS" => {
                // Parallel Prefix-Sum Kogge-Stone Adder Tree (Opcode 7'd106)
                let d = if dest > 0 { dest } else { 2 };
                let s = if src > 0 { src } else { d };
                let v = self.registers[s];
                let b0 = (v & 0xFF) as u32;
                let b1 = ((v >> 8) & 0xFF) as u32;
                let b2 = ((v >> 16) & 0xFF) as u32;
                let b3 = ((v >> 24) & 0xFF) as u32;
                let s0 = b0 & 0xFF;
                let s1 = (b0 + b1) & 0xFF;
                let s2 = (b0 + b1 + b2) & 0xFF;
                let s3 = (b0 + b1 + b2 + b3) & 0xFF;
                self.registers[d] = s0 | (s1 << 8) | (s2 << 16) | (s3 << 24);
            }
            "TT" => {
                // Tensor Tile Strided Swizzle / Transpose (Opcode 7'd115)
                let d = if dest > 0 { dest } else { 3 };
                let s = if src > 0 { src } else { d };
                let v = self.registers[s];
                let mut res = 0u32;
                for row in 0..4 {
                    for col in 0..4 {
                        let bit = (v >> (row * 8 + col * 2)) & 0x3;
                        res |= bit << (col * 8 + row * 2);
                    }
                }
                self.registers[d] = res;
            }
            "IR" => {
                // In-Network Flight Reduction (Opcode 6'd80)
                let d = if dest > 0 { dest } else { 1 };
                let s = if src > 0 { src } else { 0 };
                self.registers[d] = self.registers[d].wrapping_add(self.registers[s]);
            }
            "WH" => {
                // Deterministic 4D Hyper-Torus Wormhole Tunnel (Opcode 7'd135)
                let d = if dest > 0 { dest } else { 3 };
                let s = if src > 0 { src } else { 0 };
                self.registers[d] = 0x5500_0000 | (self.registers[s] & 0x00FF_FFFF);
            }
            "DF" => {
                // Adaptive Deflection Routing (Opcode 6'd76)
                let d = if dest > 0 { dest } else { 0 };
                self.registers[d] = 1;
            }
            "AC" => {
                // Hardware Capability Token (Opcode 0x14)
                let d = if dest > 0 { dest } else { 1 };
                self.registers[d] = 0xC4F0_0001;
            }
            "SN" => {
                // Hardware Bounds Sanitization (Opcode 0x15)
                let d = if dest > 0 { dest } else { 1 };
                let s = if src > 0 { src } else { 0 };
                self.registers[d] = self.registers[s] & 0x00FF_FFFF;
            }
            "SC" => {
                // Hardware Secure I-Cache Patch (Opcode 0x16)
                let d = if dest > 0 { dest } else { 0 };
                self.registers[d] = 1;
            }
            "LF" => {
                // Brain 4 Neuromorphic LIF Spike Generator (Opcode 6'd74)
                self.stdp_updates_count += 1;
                let d = if dest > 0 { dest } else { 12 };
                let s = if src > 0 { src } else { 0 };
                self.registers[d] = if self.registers[s] > 100 { 1 } else { 0 };
            }
            "TX" => {
                // NoC Channel Send Wormhole Packet Injection (Opcode 6'd78)
                let s = if src > 0 { src } else { dest };
                let _val = self.registers[s];
                self.spatial_broadcast_count += 1;
            }
            "RX" => {
                // Core Mailbox Channel Recv FIFO Pop (Opcode 6'd79)
                let d = if dest > 0 { dest } else { 0 };
                self.registers[d] = self.noc_rx_fifo.pop().unwrap_or(0);
            }
            "CC" => {
                // Chip-Wide 256-Core I/D Cache & Pipeline Invalidation (Homopolymer CC)
                self.cache_invalidations += 1;
                self.csr_stall_cnt = 0;
                self.shadow_bank.clear();
            }
            "DD" => {
                // Zero-Overhead Direct 4D-Torus NoC DMA Transfer (Homopolymer DD)
                self.dma_transfers += 1;
                let s = if src > 0 { src } else { dest };
                let val = self.registers[s];
                if dest_bank > 0 {
                    self.bank_registers[dest_bank.min(15)][dest] = val;
                }
                self.spatial_broadcast_count += 1;
            }
            "EE" => {
                // Energy-Aware Dynamic Voltage and Frequency Scaling (Homopolymer EE)
                self.dvfs_energy_state = 1; // Eco mode
                self.thermal_level = 25;    // Dissipate heat to baseline
                self.energy_saved_uw += 450;
            }
            "BB" => {
                // Brain-Bridge Cross-Neuromorphic Synchronization (Homopolymer BB)
                for i in 0..4 {
                    self.wave_reg.amplitudes[i] = (self.stdp_weights[i] as u32).min(255) as u8;
                }
            }
            "FF" => {
                // Fredkin Reversible Full Fold (Homopolymer FF)
                let mut acc = 0u32;
                while let Some(top) = self.reversible_stack.pop() {
                    acc = acc.wrapping_add(top);
                }
                let d = if dest > 0 { dest } else { 0 };
                self.registers[d] = acc;
                self.reversible_ops_count += 1;
            }
            "11" => {
                // Photonic Laser Pump Strobe (Homopolymer 11)
                self.photonic_pumps += 1;
                self.wave_reg.amplitudes = [255, 255, 255, 255];
            }
            "88" => {
                // Region Arena Instantaneous 0-Cycle Reset (Homopolymer 88)
                self.reversible_stack.clear();
                self.arena_resets += 1;
            }
            "99" => {
                // Global Hardware Sentry Watchdog Trip (Homopolymer 99)
                self.thermal_threshold = 180;
                self.sentry_trips += 1;
            }
            "aa" => {
                // All-to-all NoC Hypercube Scatter (Homopolymer aa)
                self.spatial_broadcast_count += 4;
            }
            "ee" => {
                // Event-driven Neuromorphic Spike Broadcast (Homopolymer ee)
                self.stdp_updates_count += 1;
                for w in self.stdp_weights.iter_mut() {
                    *w = (*w + 1).min(127);
                }
            }
            _ => {}
        }

        if self.trace_enabled {
            println!(
                "[TRACE] Core{} Cycle{}: OP={} R{}={:#010X} (src=R{}, imm={})",
                self.id, self.cycle_count, op, dest, self.registers[dest.min(15)], src, imm
            );
        }

        // Bank isolation & writeback:
        // If dest_bank > 0: write result to bank_registers[dest_bank][dest], and restore local registers[dest]
        // If dest_bank == 0: write result to registers[dest] and mirror to bank_registers[0][dest]
        if dest_bank > 0 {
            let result_val = self.registers[dest];
            self.registers[dest] = saved_local_reg;
            self.bank_registers[dest_bank.min(15)][dest] = result_val;
            self.spatial_broadcast_count += 1;
        } else {
            self.bank_registers[0][dest] = self.registers[dest];
        }

        // Homopolymer AA: Dual-bank SIMD self-broadcast / lockstep auto-accumulation (RA <- RA * RA or self-broadcast)
        // Dest bank 10, Dest reg 10 (or token containing "AA" in position 3..5)
        if (dest_bank == 10 && dest_reg == 10) || (slot.len() >= 5 && &slot[3..5] == "AA") {
            self.csr_vec_burst_cnt += 1;
            let val = if self.registers[10] != 0 {
                self.registers[10]
            } else if saved_local_reg != 0 {
                saved_local_reg
            } else {
                self.bank_registers[10][10]
            };
            let acc = val.wrapping_mul(val);
            self.registers[10] = acc;
            self.bank_registers[0][10] = acc;
            self.bank_registers[10][10] = acc;
        }

        // Slight thermal dissipation check
        self.thermal_level = (self.thermal_level + 1).min(self.thermal_threshold);

        // Increment hardware CSR cycle counter
        self.csr_cycle_cnt += 1;

        // Optional spatial broadcast payload (broadcast dest register if non-zero)
        if op == "SB" || (dest > 0 && (op == "TL" || op == "ST" || op == "OP" || op == "PK")) {
            if dest_bank > 0 {
                Some(self.bank_registers[dest_bank.min(15)][dest])
            } else {
                Some(self.registers[dest])
            }
        } else {
            None
        }
    }

    /// Read from a banked register file
    pub fn get_bank_register(&self, bank: usize, reg: usize) -> u32 {
        self.bank_registers[bank.min(15)][reg.min(15)]
    }

    /// Write to a banked register file (mirrors Bank 0 to local registers)
    pub fn set_bank_register(&mut self, bank: usize, reg: usize, val: u32) {
        let b = bank.min(15);
        let r = reg.min(15);
        self.bank_registers[b][r] = val;
        if b == 0 {
            self.registers[r] = val;
        }
    }

    /// Trigger a hardware trap (Milestone #000, #181)
    /// Sets MCAUSE, saves faulting cycle to MEPC, and vectors to ISR at 0x0004
    pub fn trigger_trap(&mut self, cause: u32) {
        self.mcause = cause;
        self.mepc = self.cycle_count;
        self.in_trap = true;
        self.trap_count += 1;
        if self.trace_enabled {
            println!(
                "[TRAP] Core{}: MCAUSE={:#06X} MEPC={} -> ISR Vector {:#06X}",
                self.id, self.mcause, self.mepc, self.trap_handler_addr
            );
        }
    }

    /// Read a hardware CSR (Milestone #180)
    pub fn read_csr(&self, csr_id: usize) -> u32 {
        match csr_id {
            0 => self.csr_cycle_cnt,
            1 => self.csr_stall_cnt,
            2 => self.csr_pred_exec_cnt,
            3 => self.csr_vec_burst_cnt,
            _ => 0,
        }
    }

    /// Advance Galois LFSR by one step and return the new state
    pub fn advance_lfsr(&mut self) -> u32 {
        let feedback = self.lfsr_state & 1;
        self.lfsr_state >>= 1;
        if feedback == 1 {
            self.lfsr_state ^= 0xA000_0003;
        }
        self.lfsr_state
    }

    /// Push a packet into the NoC receive FIFO
    pub fn noc_push(&mut self, packet: u32) {
        self.noc_rx_fifo.push(packet);
    }

    /// Pop a packet from the NoC receive FIFO (if available)
    pub fn noc_pop(&mut self) -> Option<u32> {
        if self.noc_rx_fifo.is_empty() {
            None
        } else {
            Some(self.noc_rx_fifo.remove(0))
        }
    }

    /// Core dump: print all register values
    pub fn core_dump(&self) {
        println!("=== Core {} Dump ===", self.id);
        for i in 0..16 {
            println!("  R{:02}: {:#010X} ({})", i, self.registers[i], self.registers[i]);
        }
        println!("  MCAUSE: {:#06X}  MEPC: {}  TRAP: {}", self.mcause, self.mepc, self.in_trap);
        println!("  LFSR: {:#010X}", self.lfsr_state);
        println!("  CSR[CYCLE_CNT]={} CSR[STALL_CNT]={} CSR[PRED_EXEC_CNT]={} CSR[VEC_BURST_CNT]={}",
            self.csr_cycle_cnt, self.csr_stall_cnt, self.csr_pred_exec_cnt, self.csr_vec_burst_cnt);
        println!("  Cycles: {}  Traps: {}  Shadow Bank Depth: {}",
            self.cycle_count, self.trap_count, self.shadow_bank.len());
    }
}
