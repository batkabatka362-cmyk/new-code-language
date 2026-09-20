// ============================================================================
// CRON Interactive Real-Time TUI Swarm & Torus Traffic Visualizer (cl_swarm_tui.rs)
//
// 100% Pure Rust - Zero External Dependencies.
// Full VT100 / ANSI Terminal Graphics, Unicode Box-Drawing, Sub-Character Meters.
//
// Features:
// 1. Tab 1: 4D Torus Plane & Spatial Core Matrix (256-Core Minimap + (Z,W) Zoom Slice + Inspector).
// 2. Tab 2: 16-Chip DWDM Cluster Grid (4x4 Sockets, Optical Link BW, Ring AllReduce).
// 3. Tab 3: NoC Router Microarchitecture & 9-Port VC Buffer Heatmap.
// 4. Tab 4: Swarm Self-Synthesis & Continuous Silicon Vibe-Healing Telemetry.
// 5. Interactive & Non-Interactive (Snapshot / Headless / JSON) execution modes.
// ============================================================================

use std::collections::VecDeque;

/// Active View Mode in the TUI Dashboard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiViewMode {
    TorusPlane,    // Mode 1: 4D Torus Spatial Plane Slice & Agent Matrix
    ClusterMacro,  // Mode 2: 16-Chip DWDM Optical Swarm Cluster (4x4 Grid)
    RouterHeatmap, // Mode 3: 9-Port Crossbar Switch & VC Buffer Congestion
    SwarmTelemetry,// Mode 4: Closed-Loop Synthesis & Vibe-Healing Engine Status
}

impl TuiViewMode {
    pub fn name(&self) -> &'static str {
        match self {
            TuiViewMode::TorusPlane => "4D Torus Plane",
            TuiViewMode::ClusterMacro => "16-Chip Cluster",
            TuiViewMode::RouterHeatmap => "NoC VC Router",
            TuiViewMode::SwarmTelemetry => "Swarm Telemetry",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            TuiViewMode::TorusPlane => 1,
            TuiViewMode::ClusterMacro => 2,
            TuiViewMode::RouterHeatmap => 3,
            TuiViewMode::SwarmTelemetry => 4,
        }
    }

    pub fn from_index(idx: usize) -> Self {
        match idx {
            1 => TuiViewMode::TorusPlane,
            2 => TuiViewMode::ClusterMacro,
            3 => TuiViewMode::RouterHeatmap,
            4 => TuiViewMode::SwarmTelemetry,
            _ => TuiViewMode::TorusPlane,
        }
    }
}

/// Configuration for the Swarm TUI Visualizer
#[derive(Debug, Clone)]
pub struct SwarmTuiConfig {
    pub tick_rate_hz: u64,
    pub use_color: bool,
    pub active_view: TuiViewMode,
    pub selected_z: usize,
    pub selected_w: usize,
    pub selected_chip: usize,
    pub selected_core_x: usize,
    pub selected_core_y: usize,
}

impl Default for SwarmTuiConfig {
    fn default() -> Self {
        Self {
            tick_rate_hz: 10,
            use_color: true,
            active_view: TuiViewMode::TorusPlane,
            selected_z: 0,
            selected_w: 0,
            selected_chip: 0,
            selected_core_x: 0,
            selected_core_y: 0,
        }
    }
}

/// A transient flit packet moving along the toroidal mesh in the TUI simulation
#[derive(Debug, Clone)]
pub struct TuiPacketTrail {
    pub packet_id: usize,
    pub src_chip: usize,
    pub dst_chip: usize,
    pub current_x: usize,
    pub current_y: usize,
    pub target_x: usize,
    pub target_y: usize,
    pub kind_symbol: char,
    pub hops_taken: usize,
    pub progress: usize, // 0..4 sub-steps
}

/// Live TUI Model and Simulation State Engine
#[derive(Debug, Clone)]
pub struct SwarmTuiModel {
    pub config: SwarmTuiConfig,
    pub tick_count: usize,
    pub is_paused: bool,
    pub packets: Vec<TuiPacketTrail>,
    pub next_packet_id: usize,
    pub recent_logs: VecDeque<String>,
    pub total_healed_faults: usize,
    pub total_packets_routed: usize,
    pub router_buffer_occupancy: [[usize; 4]; 9], // 9 ports x 4 VCs (0..8 flits)
    pub crossbar_grants: [[bool; 9]; 9],
    pub chip_traffic_gbps: [f64; 16],
    pub chip_quorum_pct: [f64; 16],
    pub chip_active_cores: [usize; 16],
    pub tier2_allreduce_step: usize,
    pub current_task: String,
    pub consensus_reached: bool,
    pub simulated_ipc: f64,
    pub simulated_energy_pj: f64,
    pub simulated_temp_c: f64,
    pub last_fault_msg: Option<String>,
}

