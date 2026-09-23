// ============================================================================
// CRON Low-Level Machine Language (.cl) VLIW Super-Optimizer
// Architecture: 256-Core 4D-Torus Heterogeneous Neuromorphic/Photonic Silicon
//
// Features:
//   1. Data Dependency DAG Construction (RAW, WAR, WAW & Structural Constraints)
//   2. Critical-Path List Scheduling (ASAP/ALAP Prioritization, IPC -> 4.0)
//   3. Silicon Heterogeneous Way Assignment (Optical, Sub-byte, ALU, NoC)
//   4. Algebraic Peephole Optimization & Reversible Involution (RF * RF = I)
//   5. Dead Bundle & Redundant NOP Elimination
//
// 100% Pure Rust - Zero External Dependencies.
// ============================================================================

use std::collections::HashSet;
use crate::cl_heal::{heal_slot_crc, CANONICAL_NOP};
use crate::cl_lang::{parse_slot, ClSlot};

/// Super-optimization engine configuration levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClOptLevel {
    Level1, // Greedy in-order hazard packer
    #[default]
    Level2, // DAG critical-path list scheduler with peephole & silicon way placement
}

/// Detailed configuration for VLIW super-optimization
#[derive(Debug, Clone)]
pub struct ClOptConfig {
    pub level: ClOptLevel,
    pub enable_peephole: bool,
    pub enable_fusion: bool,
    pub target_ipc: f64,
}

impl Default for ClOptConfig {
    fn default() -> Self {
        Self {
            level: ClOptLevel::Level2,
            enable_peephole: true,
            enable_fusion: true,
            target_ipc: 4.0,
        }
    }
}

/// Comprehensive telemetry report produced by the Super-Optimizer
#[derive(Debug, Clone, Default)]
pub struct ClOptReport {
    pub original_bundles: usize,
    pub optimized_bundles: usize,
    pub compacted_slots: usize,
    pub original_ipc: f64,
    pub optimized_ipc: f64,
    pub speedup_percentage: f64,
    pub peephole_rewrites_count: usize,
    pub critical_path_depth: usize,
    pub optimized_code: String,
}

/// Node in the Instruction Dependency Directed Acyclic Graph (DAG)
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ClDagNode {
    id: usize,
    raw: String,
    slot: ClSlot,
    read_regs: Vec<usize>,
    write_regs: Vec<usize>,
    parents: Vec<usize>,
    children: Vec<usize>,
    critical_path_depth: usize,
    preferred_way: usize,
    is_optical: bool,
    is_halt: bool,
    is_barrier: bool,
}

/// Default entry point: Super-optimizes .cl machine code with Level 2 DAG scheduler
pub fn optimize_cl_program(source: &str) -> Result<ClOptReport, String> {
    optimize_cl_program_advanced(source, &ClOptConfig::default())
}

/// Advanced entry point with user-configurable optimization levels
pub fn optimize_cl_program_advanced(source: &str, config: &ClOptConfig) -> Result<ClOptReport, String> {
    let mut raw_slots: Vec<String> = Vec::new();
    let mut total_input_bundles = 0;
    let mut _total_input_slots = 0;
    let mut active_input_slots = 0;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with("//") {
            continue;
        }

        if let Some((_hdr, slots_part)) = trimmed.split_once(':') {
            total_input_bundles += 1;
            for slot_str in slots_part.split_whitespace() {
                _total_input_slots += 1;
                let (healed, _) = heal_slot_crc(slot_str);
                if healed != CANONICAL_NOP && !healed.starts_with("_NO") {
                    raw_slots.push(healed);
                    active_input_slots += 1;
                }
            }
        }
    }

    if total_input_bundles == 0 && raw_slots.is_empty() {
        return Err("Cannot optimize empty .cl source".to_string());
    }

    let original_ipc = if total_input_bundles > 0 {
        active_input_slots as f64 / total_input_bundles as f64
    } else {
        0.0
    };

    match config.level {
        ClOptLevel::Level1 => optimize_level1_greedy(source, raw_slots, total_input_bundles, active_input_slots, original_ipc),
        ClOptLevel::Level2 => optimize_level2_dag(source, raw_slots, total_input_bundles, active_input_slots, original_ipc, config),
    }
}

