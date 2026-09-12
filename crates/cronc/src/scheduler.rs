// ============================================================================
// CRON AOT Hazard Scheduler (Milestone #178)
// Eliminates pipeline bubbles and data stalls before emitting machine bundles.
// Builds a Register Dependency DAG (RAW, WAW, WAR, Structural, Barrier)
// and schedules instructions into 4-wide VLIW bundles using List Scheduling.
// ============================================================================

use std::collections::HashSet;
use crate::codegen::{VliwBundle, VliwSlot};

#[derive(Debug, Clone)]
pub struct IRInstruction {
    pub raw_slot: String,
    pub dest: Option<usize>,
    pub srcs: HashSet<usize>,
    pub is_optical: bool,
    pub is_barrier: bool,
}

impl IRInstruction {
    pub fn from_slot(slot: &VliwSlot) -> Self {
        Self::from_raw(&slot.raw)
    }

    pub fn from_raw(raw: &str) -> Self {
        let trimmed = raw.trim();
        let prefix = trimmed.chars().next().unwrap_or('_');
        let op = if trimmed.len() >= 3 { &trimmed[1..3] } else { "NO" };
        let mode = trimmed.chars().nth(5).unwrap_or('$');

        let is_nop = op == "NO" || trimmed.starts_with("'.........");
        let is_barrier = op == "HL"
            || op == "SW"
            || op == "YD"
            || op == "RS"
            || op == "DW"
            || (op == "RC" && mode == '!')
            || op == "RT";

        let is_optical = op == "OP" || op == "WD";

        let is_writer = !is_nop
            && op != "SB"
            && op != "SH"
            && op != "RS"
            && op != "HL"
            && op != "DW"
            && op != "YD"
            && op != "PT"
            && (op != "RC" || mode == 'C');

        let dest = if is_writer && trimmed.len() >= 5 {
            usize::from_str_radix(&trimmed[3..5], 16).ok()
        } else {
            None
        };

        let mut srcs = HashSet::new();

        if !is_nop {
            // Source register 1 from slot index 6
            if trimmed.len() >= 7 {
                if let Some(src1) = trimmed[6..7].chars().next().and_then(|c| c.to_digit(16)) {
                    if src1 > 0 && mode == '$' {
                        srcs.insert(src1 as usize);
                    }
                }
            }

            // If binary op, dest is also read as the accumulator / left operand
            if (op == "PO"
                || op == "MD"
                || op == "TL"
                || op == "PK"
                || op == "RF"
                || op == "TO"
                || op == "FA"
                || op == "BK"
                || op == "BL")
                && prefix == '_'
            {
                if let Some(d) = dest {
                    if d > 0 {
                        srcs.insert(d);
                    }
                }
            }

            // Broadcast or check reads dest
            if (op == "SB" || op == "SW") && trimmed.len() >= 5 {
                if let Ok(d) = usize::from_str_radix(&trimmed[3..5], 16) {
                    if d > 0 {
                        srcs.insert(d);
                    }
                }
            }
        }

        Self {
            raw_slot: trimmed.to_string(),
            dest,
            srcs,
            is_optical,
            is_barrier,
        }
    }
}

pub struct AOTHazardScheduler {
    pub nop_slot: String,
}

impl Default for AOTHazardScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl AOTHazardScheduler {
    pub fn new() -> Self {
        Self {
            nop_slot: "_NO00#000>".to_string(),
        }
    }

    /// Schedule a slice of raw slot strings into 4-wide VLIW bundles
    pub fn schedule_raw_slots(&self, raw_slots: &[String], start_cycle: usize) -> Vec<VliwBundle> {
        let ir_list: Vec<IRInstruction> = raw_slots.iter().map(|s| IRInstruction::from_raw(s)).collect();
        self.schedule_ir(&ir_list, start_cycle)
    }

    /// Schedule a slice of VliwSlots into 4-wide VLIW bundles
    pub fn schedule_slots(&self, slots: &[VliwSlot], start_cycle: usize) -> Vec<VliwBundle> {
        let ir_list: Vec<IRInstruction> = slots.iter().map(IRInstruction::from_slot).collect();
        self.schedule_ir(&ir_list, start_cycle)
    }