impl SwarmTuiModel {
    /// Creates a fresh TUI model with default neuromorphic state
    pub fn new(config: SwarmTuiConfig) -> Self {
        let mut model = Self {
            config,
            tick_count: 0,
            is_paused: false,
            packets: Vec::new(),
            next_packet_id: 1,
            recent_logs: VecDeque::with_capacity(16),
            total_healed_faults: 0,
            total_packets_routed: 0,
            router_buffer_occupancy: [[0; 4]; 9],
            crossbar_grants: [[false; 9]; 9],
            chip_traffic_gbps: [0.0; 16],
            chip_quorum_pct: [91.4; 16],
            chip_active_cores: [256; 16],
            tier2_allreduce_step: 0,
            current_task: "Distributed FlashAttention-2 & BitNet b1.58 Multi-Core Fusion".to_string(),
            consensus_reached: true,
            simulated_ipc: 3.84,
            simulated_energy_pj: 0.14,
            simulated_temp_c: 42.5,
            last_fault_msg: None,
        };

        // Seed initial log messages
        model.push_log("CRON 4D/6D Swarm Silicon Subsystem initialized.");
        model.push_log("16-Chip DWDM Optical Mesh online: 3.2 Tbps/socket.");
        model.push_log("256-Core 4D-Torus physical fabric clocked at 3.2 GHz.");
        model.push_log("Autonomous multi-agent swarm synchronized: 100% consensus.");

        // Initialize realistic baseline buffer and traffic state
        model.seed_baseline_traffic();

        model
    }

    fn push_log(&mut self, msg: &str) {
        if self.recent_logs.len() >= 8 {
            self.recent_logs.pop_front();
        }
        self.recent_logs.push_back(format!("[T{:04}] {}", self.tick_count, msg));
    }

    fn seed_baseline_traffic(&mut self) {
        for port in 0..9 {
            for vc in 0..4 {
                self.router_buffer_occupancy[port][vc] = (port * 3 + vc * 2) % 6;
            }
        }
        for chip in 0..16 {
            self.chip_traffic_gbps[chip] = 120.0 + ((chip * 19) % 180) as f64;
            self.chip_quorum_pct[chip] = 88.0 + ((chip * 7) % 12) as f64;
        }
    }