/// Level 1: Fast In-Order Greedy Hazard Packer
fn optimize_level1_greedy(
    source: &str,
    slots: Vec<String>,
    total_input_bundles: usize,
    active_input_slots: usize,
    original_ipc: f64,
) -> Result<ClOptReport, String> {
    let mut optimized_bundles: Vec<Vec<String>> = Vec::new();
    let mut current_bundle: Vec<String> = Vec::with_capacity(4);
    let mut current_dest_regs: HashSet<usize> = HashSet::new();
    let mut current_src_regs: HashSet<usize> = HashSet::new();
    let mut has_optical_in_current = false;

    for slot_str in slots {
        if let Ok(slot) = parse_slot(&slot_str) {
            let mut can_pack = current_bundle.len() < 4;

            if slot.opcode == "OP" && has_optical_in_current {
                can_pack = false;
            }

            if let Some(src) = slot.src_reg {
                if src > 0 && current_dest_regs.contains(&src) {
                    can_pack = false;
                }
            }

            if let Some(dest) = slot.dest_reg {
                if dest > 0 && current_dest_regs.contains(&dest) {
                    can_pack = false;
                }
                if dest > 0 && current_src_regs.contains(&dest) {
                    can_pack = false;
                }
            }

            if can_pack {
                if let Some(d) = slot.dest_reg {
                    if d > 0 {
                        current_dest_regs.insert(d);
                    }
                }
                if let Some(s) = slot.src_reg {
                    if s > 0 {
                        current_src_regs.insert(s);
                    }
                }
                if slot.opcode == "OP" {
                    has_optical_in_current = true;
                }
                current_bundle.push(slot_str);
            } else {
                while current_bundle.len() < 4 {
                    current_bundle.push(CANONICAL_NOP.to_string());
                }
                optimized_bundles.push(current_bundle);

                current_bundle = Vec::with_capacity(4);
                current_dest_regs.clear();
                current_src_regs.clear();
                has_optical_in_current = false;

                if let Some(d) = slot.dest_reg {
                    if d > 0 {
                        current_dest_regs.insert(d);
                    }
                }
                if let Some(s) = slot.src_reg {
                    if s > 0 {
                        current_src_regs.insert(s);
                    }
                }
                if slot.opcode == "OP" {
                    has_optical_in_current = true;
                }
                current_bundle.push(slot_str);
            }
        }
    }

    if !current_bundle.is_empty() {
        while current_bundle.len() < 4 {
            current_bundle.push(CANONICAL_NOP.to_string());
        }
        optimized_bundles.push(current_bundle);
    }

    finalize_report(source, optimized_bundles, total_input_bundles, active_input_slots, original_ipc, 0, 1)
}

/// Level 2: Advanced DAG-Based Critical-Path List Scheduler with Peephole Rewrites
fn optimize_level2_dag(
    source: &str,
    mut slots: Vec<String>,
    total_input_bundles: usize,
    _active_input_slots: usize,
    original_ipc: f64,
    config: &ClOptConfig,
) -> Result<ClOptReport, String> {
    let mut peephole_count = 0;

    // 1. Algebraic Peephole Rewrites (Involution cancellation, dead immediate folds)
    if config.enable_peephole {
        let (rewritten_slots, rewrites) = apply_algebraic_peephole(&slots);
        slots = rewritten_slots;
        peephole_count = rewrites;
    }

    let active_slots = slots.len();

    // 2. Build Dependency DAG
    let mut dag_nodes = build_dependency_dag(&slots)?;

    // 3. Critical-Path Analysis
    let max_depth = calculate_critical_paths(&mut dag_nodes);

    // 4. Critical-Path List Scheduling with Heterogeneous Way Placement
    let scheduled_bundles = schedule_dag_list(&dag_nodes);

    finalize_report(
        source,
        scheduled_bundles,
        total_input_bundles,
        active_slots,
        original_ipc,
        peephole_count,
        max_depth,
    )
}

