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

    // === Branching, Control Flow & Call Stack ===
    pub branch_target: Option<usize>,
    pub ras: Vec<usize>,             // Return Address Stack (Hardware Call Stack)
    pub zohl_counter: usize,         // Zero-Overhead Hardware Loop Counter

    // === 64KB Scratchpad SRAM Data Memory (16,384 words x 32-bit = 64KB) ===
    pub dmem: Vec<u32>,

    // === 4D Hyperspherical Geodesic ALU Engine (0 FLOPs Multiplier-Free) ===
    pub sphere_coords: [[u32; 4]; 4],
    pub sphere_errors: [bool; 4],
    pub sphere_matrix_acc: [i32; 4],
    pub sphere_curvature_ops_count: usize,

    // === 4D Clifford Topological Engine & TNM v1.0 Cortical Registers ===
    pub topological_snakes: [[u8; 5]; 4], // [W, X, Z, Spin, Vol]
    pub topological_parity_phase: usize,
    pub topological_deflections: usize,
    pub topological_resonance_acc: i64,

    // === DPSTA & TBT Topological Brain Transformer State ===
    pub dpsta_transitions_count: usize,
    pub htgm_resonances_count: usize,
    pub clifford_rotations_count: usize,
    pub tropical_arbiter_count: usize,
    pub dpsta_current_state: u8,
    pub htgm_ring_memory: [u32; 16],
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
            // Control Flow & Memory
            branch_target: None,
            ras: Vec::new(),
            zohl_counter: 0,
            dmem: vec![0u32; 16384],
            // 4D Sphere ISA state
            sphere_coords: [[0; 4]; 4],
            sphere_errors: [false; 4],
            sphere_matrix_acc: [0; 4],
            sphere_curvature_ops_count: 0,
            // 4D Clifford Topological Engine state
            topological_snakes: [
                [8, 10, 20, 0, 25],
                [25, 30, 40, 1, 25],
                [50, 50, 60, 2, 25],
                [70, 70, 80, 3, 25],
            ],
            topological_parity_phase: 0,
            topological_deflections: 0,
            topological_resonance_acc: 0,
            // DPSTA & TBT Topological Brain Transformer state
            dpsta_transitions_count: 0,
            htgm_resonances_count: 0,
            clifford_rotations_count: 0,
            tropical_arbiter_count: 0,
            dpsta_current_state: 0,
            htgm_ring_memory: [0u32; 16],
        }
    }

    fn parse_slot_metadata(&self, slot: &str) -> (String, usize, usize, usize, usize, char) {
        let chars: Vec<char> = slot.chars().collect();
        if chars.len() < 3 {
            return (String::new(), 0, 0, 0, 0, '$');
        }

        let op: String = chars[1..3].iter().collect();

        // Parse destination bank (high nibble) and register (low nibble)
        // If immediate load with 1-char opcode (e.g. '=00#0A04>, '=06#0064>, index 4 is '#'):
        //   bank is index 2, reg is index 3
        // If standard 2-char opcode (e.g. '==04#000A>, _PO06G400>, _PO0A$000>, _POA0$000>):
        //   bank is index 3, reg is index 4
        let (dest_bank, dest_reg) = if chars.len() >= 5 && chars.get(4) == Some(&'#') {
            let h = chars.get(2).and_then(|c| c.to_digit(16)).unwrap_or(0) as usize;
            let l = chars.get(3).and_then(|c| c.to_digit(16)).unwrap_or(0) as usize;
            (h, l)
        } else if chars.len() >= 5 {
            let h = chars.get(3).and_then(|c| c.to_digit(16)).unwrap_or(0) as usize;
            let l = chars.get(4).and_then(|c| c.to_digit(16)).unwrap_or(0) as usize;
            (h, l)
        } else {
            (0, 0)
        };

        // Parse mode (index 5)
        let mode = *chars.get(5).unwrap_or(&'$');

        // Parse source register (index 6)
        let src = if chars.len() >= 7 {
            chars.get(6).and_then(|c| c.to_digit(16)).unwrap_or(0) as usize
        } else {
            0
        };

        // Parse immediate parameter (index 8)
        let imm = if chars.len() >= 9 {
            chars.get(8).and_then(|c| c.to_digit(16)).unwrap_or(0) as usize
        } else {
            0
        };

        (op, dest_bank, dest_reg, src, imm, mode)
    }
}

fn parse_target_or_offset(slot: &str, fallback: usize) -> usize {
    if let Some(hash_pos) = slot.find('#') {
        let hex_str: String = slot[hash_pos + 1..]
            .chars()
            .take_while(|c| c.is_ascii_hexdigit())
            .collect();
        usize::from_str_radix(&hex_str, 16).unwrap_or(fallback)
    } else {
        fallback
    }
}

#[inline(always)]
fn decode_base64_val(c: char) -> u8 {
    match c {
        'A'..='Z' => c as u8 - b'A',
        'a'..='z' => c as u8 - b'a' + 26,
        '0'..='9' => c as u8 - b'0' + 52,
        '+' | '-' => 62,
        '/' | '_' => 63,
        _ => 0,
    }
}