    /// Advances the simulation clock by a single cycle
    pub fn step_tick(&mut self) {
        self.tick_count += 1;

        // 1. Advance packet movements
        let mut completed = Vec::new();
        for (idx, pkt) in self.packets.iter_mut().enumerate() {
            pkt.progress += 1;
            if pkt.progress >= 2 {
                pkt.progress = 0;
                pkt.hops_taken += 1;
                // Simple Dimension-Order step toward target
                if pkt.current_x != pkt.target_x {
                    pkt.current_x = (pkt.current_x + 1) % 4;
                } else if pkt.current_y != pkt.target_y {
                    pkt.current_y = (pkt.current_y + 1) % 4;
                } else {
                    completed.push(idx);
                }
            }
        }

        // Remove completed packets
        for &idx in completed.iter().rev() {
            self.packets.remove(idx);
            self.total_packets_routed += 1;
        }

        // 2. Occasionally spawn background packet flits
        if self.tick_count % 3 == 0 && self.packets.len() < 12 {
            let p_id = self.next_packet_id;
            self.next_packet_id += 1;
            let src_x = (self.tick_count * 3) % 4;
            let src_y = (self.tick_count * 7) % 4;
            let dst_x = (src_x + 2) % 4;
            let dst_y = (src_y + 3) % 4;
            let symbols = ['●', '◆', '▲', '■', '★'];
            let kind_symbol = symbols[self.tick_count % symbols.len()];

            self.packets.push(TuiPacketTrail {
                packet_id: p_id,
                src_chip: self.config.selected_chip,
                dst_chip: (self.config.selected_chip + 1) % 16,
                current_x: src_x,
                current_y: src_y,
                target_x: dst_x,
                target_y: dst_y,
                kind_symbol,
                hops_taken: 0,
                progress: 0,
            });
        }

        // 3. Modulate router buffer occupancy and crossbar switch
        for port in 0..9 {
            for vc in 0..4 {
                let delta = if (self.tick_count + port + vc) % 5 == 0 {
                    1
                } else if (self.tick_count + port + vc) % 7 == 0 {
                    if self.router_buffer_occupancy[port][vc] > 0 { 8 } else { 0 }
                } else {
                    0
                };
                if delta == 1 && self.router_buffer_occupancy[port][vc] < 8 {
                    self.router_buffer_occupancy[port][vc] += 1;
                } else if delta == 8 && self.router_buffer_occupancy[port][vc] > 0 {
                    self.router_buffer_occupancy[port][vc] -= 1;
                }
            }
        }

        // Randomize active crossbar grants
        for in_p in 0..9 {
            let grant_out = (in_p + self.tick_count) % 9;
            for out_p in 0..9 {
                self.crossbar_grants[in_p][out_p] = out_p == grant_out;
            }
        }

        // 4. Update optical cluster bandwidth modulation
        for chip in 0..16 {
            let wave = ((self.tick_count as f64 * 0.15 + chip as f64).sin() * 45.0) as f64;
            self.chip_traffic_gbps[chip] = (200.0 + wave).clamp(50.0, 360.0);
        }

        // 5. Advance optical ring AllReduce step
        self.tier2_allreduce_step = (self.tier2_allreduce_step + 1) % 16;

        // 6. Dynamic temperature and IPC modulation
        self.simulated_ipc = 3.80 + ((self.tick_count as f64 * 0.2).sin() * 0.15);
        self.simulated_temp_c = 41.5 + ((self.tick_count as f64 * 0.05).sin() * 3.0);
        self.simulated_energy_pj = 0.138 + ((self.tick_count as f64 * 0.1).cos() * 0.015);
    }

    /// Injects a synthetic hardware fault / bit-flip into the fabric to demonstrate Vibe-Healing
    pub fn inject_fault(&mut self) {
        self.total_healed_faults += 1;
        let fault_core = (self.tick_count * 17) % 256;
        let err_kinds = [
            "CRC-8 ATM Parity Mismatch on Bundle B0012 slot 2",
            "RAW Pipeline Hazard detected on R4: Writeback cycle collision",
            "Single-Event Upset (SEU) in VC3 flit header parity",
            "Thermal Throttle detected on Chip 03: Rerouting around hotspot",
        ];
        let chosen_err = err_kinds[self.total_healed_faults % err_kinds.len()];
        let log_entry = format!("FAULT INJECTED at Core #{}: {} -> Vibe-Healer engaged.", fault_core, chosen_err);
        self.last_fault_msg = Some(format!("Core #{:03}: {} -> HEALED", fault_core, chosen_err));
        self.push_log(&log_entry);
        self.push_log("Vibe-Healing: AST mutated, NOP compacted, CRC-8 regenerated in 1 cycle.");
    }

    /// Injects a packet burst between chips
    pub fn inject_packet(&mut self, src_chip: usize, dst_chip: usize) {
        let p_id = self.next_packet_id;
        self.next_packet_id += 1;
        self.packets.push(TuiPacketTrail {
            packet_id: p_id,
            src_chip,
            dst_chip,
            current_x: 0,
            current_y: 0,
            target_x: 3,
            target_y: 3,
            kind_symbol: '★',
            hops_taken: 0,
            progress: 0,
        });
        self.push_log(&format!("Packet #{} injected: Chip {} -> Chip {} (High Priority Broadcast)", p_id, src_chip, dst_chip));
    }

    /// Cycles through the Torus Z slice
    pub fn cycle_z(&mut self, forward: bool) {
        if forward {
            self.config.selected_z = (self.config.selected_z + 1) % 4;
        } else {
            self.config.selected_z = (self.config.selected_z + 3) % 4;
        }
        self.push_log(&format!("Switched 4D Torus Plane to Z={}, W={}", self.config.selected_z, self.config.selected_w));
    }