/// Applies algebraic peephole rewrites such as reversible involution ($RF \cdot RF = I$)
fn apply_algebraic_peephole(slots: &[String]) -> (Vec<String>, usize) {
    let mut result: Vec<String> = Vec::with_capacity(slots.len());
    let mut rewrites = 0;
    let mut i = 0;

    while i < slots.len() {
        if let Ok(slot) = parse_slot(&slots[i]) {
            // Reversible Fredkin Involution: Consecutive RF gates on identical registers cancel out
            if slot.opcode == "RF" && i + 1 < slots.len() {
                if let Ok(next_slot) = parse_slot(&slots[i + 1]) {
                    if next_slot.opcode == "RF"
                        && slot.dest_reg == next_slot.dest_reg
                        && slot.src_reg == next_slot.src_reg
                    {
                        // Both cancel out completely (0 entropy loss identity)
                        rewrites += 2;
                        i += 2;
                        continue;
                    }
                }
            }

            // Redundant identity move elimination: PS (Parallel Prefix-Sum or reg-to-reg copy) where dest == src
            if slot.opcode == "PS" && slot.dest_reg == slot.src_reg && slot.dest_reg.is_some() {
                rewrites += 1;
                i += 1;
                continue;
            }

            // Dead immediate overwrite: two consecutive immediate loads to the same register
            if (slot.prefix == '\'' || slot.opcode.starts_with('=')) && i + 1 < slots.len() {
                if let Ok(next_slot) = parse_slot(&slots[i + 1]) {
                    if (next_slot.prefix == '\'' || next_slot.opcode.starts_with('='))
                        && slot.dest_reg == next_slot.dest_reg
                        && slot.dest_reg.is_some()
                    {
                        // First load is completely dead; drop it
                        rewrites += 1;
                        i += 1;
                        continue;
                    }
                }
            }
        }

        result.push(slots[i].clone());
        i += 1;
    }

    (result, rewrites)
}