    /// Core List Scheduling algorithm with DAG dependency analysis
    pub fn schedule_ir(&self, instructions: &[IRInstruction], start_cycle: usize) -> Vec<VliwBundle> {
        let n = instructions.len();
        if n == 0 {
            return Vec::new();
        }

        // 1. Build Dependency DAG
        let mut dep_count = vec![0usize; n];
        let mut dependents: Vec<Vec<usize>> = vec![Vec::new(); n];

        for j in 0..n {
            let inst_j = &instructions[j];
            for i in 0..j {
                let inst_i = &instructions[i];

                let mut has_dep = false;

                // RAW: i writes R, j reads R
                if let Some(w_i) = inst_i.dest {
                    if inst_j.srcs.contains(&w_i) {
                        has_dep = true;
                    }
                }

                // WAW: i writes R, j writes R
                if let (Some(w_i), Some(w_j)) = (inst_i.dest, inst_j.dest) {
                    if w_i == w_j {
                        has_dep = true;
                    }
                }

                // WAR: i reads R, j writes R
                if let Some(w_j) = inst_j.dest {
                    if inst_i.srcs.contains(&w_j) {
                        has_dep = true;
                    }
                }

                // Control / memory barrier: instructions AFTER a barrier cannot run until barrier finishes
                if inst_i.is_barrier {
                    has_dep = true;
                }

                if has_dep {
                    dep_count[j] += 1;
                    dependents[i].push(j);
                }
            }
        }

        // 2. Ready pool: instructions with 0 dependencies
        let mut ready_pool: Vec<usize> = (0..n).filter(|&idx| dep_count[idx] == 0).collect();
        let mut scheduled = vec![false; n];
        let mut bundles = Vec::new();
        let mut cycle = start_cycle;

        while !ready_pool.is_empty() {
            let mut current_slots: Vec<String> = Vec::with_capacity(4);
            let mut current_cycle_writes = HashSet::new();
            let mut current_cycle_reads = HashSet::new();
            let mut optical_in_cycle = false;
            let mut scheduled_in_this_cycle = Vec::new();

            for _ in 0..4 {
                // Find first candidate in ready_pool that doesn't violate intra-bundle hazards
                let mut candidate_pos = None;
                for (pos, &idx) in ready_pool.iter().enumerate() {
                    let inst = &instructions[idx];

                    // RAW in current cycle: cannot read register written in same cycle
                    let raw_hazard = inst.srcs.iter().any(|r| current_cycle_writes.contains(r));

                    // WAW in current cycle: cannot write register already written in same cycle
                    let waw_hazard = inst.dest.map_or(false, |d| current_cycle_writes.contains(&d));

                    // WAR in current cycle: cannot write register already read in same cycle
                    let war_hazard = inst.dest.map_or(false, |d| current_cycle_reads.contains(&d));

                    // Structural: max 1 optical op per cycle
                    let optical_hazard = inst.is_optical && optical_in_cycle;

                    if !raw_hazard && !waw_hazard && !war_hazard && !optical_hazard {
                        candidate_pos = Some(pos);
                        break;
                    }
                }

                if let Some(pos) = candidate_pos {
                    let idx = ready_pool.remove(pos);
                    let inst = &instructions[idx];

                    current_slots.push(inst.raw_slot.clone());
                    if let Some(d) = inst.dest {
                        current_cycle_writes.insert(d);
                    }
                    for &s in &inst.srcs {
                        current_cycle_reads.insert(s);
                    }
                    if inst.is_optical {
                        optical_in_cycle = true;
                    }

                    scheduled[idx] = true;
                    scheduled_in_this_cycle.push(idx);

                    // If barrier, terminate current bundle early
                    if inst.is_barrier {
                        break;
                    }
                } else {
                    // No ready instruction can fit in this cycle
                    break;
                }
            }

            // Pad remainder with NOPs
            while current_slots.len() < 4 {
                current_slots.push(self.nop_slot.clone());
            }

            bundles.push(VliwBundle {
                cycle,
                slots: [
                    VliwSlot::new(&current_slots[0]),
                    VliwSlot::new(&current_slots[1]),
                    VliwSlot::new(&current_slots[2]),
                    VliwSlot::new(&current_slots[3]),
                ],
            });
            cycle += 1;

            // 3. Resolve dependencies for newly scheduled instructions
            for &idx in &scheduled_in_this_cycle {
                for &succ in &dependents[idx] {
                    dep_count[succ] = dep_count[succ].saturating_sub(1);
                    if dep_count[succ] == 0 && !scheduled[succ] && !ready_pool.contains(&succ) {
                        ready_pool.push(succ);
                    }
                }
            }

            // Sort ready pool by original index to preserve program order preference
            ready_pool.sort_unstable();

            // Safety fallback against circular deadlocks (should never occur in DAG)
            if ready_pool.is_empty() && scheduled.iter().any(|&s| !s) {
                if let Some(first_unscheduled) = scheduled.iter().position(|&s| !s) {
                    ready_pool.push(first_unscheduled);
                }
            }
        }

        bundles
    }
}