    /// Cycles through the Torus W slice
    pub fn cycle_w(&mut self, forward: bool) {
        if forward {
            self.config.selected_w = (self.config.selected_w + 1) % 4;
        } else {
            self.config.selected_w = (self.config.selected_w + 3) % 4;
        }
        self.push_log(&format!("Switched 4D Torus Plane to Z={}, W={}", self.config.selected_z, self.config.selected_w));
    }

    /// Sets the active view mode
    pub fn set_view_mode(&mut self, mode: TuiViewMode) {
        self.config.active_view = mode;
        self.push_log(&format!("View mode switched to: {}", mode.name()));
    }

    /// Toggles play/pause
    pub fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
        if self.is_paused {
            self.push_log("Simulation PAUSED [Space to resume]");
        } else {
            self.push_log("Simulation RESUMED");
        }
    }

    // ========================================================================
    // ANSI Rendering Helpers
    // ========================================================================

    fn c(&self, code: &str, text: &str) -> String {
        if self.config.use_color {
            format!("\x1b[{}m{}\x1b[0m", code, text)
        } else {
            text.to_string()
        }
    }

    fn meter(&self, value: usize, max: usize, width: usize) -> String {
        let pct = if max > 0 { value.min(max) as f64 / max as f64 } else { 0.0 };
        let filled_chars = (pct * width as f64).round() as usize;
        let mut s = String::new();
        for i in 0..width {
            if i < filled_chars {
                s.push('█');
            } else {
                s.push('░');
            }
        }
        if self.config.use_color {
            if pct < 0.35 {
                self.c("32", &s) // Green
            } else if pct < 0.70 {
                self.c("33", &s) // Yellow
            } else {
                self.c("31;1", &s) // Red
            }
        } else {
            s
        }
    }

    // ========================================================================
    // Full Frame Screen Renderer
    // ========================================================================

    /// Renders a complete ANSI terminal frame (width x height)
    pub fn render_frame(&self, width: usize, _height: usize) -> String {
        let mut out = String::new();
        let term_w = width.max(88);

        // Header Banner
        out.push_str(&self.render_header_banner(term_w));
        out.push_str("\n");

        // View Tabs Navigation
        out.push_str(&self.render_view_tabs(term_w));
        out.push_str("\n");

        // Primary View Panel
        match self.config.active_view {
            TuiViewMode::TorusPlane => {
                out.push_str(&self.render_tab_torus_plane(term_w));
            }
            TuiViewMode::ClusterMacro => {
                out.push_str(&self.render_tab_cluster_macro(term_w));
            }
            TuiViewMode::RouterHeatmap => {
                out.push_str(&self.render_tab_router_heatmap(term_w));
            }
            TuiViewMode::SwarmTelemetry => {
                out.push_str(&self.render_tab_swarm_telemetry(term_w));
            }
        }
        out.push_str("\n");

        // Bottom Telemetry Ticker & Hotkeys Bar
        out.push_str(&self.render_bottom_status_bar(term_w));

        out
    }

    /// Renders the top brand header banner
    fn render_header_banner(&self, width: usize) -> String {
        let title = "CRON 4D/6D SILICON SWARM - REAL-TIME HARDWARE MONITOR & TUI";
        let state_badge = if self.is_paused {
            self.c("43;30;1", " PAUSED ")
        } else {
            self.c("42;30;1", " LIVE 3.2GHz ")
        };

        let metrics = format!(
            "Tick: {:05} | IPC: {:.2} | Temp: {:.1}°C | Energy: {:.2} pJ/op | Faults Healed: {}",
            self.tick_count, self.simulated_ipc, self.simulated_temp_c, self.simulated_energy_pj, self.total_healed_faults
        );

        let mut s = String::new();
        s.push_str(&self.c("36;1", "╔═"));
        s.push_str(&self.c("1;37", title));
        s.push_str(" ");
        s.push_str(&state_badge);
        let used = 2 + title.len() + 1 + 10;
        let pad = if width > used { width - used } else { 2 };
        for _ in 0..pad.saturating_sub(metrics.len() + 3) {
            s.push_str("═");
        }
        s.push_str(" ");
        s.push_str(&self.c("90", &metrics));
        s.push_str(&self.c("36;1", " ╗\n"));
        s
    }

    /// Renders the 4 navigation tabs
    fn render_view_tabs(&self, width: usize) -> String {
        let tabs = [
            (TuiViewMode::TorusPlane, "[1] 4D Torus Plane"),
            (TuiViewMode::ClusterMacro, "[2] 16-Chip Cluster"),
            (TuiViewMode::RouterHeatmap, "[3] NoC VC Heatmap"),
            (TuiViewMode::SwarmTelemetry, "[4] Swarm Telemetry"),
        ];

        let mut s = String::new();
        s.push_str(&self.c("36;1", "║ "));
        for (mode, label) in tabs.iter() {
            if self.config.active_view == *mode {
                s.push_str(&self.c("46;30;1", &format!(" {} ", label)));
            } else {
                s.push_str(&self.c("37", &format!(" {} ", label)));
            }
            s.push_str("  ");
        }
        s.push_str(&self.c("90", "| Slices: Z="));
        s.push_str(&self.c("33;1", &self.config.selected_z.to_string()));
        s.push_str(&self.c("90", " W="));
        s.push_str(&self.c("33;1", &self.config.selected_w.to_string()));
        s.push_str(&self.c("90", " Chip="));
        s.push_str(&self.c("33;1", &format!("{:02}", self.config.selected_chip)));

        let plain_len = 70;
        let pad = if width > plain_len { width - plain_len } else { 2 };
        for _ in 0..pad.saturating_sub(4) {
            s.push(' ');
        }
        s.push_str(&self.c("36;1", " ║"));
        s
    }

    // ========================================================================
    // TAB 1: 4D Torus Plane & Spatial Core Matrix
    // ========================================================================
    fn render_tab_torus_plane(&self, _width: usize) -> String {
        let mut s = String::new();
        s.push_str(&self.c("36", "╟──────────────────────────────────────────────────────────────────────────────────────────╢\n"));
        s.push_str(&self.c("36;1", "║ "));
        s.push_str(&self.c("1;37", "4D TORUS (4x4x4x4) SPATIAL MESH - 2D PLANE SLICE [Z="));
        s.push_str(&self.c("33;1", &self.config.selected_z.to_string()));
        s.push_str(&self.c("1;37", ", W="));
        s.push_str(&self.c("33;1", &self.config.selected_w.to_string()));
        s.push_str(&self.c("1;37", "]"));
        s.push_str("                   CORE INSPECTOR & COT TRACE          ");
        s.push_str(&self.c("36;1", "║\n"));

        // Roles mapping for 16 cores in (X,Y)
        let roles = [
            ['P', 'C', 'V', 'K'], // Y=0
            ['R', 'M', 'S', 'A'], // Y=1
            ['C', 'C', 'V', 'K'], // Y=2
            ['R', 'M', 'S', 'P'], // Y=3
        ];

        // Draw 4x4 Grid of Cores with Toroidal Interconnects + Sidebar
        for y in 0..4 {
            // Row header with toroidal vertical links
            s.push_str(&self.c("36;1", "║  "));

            // Core cells
            for x in 0..4 {
                let role = roles[y][x];
                let is_selected = x == self.config.selected_core_x && y == self.config.selected_core_y;
                let core_linear_id = self.config.selected_w * 64 + self.config.selected_z * 16 + y * 4 + x;

                // Check if any packet is currently at this (x, y)
                let packet_here = self.packets.iter().find(|p| p.current_x == x && p.current_y == y);

                let cell_content = if let Some(pkt) = packet_here {
                    format!("[{}{}]", self.c("31;1", &pkt.kind_symbol.to_string()), self.c("33;1", &role.to_string()))
                } else if is_selected {
                    self.c("45;37;1", &format!("*{:02}{}*", core_linear_id % 100, role))
                } else {
                    match role {
                        'P' => self.c("32;1", &format!(" C{:02}P ", core_linear_id % 100)),
                        'C' => self.c("36;1", &format!(" C{:02}C ", core_linear_id % 100)),
                        'V' => self.c("35;1", &format!(" C{:02}V ", core_linear_id % 100)),
                        'K' => self.c("33;1", &format!(" C{:02}K ", core_linear_id % 100)),
                        'R' => self.c("34;1", &format!(" C{:02}R ", core_linear_id % 100)),
                        'M' => self.c("37;1", &format!(" C{:02}M ", core_linear_id % 100)),
                        _   => self.c("37",   &format!(" C{:02}{} ", core_linear_id % 100, role)),
                    }
                };

                s.push_str(&cell_content);

                // Horizontal Torus Link
                if x < 3 {
                    let link_active = self.packets.iter().any(|p| p.current_y == y && (p.current_x == x || p.current_x == x + 1));
                    if link_active {
                        s.push_str(&self.c("31;1", "⇄"));
                    } else {
                        s.push_str(&self.c("90", "─"));
                    }
                } else {
                    s.push_str(&self.c("90", "⟲ "));
                }
            }

            // Sidebar info depending on row y
            s.push_str(&self.c("36;1", " │ "));
            match y {
                0 => {
                    let sel_id = self.config.selected_w * 64 + self.config.selected_z * 16 + self.config.selected_core_y * 4 + self.config.selected_core_x;
                    s.push_str(&format!("Selected Core: #{:03} | Role: {} | State: {}",
                        sel_id, self.c("32;1", "Coder/Verifier"), self.c("36;1", "Thinking")));
                }
                1 => {
                    s.push_str(&format!("4D Coords: ({}, {}, {}, {}) | Die: Socket #00",
                        self.config.selected_core_x, self.config.selected_core_y, self.config.selected_z, self.config.selected_w));
                }
                2 => {
                    s.push_str(&format!("Scratchpad: R0=0x{:04X} R1=0x{:04X} | VLIW Slot: 4-way",
                        (self.tick_count * 13) % 0xFFFF, (self.tick_count * 29) % 0xFFFF));
                }
                3 => {
                    s.push_str(&format!("Active Packet Count: {:02} | Flits in flight: {:02}",
                        self.packets.len(), self.packets.len() * 4));
                }
                _ => {}
            }
            s.push_str(&self.c("36;1", "   ║\n"));

            // Torus Vertical Links between rows
            if y < 3 {
                s.push_str(&self.c("36;1", "║   "));
                for x in 0..4 {
                    let vlink_active = self.packets.iter().any(|p| p.current_x == x && (p.current_y == y || p.current_y == y + 1));
                    if vlink_active {
                        s.push_str(&self.c("31;1", " ⇅    "));
                    } else {
                        s.push_str(&self.c("90", " │    "));
                    }
                }
                s.push_str(&self.c("36;1", "     │ "));
                match y {
                    0 => s.push_str("CoT: \"Synthesizing SIMD vector loop with CRC-8 auth...\""),
                    1 => s.push_str("Hazard Check: RAW collision free, 0 bubbles required"),
                    2 => s.push_str("Optical WDM: Laser Lambda-3 locked (1550.92 nm)"),
                    _ => s.push_str(""),
                }
                s.push_str(&self.c("36;1", "    ║\n"));
            }
        }

        // Toroidal Wraparound Footer
        s.push_str(&self.c("36;1", "║  ⟲   ⟲   ⟲   ⟲ (Toroidal Wraparound Y)  │ Total Mesh Throughput: 360.0 GB/s peak       ║\n"));
        s
    }

    // ========================================================================
    // TAB 2: 16-Chip DWDM Cluster Grid
    // ========================================================================
    fn render_tab_cluster_macro(&self, _width: usize) -> String {
        let mut s = String::new();
        s.push_str(&self.c("36", "╟──────────────────────────────────────────────────────────────────────────────────────────╢\n"));
        s.push_str(&self.c("36;1", "║ "));
        s.push_str(&self.c("1;37", "16-CHIP 4,096-CORE DISTRIBUTED SWARM CLUSTER (4x4 DWDM OPTICAL MESH - 3.2 Tbps/socket)     "));
        s.push_str(&self.c("36;1", "║\n"));

        for cy in 0..4 {
            s.push_str(&self.c("36;1", "║  "));
            for cx in 0..4 {
                let chip_id = cy * 4 + cx;
                let bw = self.chip_traffic_gbps[chip_id];
                let is_sel = chip_id == self.config.selected_chip;
                let chip_box = if is_sel {
                    self.c("44;37;1", &format!(" [CHIP {:02}] ", chip_id))
                } else if bw > 250.0 {
                    self.c("31;1", &format!(" [Chip {:02}] ", chip_id))
                } else {
                    self.c("36;1", &format!(" [Chip {:02}] ", chip_id))
                };
                s.push_str(&chip_box);

                if cx < 3 {
                    s.push_str(&self.c("32;1", "═λ═")); // Optical waveguide
                }
            }

            s.push_str(&self.c("36;1", " │ "));
            // Cluster Telemetry Sidebar
            match cy {
                0 => s.push_str(&format!("Tier-2 AllReduce Ring: Step {:02}/16 (Optical Collective)", self.tier2_allreduce_step)),
                1 => s.push_str(&format!("Chip #00 Quorum: {:.1}% | Chip #15 Quorum: {:.1}%", self.chip_quorum_pct[0], self.chip_quorum_pct[15])),
                2 => s.push_str(&format!("Total Active Silicon Cores: 4,096 / 4,096 (100% Health)")),
                3 => s.push_str(&format!("Inter-Die Optical Latency: 42 ns across Echelle Grating")),
                _ => s.push_str(""),
            }
            s.push_str(&self.c("36;1", "  ║\n"));

            if cy < 3 {
                s.push_str(&self.c("36;1", "║     ║λ║         ║λ║         ║λ║         ║λ║      │                                           ║\n"));
            }
        }
        s
    }

    // ========================================================================
    // TAB 3: NoC Router & VC Buffer Congestion Heatmap
    // ========================================================================
    fn render_tab_router_heatmap(&self, _width: usize) -> String {
        let mut s = String::new();
        s.push_str(&self.c("36", "╟──────────────────────────────────────────────────────────────────────────────────────────╢\n"));
        s.push_str(&self.c("36;1", "║ "));
        s.push_str(&self.c("1;37", "9-PORT VIRTUAL CHANNEL ROUTER MICROARCHITECTURE & BUFFER CONGESTION HEATMAP                 "));
        s.push_str(&self.c("36;1", "║\n"));

        let port_names = ["+X (East)", "-X (West)", "+Y (North)", "-Y (South)", "+Z (Up)", "-Z (Down)", "+W (Hyper)", "-W (Ana)", "Local (PE)"];

        for port in 0..port_names.len().min(6) {
            s.push_str(&self.c("36;1", "║ "));
            s.push_str(&format!("{:<11} ", port_names[port]));
            s.push_str("VC0: ");
            s.push_str(&self.meter(self.router_buffer_occupancy[port][0], 8, 4));
            s.push_str(" VC1: ");
            s.push_str(&self.meter(self.router_buffer_occupancy[port][1], 8, 4));
            s.push_str(" VC2: ");
            s.push_str(&self.meter(self.router_buffer_occupancy[port][2], 8, 4));
            s.push_str(" VC3: ");
            s.push_str(&self.meter(self.router_buffer_occupancy[port][3], 8, 4));

            let total_flits: usize = self.router_buffer_occupancy[port].iter().sum();
            let hol_status = if total_flits >= 28 {
                self.c("31;1", " [HOL BLOCK] ")
            } else {
                self.c("32", " [FLOW OK]   ")
            };
            s.push_str(&hol_status);

            s.push_str(&self.c("36;1", "│ "));
            // Crossbar grant summary
            let granted_to = (0..9).find(|&out_p| self.crossbar_grants[port][out_p]);
            if let Some(target) = granted_to {
                s.push_str(&format!("Xbar Grant -> P{:02} ({:.1} GB/s) ", target, 40.0));
            } else {
                s.push_str("Xbar Grant -> None (Stall)  ");
            }
            s.push_str(&self.c("36;1", "║\n"));
        }

        s.push_str(&self.c("36;1", "║ Flow Control: Credit-Based VC Turn-Model (Deadlock-Free 4D-DOR) | Flit Queue Latency: 1.2 ns ║\n"));
        s
    }

    // ========================================================================
    // TAB 4: Swarm Synthesis & Vibe-Healing Telemetry
    // ========================================================================
    fn render_tab_swarm_telemetry(&self, _width: usize) -> String {
        let mut s = String::new();
        s.push_str(&self.c("36", "╟──────────────────────────────────────────────────────────────────────────────────────────╢\n"));
        s.push_str(&self.c("36;1", "║ "));
        s.push_str(&self.c("1;37", "AUTONOMOUS SWARM SELF-SYNTHESIS & CONTINUOUS SILICON VIBE-HEALING ENGINE                    "));
        s.push_str(&self.c("36;1", "║\n"));

        s.push_str(&self.c("36;1", "║ "));
        s.push_str(&format!("Target Workload: {}", self.c("32;1", &self.current_task)));
        s.push_str("  │ Status: ");
        s.push_str(&self.c("36;1", "Certified SSS+ Hazard-Free"));
        s.push_str(&self.c("36;1", " ║\n"));

        s.push_str(&self.c("36;1", "║ "));
        s.push_str(&format!("Vibe-Healing Repaired: {:03} AST Hazards | CRC-8 Mismatches Resolved: {:02} | NOP Compacted: 42% ",
            self.total_healed_faults * 2, self.total_healed_faults));
        s.push_str(&self.c("36;1", "║\n"));

        s.push_str(&self.c("36;1", "║ "));
        s.push_str(&format!("Consensus Quorum: {:02}/16 Chips (100.0%) | Swarm Consensus: {} | JIT Traps: 0  ",
            16, self.c("32;1", "PASSED")));
        s.push_str(&self.c("36;1", "║\n"));

        if let Some(ref fault) = self.last_fault_msg {
            s.push_str(&self.c("36;1", "║ "));
            s.push_str(&self.c("33;1", &format!("Last Real-Time Healing Event: {}", fault)));
            s.push_str(&self.c("36;1", " ║\n"));
        } else {
            s.push_str(&self.c("36;1", "║ Last Real-Time Healing Event: No uncorrected single-event upsets. Parity 100% Clean.       ║\n"));
        }
        s
    }

    // ========================================================================
    // Bottom Log Ticker & Hotkeys Bar
    // ========================================================================
    fn render_bottom_status_bar(&self, width: usize) -> String {
        let mut s = String::new();
        s.push_str(&self.c("36", "╟──────────────────────────────────────────────────────────────────────────────────────────╢\n"));

        // Recent log event
        let last_log = self.recent_logs.back().cloned().unwrap_or_else(|| "Swarm idle.".to_string());
        s.push_str(&self.c("36;1", "║ "));
        s.push_str(&self.c("90", "LOG: "));
        s.push_str(&self.c("37", &format!("{:<80}", last_log)));
        s.push_str(&self.c("36;1", " ║\n"));

        // Hotkeys help
        let hotkeys = "[1..4] Tabs  [z/w] Torus Dim  [Space] Pause  [s] Step  [c] Inject Fault  [p] Packet  [q] Quit";
        s.push_str(&self.c("36;1", "╚═"));
        s.push_str(&self.c("1;33", hotkeys));
        let used = 2 + hotkeys.len() + 2;
        let pad = if width > used { width - used } else { 2 };
        for _ in 0..pad {
            s.push_str("═");
        }
        s.push_str(&self.c("36;1", "╝"));
        s
    }

    // ========================================================================
    // Headless Snapshot & Telemetry Export
    // ========================================================================

    /// Advances simulation by `ticks` cycles and renders a single frame snapshot
    pub fn render_snapshot(&mut self, ticks: usize) -> String {
        for _ in 0..ticks {
            self.step_tick();
        }
        self.render_frame(90, 24)
    }

    /// Serializes live state into JSON string
    pub fn telemetry_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"tick_count\": {},\n", self.tick_count));
        json.push_str(&format!("  \"is_paused\": {},\n", self.is_paused));
        json.push_str(&format!("  \"simulated_ipc\": {:.3},\n", self.simulated_ipc));
        json.push_str(&format!("  \"simulated_temp_c\": {:.2},\n", self.simulated_temp_c));
        json.push_str(&format!("  \"simulated_energy_pj\": {:.4},\n", self.simulated_energy_pj));
        json.push_str(&format!("  \"total_packets_routed\": {},\n", self.total_packets_routed));
        json.push_str(&format!("  \"total_healed_faults\": {},\n", self.total_healed_faults));
        json.push_str(&format!("  \"active_view\": \"{}\",\n", self.config.active_view.name()));
        json.push_str(&format!("  \"selected_z\": {},\n", self.config.selected_z));
        json.push_str(&format!("  \"selected_w\": {},\n", self.config.selected_w));
        json.push_str(&format!("  \"selected_chip\": {},\n", self.config.selected_chip));
        json.push_str(&format!("  \"consensus_reached\": {},\n", self.consensus_reached));
        json.push_str(&format!("  \"packets_in_flight\": {},\n", self.packets.len()));
        json.push_str("  \"chip_traffic_gbps\": [\n");
        for (i, val) in self.chip_traffic_gbps.iter().enumerate() {
            json.push_str(&format!("    {:.2}{}\n", val, if i + 1 < 16 { "," } else { "" }));
        }
        json.push_str("  ]\n");
        json.push_str("}\n");
        json
    }
}