/// Constructs the Data Dependency DAG across all active slots
fn build_dependency_dag(slots: &[String]) -> Result<Vec<ClDagNode>, String> {
    let mut nodes: Vec<ClDagNode> = Vec::with_capacity(slots.len());

    for (idx, slot_str) in slots.iter().enumerate() {
        let slot = parse_slot(slot_str)?;

        let mut read_regs = Vec::new();
        let mut write_regs = Vec::new();

        let d = slot.dest_reg.unwrap_or(0);
        let s = slot.src_reg.unwrap_or(0);
        let imm_nibble = slot.imm_token.to_digit(16).unwrap_or(0) as usize;

        // Determine read and write registers per opcode semantics
        match slot.opcode.as_str() {
            "RF" => {
                if d > 0 { read_regs.push(d); write_regs.push(d); }
                if s > 0 { read_regs.push(s); write_regs.push(s); }
            }
            "TO" => {
                read_regs.push(10);
                read_regs.push(11);
                if d > 0 { read_regs.push(d); write_regs.push(d); }
            }
            "PO" | "P0" | "P1" => {
                if imm_nibble > 0 && imm_nibble < 16 && s > 0 {
                    read_regs.push(s);
                    read_regs.push(imm_nibble);
                } else {
                    if d > 0 { read_regs.push(d); }
                    if s > 0 { read_regs.push(s); }
                }
                if d > 0 { write_regs.push(d); }
            }
            "MD" => {
                if d > 0 { read_regs.push(d); write_regs.push(d); }
                if s > 0 { read_regs.push(s); }
            }
            "FA" | "TT" | "PS" => {
                if s > 0 { read_regs.push(s); }
                if d > 0 { write_regs.push(d); }
            }
            "LI" | "LF" => {
                if s > 0 { read_regs.push(s); }
                if d > 0 { write_regs.push(d); }
            }
            "BK" | "ST" | "RX" | "PK" | "TL" => {
                if d > 0 { write_regs.push(d); }
            }
            "OP" | "WD" => {
                if s > 0 { read_regs.push(s); }
                if d > 0 { write_regs.push(d); }
            }
            "WH" => {
                if s > 0 { read_regs.push(s); }
                if d > 0 { write_regs.push(d); }
            }
            "TX" | "SB" | "aa" => {
                if s > 0 { read_regs.push(s); }
            }
            "RS" | "88" => {
                write_regs.push(1);
            }
            op if op.starts_with('=') || slot.prefix == '\'' => {
                if d > 0 { write_regs.push(d); }
            }
            _ => {
                if d > 0 && slot.dest_reg.is_some() {
                    write_regs.push(d);
                }
            }
        }

        // Heterogeneous Functional Way Assignment:
        // Way 0: Optical GEMM & Autodiff
        // Way 1: Sub-byte MAC & Reversible Gates
        // Way 2: SIMD ALU & Neuromorphic STDP
        // Way 3: Spatial NoC & Control/Barriers
        let preferred_way = match slot.opcode.as_str() {
            "OP" | "WD" | "FA" | "TT" => 0,
            "MD" | "PK" | "RF" | "TO" | "BK" => 1,
            "PO" | "P0" | "P1" | "ST" | "LI" | "LF" | "SY" => 2,
            "TX" | "RX" | "SB" | "WH" | "bb" | "HL" | "JP" | "BZ" | "BN" => 3,
            _ => idx % 4, // Immediate loads and generic ops can use any available way
        };

        let is_optical = slot.opcode == "OP" || slot.opcode == "WD";
        let is_halt = slot.opcode == "HL";
        let is_barrier = slot.opcode == "bb";

        nodes.push(ClDagNode {
            id: idx,
            raw: slot_str.clone(),
            slot,
            read_regs,
            write_regs,
            parents: Vec::new(),
            children: Vec::new(),
            critical_path_depth: 1,
            preferred_way,
            is_optical,
            is_halt,
            is_barrier,
        });
    }

    // Connect DAG edges based on RAW, WAR, WAW, and Control dependencies
    for i in 0..nodes.len() {
        for j in 0..i {
            let mut has_dep = false;

            // 1. RAW: j writes what i reads
            for &w in &nodes[j].write_regs {
                if nodes[i].read_regs.contains(&w) {
                    has_dep = true;
                    break;
                }
            }

            // 2. WAR: j reads what i writes (cannot reorder i before j)
            if !has_dep {
                for &r in &nodes[j].read_regs {
                    if nodes[i].write_regs.contains(&r) {
                        has_dep = true;
                        break;
                    }
                }
            }

            // 3. WAW: j writes what i writes
            if !has_dep {
                for &w in &nodes[j].write_regs {
                    if nodes[i].write_regs.contains(&w) {
                        has_dep = true;
                        break;
                    }
                }
            }

            // 4. Control / Barrier: Halt and global barrier serialize dependencies
            if !has_dep && (nodes[j].is_halt || nodes[j].is_barrier || nodes[i].is_halt || nodes[i].is_barrier) {
                has_dep = true;
            }

            if has_dep {
                nodes[i].parents.push(j);
                nodes[j].children.push(i);
            }
        }
    }

    Ok(nodes)
}

/// Recursively computes critical path depth for each node in the DAG
fn calculate_critical_paths(nodes: &mut [ClDagNode]) -> usize {
    let n = nodes.len();
    let mut max_global_depth = 1;

    // Traverse in reverse topological order (sinks to roots)
    for i in (0..n).rev() {
        let mut child_max = 0;
        for &child_id in &nodes[i].children {
            if nodes[child_id].critical_path_depth > child_max {
                child_max = nodes[child_id].critical_path_depth;
            }
        }
        nodes[i].critical_path_depth = 1 + child_max;
        if nodes[i].critical_path_depth > max_global_depth {
            max_global_depth = nodes[i].critical_path_depth;
        }
    }

    max_global_depth
}

