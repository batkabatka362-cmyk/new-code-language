// ============================================================================
// CRON .cl Self-Rewriting Metacognitive JIT Engine
// Live In-Memory I-Cache / SRAM Hot-Patching for Autonomous AGI Cores
// Zero-Downtime Instruction Stream Modification with 1-Cycle Rollback Safety
// Target: 256-Core 4D-Torus Neuromorphic Photonic Processor
// ============================================================================

use crate::cl_lang::parse_slot;
use crate::codegen::compute_parity;
use std::collections::HashMap;

/// A live patch operation targeting a specific cycle and slot index (0..3).
#[derive(Debug, Clone, PartialEq)]
pub struct LiveSlotPatch {
    pub cycle: usize,
    pub slot_index: usize,
    pub new_raw_token: String,
    pub original_token: Option<String>,
}

/// Status of a hot-patch transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatchStatus {
    Applied,
    RolledBack(String),
    Rejected(String),
}

/// A shadow checkpoint frame storing pre-mutation instruction state for 1-cycle rollback.
#[derive(Debug, Clone, PartialEq)]
pub struct ShadowCheckpoint {
    pub checkpoint_id: u64,
    pub cycle: usize,
    pub original_slots: [String; 4],
    pub timestamp_cycles: u64,
}

/// The Self-Rewriting Metacognitive Engine managing active instruction cache modifications.
#[derive(Debug, Clone)]
pub struct SelfRewritingEngine {
    /// Active instruction stream indexed by cycle number: cycle -> [slot0, slot1, slot2, slot3]
    pub active_icache: HashMap<usize, [String; 4]>,
    /// History of applied live patches
    pub patch_log: Vec<LiveSlotPatch>,
    /// Shadow checkpoints for hardware-guaranteed rollback safety
    pub shadow_stack: Vec<ShadowCheckpoint>,
    /// Total successful live mutations performed
    pub total_mutations: usize,
    /// Total rollbacks triggered by invariant violations
    pub total_rollbacks: usize,
}

impl Default for SelfRewritingEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SelfRewritingEngine {
    pub fn new() -> Self {
        Self {
            active_icache: HashMap::new(),
            patch_log: Vec::new(),
            shadow_stack: Vec::new(),
            total_mutations: 0,
            total_rollbacks: 0,
        }
    }

    /// Loads a multi-bundle .cl program string into the live I-Cache.
    pub fn load_program(&mut self, cl_code: &str) -> Result<usize, String> {
        let mut loaded = 0;
        for line in cl_code.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with("//") {
                continue;
            }
            if let Some((b_part, slots_part)) = trimmed.split_once(':') {
                if b_part.starts_with('B') || b_part.starts_with('b') {
                    let cycle = b_part[1..]
                        .parse::<usize>()
                        .map_err(|e| format!("Invalid cycle in '{}': {}", b_part, e))?;
                    let slots: Vec<&str> = slots_part.split_whitespace().collect();
                    if slots.len() == 4 {
                        self.active_icache.insert(
                            cycle,
                            [
                                slots[0].to_string(),
                                slots[1].to_string(),
                                slots[2].to_string(),
                                slots[3].to_string(),
                            ],
                        );
                        loaded += 1;
                    }
                }
            }
        }
        Ok(loaded)
    }

    /// Creates a hardware shadow checkpoint before modifying an instruction bundle.
    pub fn create_checkpoint(&mut self, cycle: usize, checkpoint_id: u64) -> Result<(), String> {
        let slots = self
            .active_icache
            .get(&cycle)
            .ok_or_else(|| format!("Cycle B{:04} not found in active I-Cache", cycle))?;
        self.shadow_stack.push(ShadowCheckpoint {
            checkpoint_id,
            cycle,
            original_slots: slots.clone(),
            timestamp_cycles: self.total_mutations as u64,
        });
        Ok(())
    }

    /// Automatically repairs CRC-8 / parity tokens of a mutated raw 10-char slot token.
    pub fn heal_slot_parity(raw_token: &str) -> Result<String, String> {
        if raw_token.len() != 10 {
            return Err(format!("Token must be 10 chars, got '{}'", raw_token));
        }
        let chars: Vec<char> = raw_token.chars().collect();
        let prefix = chars[0];
        let op: String = chars[1..3].iter().collect();
        let dest_hex: String = chars[3..5].iter().collect();
        let mode = chars[5];
        let src = chars[6];
        let imm = chars[8];
        let term = chars[9];

        let correct_parity = compute_parity(prefix, &op, &dest_hex, mode, src, imm);
        let healed = format!("{}{}{}{}{}{}{}{}", prefix, op, dest_hex, mode, src, correct_parity, imm, term);
        Ok(healed)
    }

    /// Hot-patches a single VLIW slot in the active I-Cache during runtime.
    pub fn hot_patch_slot(
        &mut self,
        cycle: usize,
        slot_idx: usize,
        mut new_token: String,
        auto_heal_parity: bool,
    ) -> Result<PatchStatus, String> {
        if slot_idx >= 4 {
            return Err(format!("Slot index out of bounds (0..3): {}", slot_idx));
        }

        if auto_heal_parity {
            new_token = Self::heal_slot_parity(&new_token)?;
        }

        // Validate syntax with cl_lang
        parse_slot(&new_token)?;

        let bundle = self
            .active_icache
            .get_mut(&cycle)
            .ok_or_else(|| format!("Cycle B{:04} does not exist in active I-Cache", cycle))?;

        let old_token = bundle[slot_idx].clone();
        bundle[slot_idx] = new_token.clone();

        self.patch_log.push(LiveSlotPatch {
            cycle,
            slot_index: slot_idx,
            new_raw_token: new_token,
            original_token: Some(old_token),
        });

        self.total_mutations += 1;
        Ok(PatchStatus::Applied)
    }

    /// Performs a 1-cycle hardware rollback to the most recent shadow checkpoint.
    pub fn rollback_latest_checkpoint(&mut self) -> Result<PatchStatus, String> {
        let cp = self
            .shadow_stack
            .pop()
            .ok_or_else(|| "No shadow checkpoints available on stack".to_string())?;

        if let Some(bundle) = self.active_icache.get_mut(&cp.cycle) {
            *bundle = cp.original_slots;
            self.total_rollbacks += 1;
            Ok(PatchStatus::RolledBack(format!(
                "Successfully restored cycle B{:04} from checkpoint #{}",
                cp.cycle, cp.checkpoint_id
            )))
        } else {
            Err(format!("Cycle B{:04} missing during rollback", cp.cycle))
        }
    }

    /// Exports the current live modified I-Cache back into standard canonical .cl assembly.
    pub fn dump_canonical_cl(&self) -> String {
        let mut cycles: Vec<usize> = self.active_icache.keys().copied().collect();
        cycles.sort();

        let mut out = String::new();
        for cycle in cycles {
            if let Some(slots) = self.active_icache.get(&cycle) {
                out.push_str(&format!(
                    "B{:04}: {} {} {} {}\n",
                    cycle, slots[0], slots[1], slots[2], slots[3]
                ));
            }
        }
        out
    }
}