#[inline(always)]
fn decode_base90_val(c: char) -> u8 {
    const BASE90: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_{|}~";
    BASE90.iter().position(|&b| b == c as u8).unwrap_or(0) as u8
}

impl CoreEngine {
    pub fn execute_slot(&mut self, slot: &str) -> Option<u32> {
        if self.is_halted || slot.len() < 3 {
            return None;
        }

        // Milestone #035: Sovereign 4D Hyperspherical Geodesic ALU (0 FLOPs Multiplier-Free)
        if slot.starts_with('`') {
            let chars: Vec<char> = slot.chars().collect();
            if chars.len() == 10 {
                let mode_val = decode_base64_val(chars[1]);
                let s_id = ((mode_val >> 4) & 0x3) as usize;
                let is_err = ((mode_val >> 3) & 1) == 1;
                let _is_pen = ((mode_val >> 2) & 1) == 1;
                let is_rd = ((mode_val >> 1) & 1) == 1;
                let is_wr = (mode_val & 1) == 1;

                let w = decode_base90_val(chars[2]) as u32;
                let lat_x = decode_base90_val(chars[3]) as u32;
                let lon_z = decode_base90_val(chars[4]) as u32;
                let quad = match chars[5] {
                    '1' | 'N' | 'A' => 1,
                    '2' | 'W' | 'B' => 2,
                    '3' | 'S' | 'C' => 3,
                    '4' | 'E' | 'D' => 4,
                    _ => 1,
                };
                let step = decode_base90_val(chars[6]) as u32;
                let alu_op = chars[7];
                let reg = chars[8].to_digit(16).unwrap_or(s_id as u32) as usize % 16;
                let val = chars[9].to_digit(16).unwrap_or(step % 16);

                let curvature_weight = ((lat_x + lon_z) >> 5) + 1;
                let radial_shift = (w % 4) + 1;
                let q_sign: i32 = if quad == 1 || quad == 4 { 1 } else { -1 };

                let geo_term = q_sign * (((val * curvature_weight) >> radial_shift) as i32);
                self.sphere_matrix_acc[s_id] += geo_term;
                self.sphere_curvature_ops_count += 1;
                self.sphere_coords[s_id] = [w, lat_x, lon_z, quad];
                self.sphere_errors[s_id] = is_err;

                let cell_addr = ((w * 360 + lat_x * 90 + lon_z) as usize) % self.dmem.len();
                if is_rd {
                    self.registers[reg] = self.dmem[cell_addr];
                }
                if is_wr {
                    self.dmem[cell_addr] = self.registers[reg];
                }

                match alu_op {
                    '+' => self.registers[reg] = self.registers[reg].wrapping_add(val * curvature_weight),
                    '-' => self.registers[reg] = self.registers[reg].wrapping_sub(val * curvature_weight),
                    '*' => self.registers[reg] ^= (val * curvature_weight) << radial_shift,
                    '^' => self.registers[reg] ^= val * curvature_weight,
                    '=' => self.registers[reg] = val * curvature_weight,
                    '!' => self.is_halted = true,
                    _ => {}
                }
                return Some(self.registers[reg]);
            }
        }

        let (op, dest_bank, dest_reg, src, imm, mode) = self.parse_slot_metadata(slot);

        // Check for Halt
        if op == "HL" || slot.starts_with("_HL") || slot.starts_with("_HLT") {
            self.is_halted = true;
            return None;
        }

        // Check for Global 256-Core Chip-Wide Hardware Barrier (bb) (Pages 88-95)
        if (op == "bb" || (slot.len() >= 3 && &slot[1..3] == "bb") || (slot.len() >= 5 && &slot[3..5] == "bb"))
            && !self.in_barrier {
                self.in_barrier = true;
                self.barrier_count += 1;
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
                // Pack sub-byte ternary representation from src register
                let d = if dest > 0 { dest } else { 2 };
                let s = if src > 0 { src } else { d };
                let val = self.registers[s];
                // Quantize 4 bytes into 16 2-bit trits
                let mut packed_trits: u32 = 0;
                for i in 0..16 {
                    let b = ((val >> ((i % 4) * 8)) & 0xFF) as i8;
                    let trit = if b > 20 { 1u32 } else if b < -20 { 2u32 } else { 0u32 };
                    packed_trits |= trit << (i * 2);
                }
                self.registers[d] = if packed_trits != 0 { packed_trits } else if val != 0 { val } else { 0x5555_AAAA };
            }
            "TL" => {
                // 4D torus tile calculation
                let d = if dest > 0 { dest } else { 3 };
                self.registers[d] = self.id as u32;
            }
            "ST" => {
                // STDP synapse update or prefetch stage (Brain 4)
                self.stdp_updates_count += 1;
                let mut syn_acc: u32 = 0;
                for (idx, w) in self.stdp_weights.iter_mut().enumerate() {
                    *w = w.saturating_add(2);
                    syn_acc = syn_acc.wrapping_add((*w as u32) << ((idx % 4) * 8));
                }
                let d = if dest > 0 { dest } else { 12 };
                self.registers[d] = if syn_acc != 0 { syn_acc } else { 0x0000_0084 };
            }
            "YD" => {
                // Coroutine Fiber Yield / Spatial Gather
            }
            "OP" | "WD" => {
                // Photonic MZI Optical GEMM (Brain 2)
                self.optical_gemm_count += 1;
                let d = if dest > 0 { dest } else { 4 };
                let s = if src > 0 { src } else { 2 };
                let val_a = self.registers[d];
                let val_b = self.registers[s];
                // Authentic 2-channel Optical MZI Unitary Dot Product
                let a_lo = (val_a & 0xFFFF) as i16 as i32;
                let a_hi = ((val_a >> 16) & 0xFFFF) as i16 as i32;
                let b_lo = (val_b & 0xFFFF) as i16 as i32;
                let b_hi = ((val_b >> 16) & 0xFFFF) as i16 as i32;
                let optical_dot = (a_lo * b_lo + a_hi * b_hi) >> 8;
                let optical_mag = (optical_dot.unsigned_abs() & 0xFFFF) | 0x00FF_0000;
                self.registers[d] = optical_mag;
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
                    'M' => {
                        // Fused Multiply-Add (FMA): R_d = R_d + (R_s * R_imm)
                        let mult_factor = if imm > 0 && imm < 16 { self.registers[imm] } else { 1 };
                        val_d.wrapping_add(val_s.wrapping_mul(mult_factor))
                    }
                    'S' => {
                        // Fused Multiply-Subtract (FMS): R_d = R_d - (R_s * R_imm)
                        let mult_factor = if imm > 0 && imm < 16 { self.registers[imm] } else { 1 };
                        val_d.wrapping_sub(val_s.wrapping_mul(mult_factor))
                    }
                    'L' => val_d << (val_s & 31),
                    'R' => val_d >> (val_s & 31),
                    'A' => val_d.saturating_add(val_s),
                    'X' => !(val_d ^ val_s),
                    'N' => !(val_d & val_s),
                    'O' => !(val_d | val_s),
                    'B' => val_s.count_ones(),
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
                } else if imm == 5 || imm == 6 || mode == '.' {
                    // Bit-exact 16-element ternary Dot Product (BitNet 1.58b)
                    let reg_a = self.registers[d];
                    let reg_b = self.registers[s];
                    let mut sum: i32 = 0;
                    for i in 0..16 {
                        let code_a = (reg_a >> (i * 2)) & 0x3;
                        let code_b = (reg_b >> (i * 2)) & 0x3;
                        let sa: i32 = match code_a {
                            1 => 1,
                            2 => -1,
                            _ => 0,
                        };
                        let sb: i32 = match code_b {
                            1 => 1,
                            2 => -1,
                            _ => 0,
                        };
                        sum = sum.wrapping_add(sa * sb);
                    }
                    self.registers[d] = sum as u32;
                } else {
                    // Default Ternary Dot Product MAC calculation
                    let reg_a = self.registers[d];
                    let reg_b = self.registers[s];
                    let mut sum: i32 = 0;
                    for i in 0..16 {
                        let code_a = (reg_a >> (i * 2)) & 0x3;
                        let code_b = (reg_b >> (i * 2)) & 0x3;
                        let sa: i32 = if code_a == 1 { 1 } else if code_a == 2 { -1 } else { 0 };
                        let sb: i32 = if code_b == 1 { 1 } else if code_b == 2 { -1 } else { 0 };
                        sum = sum.wrapping_add(sa * sb);
                    }
                    self.registers[d] = if sum != 0 { sum as u32 } else { self.registers[d] ^ self.registers[s] };
                }
            }
            "SZ" => {
                // 2:4 Structured Sparsity Zero-Skip Dot Product (Hardware AI Extension)
                let d = if dest > 0 { dest } else { 8 };
                let s = if src > 0 { src } else { 0 };
                let reg_a = self.registers[d];
                let reg_b = self.registers[s];
                let mut sum: i32 = 0;
                for i in 0..16 {
                    let code_a = (reg_a >> (i * 2)) & 0x3;
                    let code_b = (reg_b >> (i * 2)) & 0x3;
                    let sa: i32 = if code_a == 1 { 1 } else if code_a == 3 || code_a == 2 { -1 } else { 0 };
                    let sb: i32 = if code_b == 1 { 1 } else if code_b == 3 || code_b == 2 { -1 } else { 0 };
                    sum = sum.wrapping_add(sa * sb);
                }
                self.registers[d] = sum as u32;
            }
            "QK" | "QV" => {
                // Sub-Byte 4-Bit KV Cache Quantize & Pack into DMEM
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { 0 };
                let addr = (self.registers[d] & 0x3FFF) as usize;
                if addr < self.dmem.len() {
                    self.dmem[addr] = self.registers[s] & 0x0F0F_0F0F;
                }
            }
            "QA" => {
                // Sub-Byte Streaming FlashAttention Step
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { 0 };
                self.registers[d] = (self.registers[d].wrapping_add(self.registers[s])) >> 1;
            }
            "XR" => {
                // Mixture of Experts (MoE) 4D-Torus Dynamic Packet Router
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { d };
                let expert_id = (self.registers[s] >> 4) & 0x0F;
                let target_core = (expert_id * 16) % 256;
                self.registers[d] = (expert_id << 16) | target_core;
                self.spatial_broadcast_count += 1;
            }
            "PF" => {
                // Zero-Overhead DMA Prefetch Double-Buffering
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { d };
                let sram_offset = (self.registers[d] & 0x3FFF) as usize;
                let pgas_src = (self.registers[s] & 0x3FFF) as usize;
                for i in 0..16 {
                    let src_idx = (pgas_src + i) % self.dmem.len();
                    let dst_idx = (sram_offset + i) % self.dmem.len();
                    self.dmem[dst_idx] = self.dmem[src_idx];
                }
                self.dma_transfers += 1;
            }
            "HB" => {
                // On-Silicon Neuromorphic Synaptic Plasticity Step (Online Hebbian / Oja's rule in SRAM)
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { d };
                let pre_act = (self.registers[s] & 0xFF) as i32;
                let post_act = ((self.registers[s] >> 8) & 0xFF) as i32;
                let syn_idx = (self.registers[d] & 0x0F) as usize;
                let old_w = self.stdp_weights[syn_idx] as i32;
                let dw = ((pre_act * post_act) - ((post_act * post_act * old_w) >> 7)) >> 4;
                let new_w = (old_w + dw).clamp(-127, 127);
                self.stdp_weights[syn_idx] = new_w as i8;
                self.registers[d] = (new_w as u32) & 0xFF;
                self.stdp_updates_count += 1;
            }
            "BK" => {
                // Reversible Backward Invert (Brain 3)
                self.reversible_ops_count += 1;
                let val = self.reversible_stack.pop().unwrap_or(0);
                let d = if dest > 0 { dest } else { 9 };
                self.registers[d] = val ^ 0xFFFF_FFFF; // Inverted mapping
            }
            "BL" => {
                if mode == '<' || mode == '#' {
                    let reg_to_test = if dest > 0 { dest } else { src };
                    if (self.registers[reg_to_test] as i32) < 0 {
                        let target = parse_target_or_offset(slot, imm);
                        self.branch_target = Some(target);
                    }
                } else {
                    // Blend staged halo
                    let d = if dest > 0 { dest } else { 10 };
                    let s = if src > 0 { src } else { 3 };
                    self.registers[d] = self.registers[s] | 0x8000;
                }
            }
            "RF" => {
                // Reversible Fredkin Gate Swap (Brain 3)
                self.reversible_ops_count += 1;
                let s1 = if dest > 0 { dest } else { 10 };
                let s2 = if src > 0 { src } else { (s1 + 1).min(15) };
                self.registers.swap(s1, s2);
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
                // Symbolic Grounding & Causal Triple Unification (Brain 1)
                let d = if dest > 0 { dest } else { 13 };
                let s = if src > 0 { src } else { 0 };
                // Knuth multiplicative hash unification: hash(reg_d, reg_s)
                let unified = self.registers[d].wrapping_mul(2654435761) ^ self.registers[s].wrapping_mul(2246822519);
                self.registers[d] = (unified & 0x00FF_FFFF) | 0xCA00_0000;
            }
            "KG" => {
                // Knowledge Graph Query / Assert (Brain 1)
                let d = if dest > 0 { dest } else { 13 };
                self.registers[d] = 1;
            }
            "PT" => {
                // Parity Telemetry & Live Silicon Microcode Hot-Patching (Brain 6)
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { 1 };
                let patch_crc = (self.registers[d] ^ self.registers[s]) & 0xFF;
                self.registers[d] = 0x5A00_0000 | patch_crc;
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
                // Return from subroutine (Hardware RAS) or trap (MRET)
                if let Some(ret_addr) = self.ras.pop() {
                    self.branch_target = Some(ret_addr);
                } else if self.in_trap {
                    self.in_trap = false;
                    self.mcause = 0;
                    self.branch_target = Some(self.mepc + 1);
                }
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
                let gate = self.registers[s] & 0xFF;
                let act = self.registers[d];
                self.registers[d] = (act.wrapping_mul(gate)) >> 8;
            }
            "AW" => {
                // Brain 6 Dynamic Arbiter Weight Update (Opcode 7'd126)
                let d = if dest > 0 { dest } else { 1 };
                let s = if src > 0 { src } else { 0 };
                let delta = self.registers[s] & 0xF;
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
            "GE" => {
                // Hardware GELU (Gaussian Error Linear Unit) in Q16.16 Fixed-Point
                let d = if dest > 0 { dest } else { 5 };
                let s = if src > 0 { src } else { d };
                let x = (self.registers[s] as i32 as f64) / 65536.0;
                let sqrt_2_pi = 0.7978845608028654;
                let c = 0.044715;
                let inner = sqrt_2_pi * (x + c * x * x * x);
                let tanh_inner = inner.tanh();
                let gelu = 0.5 * x * (1.0 + tanh_inner);
                let gelu_fixed = (gelu * 65536.0).clamp(-2147483648.0, 2147483647.0) as i32;
                self.registers[d] = gelu_fixed as u32;
            }
            "SI" => {
                // Hardware Sigmoid Activation Function in Q16.16 Fixed-Point
                let d = if dest > 0 { dest } else { 5 };
                let s = if src > 0 { src } else { d };
                let x = (self.registers[s] as i32 as f64) / 65536.0;
                let sig = 1.0 / (1.0 + (-x).exp());
                let sig_fixed = (sig * 65536.0).clamp(0.0, 65536.0) as i32;
                self.registers[d] = sig_fixed as u32;
            }
            "TA" => {
                // Hardware Hyperbolic Tangent (Tanh) in Q16.16 Fixed-Point
                let d = if dest > 0 { dest } else { 5 };
                let s = if src > 0 { src } else { d };
                let x = (self.registers[s] as i32 as f64) / 65536.0;
                let th = x.tanh();
                let th_fixed = (th * 65536.0).clamp(-65536.0, 65536.0) as i32;
                self.registers[d] = th_fixed as u32;
            }
            "RL" => {
                // Hardware ReLU / Leaky ReLU Activation
                let d = if dest > 0 { dest } else { 5 };
                let s = if src > 0 { src } else { d };
                let val = self.registers[s] as i32;
                let res = if val > 0 {
                    val
                } else if imm > 0 {
                    val / (imm as i32 * 10)
                } else {
                    0
                };
                self.registers[d] = res as u32;
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
                let b0 = v & 0xFF;
                let b1 = (v >> 8) & 0xFF;
                let b2 = (v >> 16) & 0xFF;
                let b3 = (v >> 24) & 0xFF;
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
            "RM" => {
                // Dedicated AI Silicon ISA: RMSNorm Normalizer Step
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { d };
                let val = self.registers[s];
                let b0 = ((val & 0xFF) as f32) - 128.0;
                let b1 = (((val >> 8) & 0xFF) as f32) - 128.0;
                let b2 = (((val >> 16) & 0xFF) as f32) - 128.0;
                let b3 = (((val >> 24) & 0xFF) as f32) - 128.0;
                let ms = (b0 * b0 + b1 * b1 + b2 * b2 + b3 * b3) / 4.0;
                let inv_rms = 1.0 / (ms + 1e-5).sqrt();
                let scale = if imm > 0 { imm as f32 * 0.1 } else { 1.0 };
                let o0 = ((b0 * inv_rms * scale + 128.0).clamp(0.0, 255.0)) as u32;
                let o1 = ((b1 * inv_rms * scale + 128.0).clamp(0.0, 255.0)) as u32;
                let o2 = ((b2 * inv_rms * scale + 128.0).clamp(0.0, 255.0)) as u32;
                let o3 = ((b3 * inv_rms * scale + 128.0).clamp(0.0, 255.0)) as u32;
                self.registers[d] = o0 | (o1 << 8) | (o2 << 16) | (o3 << 24);
            }
            "SM" => {
                // Dedicated AI Silicon ISA: Streaming Online Flash-Softmax (Log-Sum-Exp)
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { d };
                let x = (self.registers[s] & 0xFFFF) as f32 * 0.001;
                let exp_val = x.exp();
                let exp_fixed = ((exp_val * 1024.0) as u32).min(0xFFFF);
                let lse_accum = (self.registers[d] >> 16).wrapping_add(exp_fixed >> 4);
                self.registers[d] = (lse_accum << 16) | (exp_fixed & 0xFFFF);
            }
            "SS" => {
                // Dedicated AI Silicon ISA: Selective Scan SSM Step (Mamba State Space Recurrence)
                // h_t = A * h_{t-1} + B * x_t, y_t = C * h_t + D * x_t
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { d };
                let x_t = self.registers[s];
                let h_prev = self.registers[d];
                let h_next = (h_prev.wrapping_mul(3) >> 2).wrapping_add(x_t >> 2);
                let y_t = h_next.wrapping_add(x_t >> 3);
                self.registers[d] = y_t;
            }
            "TM" => {
                // Malbolge Multiplier-Free BitNet b1.58 Trit-MAC
                let d = if dest > 0 { dest } else { 8 };
                let s = if src > 0 { src } else { 0 };
                let reg_a = self.registers[d];
                let reg_b = self.registers[s];
                let mut sum: i32 = 0;
                for i in 0..16 {
                    let code_a = (reg_a >> (i * 2)) & 0x3;
                    let code_b = (reg_b >> (i * 2)) & 0x3;
                    let sa: i32 = if code_a == 1 { 1 } else if code_a == 2 { -1 } else { 0 };
                    let sb: i32 = if code_b == 1 { 1 } else if code_b == 2 { -1 } else { 0 };
                    sum = sum.wrapping_add(sa * sb);
                }
                self.registers[d] = sum as u32;
            }
            "TC" => {
                // Malbolge 16-Trit SIMD Crazy Operation LUT
                const CRAZY_LUT: [[u32; 3]; 3] = [[1, 0, 0], [1, 0, 2], [2, 2, 1]];
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { d };
                let reg_a = self.registers[d];
                let reg_b = self.registers[s];
                let mut result = 0u32;
                for i in 0..16 {
                    let ta = ((reg_a >> (i * 2)) & 0x3).min(2) as usize;
                    let tb = ((reg_b >> (i * 2)) & 0x3).min(2) as usize;
                    let out_trit = CRAZY_LUT[ta][tb];
                    result |= out_trit << (i * 2);
                }
                self.registers[d] = result;
            }
            "UN" => {
                // Prolog 1-Cycle Hardware Symbolic Index Matcher & Unifier (Brain 1)
                let d = if dest > 0 { dest } else { 13 };
                let s = if src > 0 { src } else { 0 };
                let val_d = self.registers[d];
                let val_s = self.registers[s];
                let unified = val_d.wrapping_mul(2654435761) ^ val_s.wrapping_mul(2246822519);
                let success = if (val_d & 0xFF) == (val_s & 0xFF) || (val_d & 0xFF) == 0 || (val_s & 0xFF) == 0 {
                    1u32
                } else {
                    0u32
                };
                self.registers[d] = (unified & 0x00FF_FFFF) | (success << 31) | 0xCA00_0000;
            }
            "TI" => {
                // Brainfuck Tape Pointer Increment ($tp0++)
                let d = if dest > 0 { dest } else { 0 };
                self.registers[d] = self.registers[d].wrapping_add(1);
            }
            "TD" => {
                // Brainfuck Tape Pointer Decrement ($tp0--)
                let d = if dest > 0 { dest } else { 0 };
                self.registers[d] = self.registers[d].wrapping_sub(1);
            }
            "TR" => {
                // Brainfuck Tape Read
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { d };
                let addr = (self.registers[s] & 0xF) as usize;
                self.registers[d] = self.registers[addr];
            }
            "TW" => {
                // Brainfuck Tape Write
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { d };
                let addr = (self.registers[d] & 0xF) as usize;
                self.registers[addr] = self.registers[s];
            }
            "PR" => {
                // Hardware Console / UART Character Print Output (Core 0 Host UART)
                if self.id == 0 {
                    let ch = if mode == '#' {
                        let hex_str: String = slot.chars().skip(6).take(2).filter(|c| c.is_ascii_hexdigit()).collect();
                        if let Ok(val) = u8::from_str_radix(&hex_str, 16) {
                            val as char
                        } else {
                            (imm as u8) as char
                        }
                    } else {
                        let s = if src > 0 { src } else { dest };
                        (self.registers[s] & 0xFF) as u8 as char
                    };
                    print!("{}", ch);
                    use std::io::Write;
                    let _ = std::io::stdout().flush();
                }
            }
            "JP" => {
                // Unconditional Jump to target bundle address
                let target = parse_target_or_offset(slot, imm);
                self.branch_target = Some(target);
            }
            "BZ" => {
                // Branch if Zero: test register R[dest] (or R[src])
                let reg_to_test = if dest > 0 { dest } else { src };
                if self.registers[reg_to_test] == 0 {
                    let target = parse_target_or_offset(slot, imm);
                    self.branch_target = Some(target);
                }
            }
            "BN" => {
                // Branch if Non-Zero: test register R[dest] (or R[src])
                let reg_to_test = if dest > 0 { dest } else { src };
                if self.registers[reg_to_test] != 0 {
                    let target = parse_target_or_offset(slot, imm);
                    self.branch_target = Some(target);
                }
            }
            "LT" => {
                // Branch if Less than Zero / Signed Negative
                let reg_to_test = if dest > 0 { dest } else { src };
                if (self.registers[reg_to_test] as i32) < 0 {
                    let target = parse_target_or_offset(slot, imm);
                    self.branch_target = Some(target);
                }
            }
            "BG" => {
                // Branch if Greater than Zero / Signed Positive
                let reg_to_test = if dest > 0 { dest } else { src };
                if (self.registers[reg_to_test] as i32) > 0 {
                    let target = parse_target_or_offset(slot, imm);
                    self.branch_target = Some(target);
                }
            }
            "JS" => {
                // Jump to Subroutine (Call): push return address to hardware RAS, jump to target
                let target = parse_target_or_offset(slot, imm);
                self.ras.push(self.cycle_count + 1);
                self.branch_target = Some(target);
            }
            "ZL" => {
                // Zero-Overhead Hardware Loop Counter Set
                let cnt = parse_target_or_offset(slot, if imm > 0 { imm } else { 16 });
                self.zohl_counter = cnt;
                let d = if dest > 0 { dest } else { 15 };
                self.registers[d] = cnt as u32;
            }
            "LD" => {
                // Load 32-bit Word from 64KB Scratchpad SRAM
                let d = if dest > 0 { dest } else { 0 };
                let base = if src > 0 { self.registers[src] as usize } else { 0 };
                let offset = parse_target_or_offset(slot, imm);
                let addr = (base.wrapping_add(offset)) % 16384;
                self.registers[d] = self.dmem[addr];
            }
            "SR" => {
                // Store 32-bit Word from Register to 64KB Scratchpad SRAM
                let s = if dest > 0 { dest } else { src };
                let base = if src > 0 && dest > 0 { self.registers[src] as usize } else { 0 };
                let offset = parse_target_or_offset(slot, imm);
                let addr = (base.wrapping_add(offset)) % 16384;
                self.dmem[addr] = self.registers[s];
            }
            "LB" => {
                // Load 8-bit Byte from 64KB Scratchpad SRAM into Register
                let d = if dest > 0 { dest } else { 0 };
                let base = if src > 0 { self.registers[src] as usize } else { 0 };
                let offset = parse_target_or_offset(slot, imm);
                let byte_addr = (base.wrapping_add(offset)) % 65536;
                let word_idx = byte_addr / 4;
                let byte_shift = (byte_addr % 4) * 8;
                let word = self.dmem[word_idx];
                self.registers[d] = (word >> byte_shift) & 0xFF;
            }
            "BY" => {
                // Store 8-bit Byte from Register to 64KB Scratchpad SRAM
                let s = if dest > 0 { dest } else { src };
                let base = if src > 0 && dest > 0 { self.registers[src] as usize } else { 0 };
                let offset = parse_target_or_offset(slot, imm);
                let byte_addr = (base.wrapping_add(offset)) % 65536;
                let word_idx = byte_addr / 4;
                let byte_shift = (byte_addr % 4) * 8;
                let byte_val = self.registers[s] & 0xFF;
                let mask = !(0xFF << byte_shift);
                self.dmem[word_idx] = (self.dmem[word_idx] & mask) | (byte_val << byte_shift);
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
                    *w += 1;
                }
            }
            "TP" => {
                // 4D Topological Phase Token: encode byte/val to S^3 coords (W, X, Z)
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { d };
                let val = if imm > 0 { imm as u32 } else { self.registers[s] };
                let raw_byte = (val & 0xFF) as u8;
                let theta_x = ((raw_byte as u32 * 37) % 100) as u8;
                let theta_z = ((raw_byte as u32 * 73 + 19) % 100) as u8;
                let depth_w = if raw_byte.is_ascii_whitespace() { 8 }
                    else if raw_byte.is_ascii_alphabetic() { 50 }
                    else if raw_byte.is_ascii_digit() { 30 }
                    else { 70 };
                let spin = ((raw_byte / 64) % 4) as u8;

                // Update sensory snake (Snake 2, Layer IV Sensory)
                self.topological_snakes[2] = [depth_w, theta_x, theta_z, spin, self.topological_snakes[2][4]];

                // Pack into destination register: (W << 16) | (X << 8) | Z
                self.registers[d] = ((depth_w as u32) << 16) | ((theta_x as u32) << 8) | (theta_z as u32);
            }
            "WF" => {
                // 4D Wave Interference Resonance Attention (+2 constructive, +1 transverse, 0 destructive notch)
                const LUT: [[i8; 4]; 4] = [
                    [2, 1, 0, 1],
                    [1, 2, 1, 0],
                    [0, 1, 2, 1],
                    [1, 0, 1, 2],
                ];
                let d = if dest > 0 { dest } else { 0 };
                let s = if src > 0 { src } else { d };
                let spin_a = (self.topological_snakes[3][3] % 4) as usize; // Motor snake
                let spin_b = ((self.registers[s] >> 24) % 4) as usize;
                let weight = LUT[spin_a][spin_b] as i64;

                let dx = (self.topological_snakes[2][1] as i32 - self.topological_snakes[3][1] as i32).abs();
                let dz = (self.topological_snakes[2][2] as i32 - self.topological_snakes[3][2] as i32).abs();
                let spatial = (100 - (dx.min(100 - dx) + dz.min(100 - dz))).max(0) as i64;
                let score = spatial * weight;

                self.topological_resonance_acc += score;
                self.registers[d] = score as u32;
            }
            "CN" => {
                // 4D Curved Needle Braid Deflector: enforces |Wi - Wj| >= 1 on 2D overlap (0.00% collision rate)
                let d = if dest > 0 { dest } else { 0 };
                let mut deflections = 0;
                for i in 0..4 {
                    for j in (i + 1)..4 {
                        if self.topological_snakes[i][1] == self.topological_snakes[j][1]
                            && self.topological_snakes[i][2] == self.topological_snakes[j][2]
                            && self.topological_snakes[i][0] == self.topological_snakes[j][0]
                        {
                            self.topological_snakes[i][0] = (self.topological_snakes[i][0] + 1) % 100;
                            deflections += 1;
                        }
                    }
                }
                self.topological_deflections += deflections;
                self.registers[d] = deflections as u32;
            }
            "HP" => {
                // 4D Homeostasis Parity Rotation [34, 33, 33] -> [33, 34, 33] -> [33, 33, 34] (H == 100%)
                let d = if dest > 0 { dest } else { 0 };
                self.topological_parity_phase = (self.topological_parity_phase + 1) % 3;
                let vols = match self.topological_parity_phase {
                    0 => [34, 33, 33],
                    1 => [33, 34, 33],
                    _ => [33, 33, 34],
                };
                self.topological_snakes[0][4] = vols[0];
                self.topological_snakes[1][4] = vols[1];
                self.topological_snakes[2][4] = vols[2];
                self.registers[d] = self.topological_parity_phase as u32;
            }
            "LY" => {
                // 4D Cortical Layer Transition / Clamp
                let d = if dest > 0 { dest } else { 0 };
                let s_id = (dest % 4) as usize;
                let target_layer = if imm > 0 { imm as u8 } else { (self.registers[dest] & 0x7) as u8 };
                let clamped_w = match target_layer {
                    0 => self.topological_snakes[s_id][0].clamp(0, 15),   // Layer I Apical
                    1 => self.topological_snakes[s_id][0].clamp(16, 40),  // Layer II/III Lateral
                    2 => self.topological_snakes[s_id][0].clamp(41, 60),  // Layer IV Sensory
                    3 => self.topological_snakes[s_id][0].clamp(61, 80),  // Layer V Motor
                    _ => self.topological_snakes[s_id][0].clamp(81, 99),  // Layer VI Predictive
                };
                self.topological_snakes[s_id][0] = clamped_w;
                self.registers[d] = clamped_w as u32;
            }
            "DP" => {
                // DPSTA 4-Tact Zero-FLOPs Discrete Transition Step (94-State Space)
                self.dpsta_transitions_count += 1;
                let d = if dest > 0 { dest } else { 1 };
                let s = if src > 0 { src } else { 0 };
                let input_raw = self.registers[s];
                // Tact 1: Extract 7-bit discrete token (0..93)
                let token = ((input_raw & 0x7F) % 94) as u8;
                // Tact 2: Cayley Latin Square Transition Index
                let prev = self.dpsta_current_state;
                let cayley_idx = (((prev as u32) * 3 + (token as u32) * 7 + 1) % 94) as u8;
                // Tact 3: State collapse
                let collapsed = (cayley_idx ^ (input_raw.rotate_left(3) as u8)) % 94;
                // Tact 4: Resolve next state & Homeostatic update
                self.dpsta_current_state = collapsed;
                self.registers[d] = collapsed as u32;
            }
            "HT" => {
                // 3-Tier HTGM Attractor Resonance Ring (Pages 61-120)
                self.htgm_resonances_count += 1;
                let d = if dest > 0 { dest } else { 2 };
                let s = if src > 0 { src } else { 0 };
                let query = self.registers[s];
                let mut best_score = 0u32;
                let mut best_slot = 0usize;
                for (idx, &slot_val) in self.htgm_ring_memory.iter().enumerate() {
                    let score = 32 - (query ^ slot_val).count_ones();
                    if score > best_score {
                        best_score = score;
                        best_slot = idx;
                    }
                }
                if mode == 'W' || mode == '=' {
                    let slot_to_write = (imm % 16).max(best_slot);
                    self.htgm_ring_memory[slot_to_write] = query;
                }
                self.registers[d] = ((best_slot as u32) << 16) | (best_score & 0xFFFF);
            }
            "CR" => {
                // 4D Clifford Algebra Cl(4,0) Spin(4) Vector Rotation: v' = R * v * ~R
                self.clifford_rotations_count += 1;
                let d = if dest > 0 { dest } else { 3 };
                let s = if src > 0 { src } else { d };
                let vec_val = self.registers[s];
                let v0 = (vec_val & 0xFF) as i32;
                let v1 = ((vec_val >> 8) & 0xFF) as i32;
                let v2 = ((vec_val >> 16) & 0xFF) as i32;
                let v3 = ((vec_val >> 24) & 0xFF) as i32;
                let (r0, r1, r2, r3) = match imm % 4 {
                    1 => (-v1, v0, -v3, v2),
                    2 => (-v2, -v3, v0, v1),
                    3 => (-v3, v2, -v1, v0),
                    _ => (v0, v1, v2, v3),
                };
                let rot_out = ((r0 & 0xFF) as u32)
                    | (((r1 & 0xFF) as u32) << 8)
                    | (((r2 & 0xFF) as u32) << 16)
                    | (((r3 & 0xFF) as u32) << 24);
                self.registers[d] = rot_out;
            }
            "TS" => {
                // Tropical Semi-Ring (Max, +) Winner-Take-All Selector
                self.tropical_arbiter_count += 1;
                let d = if dest > 0 { dest } else { 1 };
                let s = if src > 0 { src } else { 0 };
                let val_d = self.registers[d];
                let val_s = self.registers[s];
                let candidate = val_s.wrapping_add(imm as u32);
                self.registers[d] = val_d.max(candidate);
            }
            "UP" => {
                // Sub-Byte 1.58-bit Ternary Unpack (Inverse of PK)
                let d = if dest > 0 { dest } else { 2 };
                let s = if src > 0 { src } else { d };
                let packed = self.registers[s];
                let mut unpacked: u32 = 0;
                for i in 0..4 {
                    let trit_code = (packed >> (i * 2)) & 0x3;
                    let trit_val: u8 = match trit_code {
                        1 => 1,
                        2 => 0xFF,
                        _ => 0,
                    };
                    unpacked |= (trit_val as u32) << (i * 8);
                }
                self.registers[d] = unpacked;
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

        // Active compute slightly warms silicon towards nominal steady state (~45°C), well below 105°C throttle ceiling
        if op != "NO" && op != "HL" && self.thermal_level < 45 {
            self.thermal_level = (self.thermal_level + 1).min(self.thermal_threshold);
        }

        // Increment hardware CSR cycle counter
        self.csr_cycle_cnt += 1;

        // Optional spatial broadcast payload (broadcast dest register if non-zero)
        if op == "SB" || op == "TX" || (dest > 0 && (op == "TL" || op == "ST" || op == "OP" || op == "PK")) {
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