/// List Scheduler: Fills 4-slot VLIW bundles prioritizing critical-path depth and heterogeneous silicon ways
fn schedule_dag_list(nodes: &[ClDagNode]) -> Vec<Vec<String>> {
    let n = nodes.len();
    if n == 0 {
        return Vec::new();
    }

    let mut scheduled = vec![false; n];
    let mut completed = vec![false; n];
    let mut in_degrees: Vec<usize> = nodes.iter().map(|node| node.parents.len()).collect();

    let mut bundles: Vec<Vec<String>> = Vec::new();

    while scheduled.iter().any(|&s| !s) {
        // Collect ready candidates (all parents completed in earlier cycles)
        let mut ready: Vec<usize> = (0..n)
            .filter(|&i| !scheduled[i] && in_degrees[i] == 0)
            .collect();

        // Sort ready candidates by critical_path_depth descending, then id ascending
        ready.sort_by(|&a, &b| {
            nodes[b].critical_path_depth
                .cmp(&nodes[a].critical_path_depth)
                .then_with(|| a.cmp(&b))
        });

        // Current cycle VLIW bundle: 4 slots
        let mut current_bundle: [Option<String>; 4] = [None, None, None, None];
        let mut cycle_scheduled_nodes: Vec<usize> = Vec::new();
        let mut current_dest_regs: HashSet<usize> = HashSet::new();
        let mut current_src_regs: HashSet<usize> = HashSet::new();
        let mut has_optical_in_current = false;
        let mut halt_scheduled = false;

        for &node_id in &ready {
            let node = &nodes[node_id];

            // Optical structural constraint: max 1 optical op per cycle per core
            if node.is_optical && has_optical_in_current {
                continue;
            }

            // Halt constraint: only schedule if all non-halt ready work is done or only halt remains
            if node.is_halt && ready.len() > 1 && !ready.iter().all(|&id| nodes[id].is_halt) {
                continue;
            }

            // Intra-cycle RAW hazard: cannot read a register written in the same cycle
            let mut hazard = false;
            for &r in &node.read_regs {
                if current_dest_regs.contains(&r) {
                    hazard = true;
                    break;
                }
            }

            // Intra-cycle WAW hazard: cannot write to the same register twice in same cycle
            if !hazard {
                for &w in &node.write_regs {
                    if current_dest_regs.contains(&w) {
                        hazard = true;
                        break;
                    }
                }
            }

            // Intra-cycle WAR hazard: cannot overwrite a register being read in the same cycle
            if !hazard {
                for &w in &node.write_regs {
                    if current_src_regs.contains(&w) {
                        hazard = true;
                        break;
                    }
                }
            }

            if hazard {
                continue;
            }

            // Find slot in bundle: try preferred_way first, otherwise first available vacant slot
            let target_slot = if node.is_halt {
                // Halt prefers slot 3
                if current_bundle[3].is_none() {
                    Some(3)
                } else {
                    (0..4).find(|&s| current_bundle[s].is_none())
                }
            } else if current_bundle[node.preferred_way].is_none() {
                Some(node.preferred_way)
            } else {
                (0..4).find(|&s| current_bundle[s].is_none())
            };

            if let Some(slot_idx) = target_slot {
                current_bundle[slot_idx] = Some(node.raw.clone());
                scheduled[node_id] = true;
                cycle_scheduled_nodes.push(node_id);

                for &w in &node.write_regs {
                    current_dest_regs.insert(w);
                }
                for &r in &node.read_regs {
                    current_src_regs.insert(r);
                }
                if node.is_optical {
                    has_optical_in_current = true;
                }
                if node.is_halt {
                    halt_scheduled = true;
                }

                // If all 4 slots are filled, bundle is full
                if current_bundle.iter().all(|s| s.is_some()) {
                    break;
                }
            }
        }

        // If no ready node could be scheduled due to hazards, advance
        if cycle_scheduled_nodes.is_empty() {
            // Force schedule first ready node to guarantee progress
            if let Some(&forced_id) = ready.first() {
                let node = &nodes[forced_id];
                current_bundle[0] = Some(node.raw.clone());
                scheduled[forced_id] = true;
                cycle_scheduled_nodes.push(forced_id);
                if node.is_halt {
                    halt_scheduled = true;
                }
            }
        }

        // Co-issue halt in slot 3 of closing bundle if all its parents are satisfied
        if !halt_scheduled {
            for (node_id, node) in nodes.iter().enumerate() {
                if !scheduled[node_id] && node.is_halt {
                    let all_parents_satisfied = node.parents.iter().all(|&p| completed[p] || cycle_scheduled_nodes.contains(&p));
                    if all_parents_satisfied {
                        let slot_idx = if current_bundle[3].is_none() {
                            Some(3)
                        } else {
                            (0..4).rposition(|s| current_bundle[s].is_none())
                        };

                        if let Some(idx) = slot_idx {
                            current_bundle[idx] = Some(node.raw.clone());
                            scheduled[node_id] = true;
                            cycle_scheduled_nodes.push(node_id);
                            halt_scheduled = true;
                            break;
                        }
                    }
                }
            }
        }

        // Pad empty slots with CANONICAL_NOP
        let mut final_bundle_slots = Vec::with_capacity(4);
        for slot in &current_bundle {
            match slot {
                Some(ref s) => final_bundle_slots.push(s.clone()),
                None => final_bundle_slots.push(CANONICAL_NOP.to_string()),
            }
        }
        bundles.push(final_bundle_slots);

        // Mark cycle nodes as completed and update in-degrees of children
        for &node_id in &cycle_scheduled_nodes {
            completed[node_id] = true;
            for &child_id in &nodes[node_id].children {
                in_degrees[child_id] = in_degrees[child_id].saturating_sub(1);
            }
        }

        if halt_scheduled && scheduled.iter().all(|&s| s) {
            break;
        }
    }

    bundles
}

/// Finalizes the report and formats the optimized .cl source code
fn finalize_report(
    source: &str,
    bundles: Vec<Vec<String>>,
    total_input_bundles: usize,
    active_slots: usize,
    original_ipc: f64,
    peephole_rewrites_count: usize,
    critical_path_depth: usize,
) -> Result<ClOptReport, String> {
    let final_bundles = bundles.len().max(1);
    let optimized_ipc = active_slots as f64 / final_bundles as f64;
    let speedup = if final_bundles < total_input_bundles {
        ((total_input_bundles as f64 - final_bundles as f64) / total_input_bundles as f64) * 100.0
    } else {
        0.0
    };

    let mut optimized_code = String::with_capacity(source.len() + 1024);
    optimized_code.push_str("; ============================================================\n");
    optimized_code.push_str("; CRON Super-Optimized VLIW Machine Code (.cl)\n");
    optimized_code.push_str(&format!(
        "; Optimized: {} bundles -> {} bundles (Speedup: +{:.1}% | IPC: {:.2} -> {:.2})\n",
        total_input_bundles, final_bundles, speedup, original_ipc, optimized_ipc
    ));
    optimized_code.push_str(&format!(
        "; Critical Path Depth: {} cycles | Peephole Rewrites: {} | Active Ops: {}\n",
        critical_path_depth, peephole_rewrites_count, active_slots
    ));
    optimized_code.push_str("; ============================================================\n\n");

    for (cycle, b) in bundles.iter().enumerate() {
        optimized_code.push_str(&format!(
            "B{:04}: {} {} {} {}\n",
            cycle, b[0], b[1], b[2], b[3]
        ));
    }

    Ok(ClOptReport {
        original_bundles: total_input_bundles,
        optimized_bundles: final_bundles,
        compacted_slots: active_slots,
        original_ipc,
        optimized_ipc,
        speedup_percentage: speedup,
        peephole_rewrites_count,
        critical_path_depth,
        optimized_code,
    })
}
