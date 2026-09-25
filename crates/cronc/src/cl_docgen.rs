//! Autonomous Microcode Documentation Generator (`cron cl-doc`)
//!
//! Analyzes `.cl` machine code kernels, extracts ISA directives (`@kernel`, `.target`, `.ipc_target`),
//! computes register footprints, coprocessor operational intensities, and generates Markdown docs.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use crate::cl_lang::parse_slot;

/// Analyzed metadata for a single `.cl` microcode kernel
#[derive(Debug, Clone)]
pub struct ClKernelDoc {
    pub file_name: String,
    pub kernel_name: String,
    pub target_silicon: String,
    pub ipc_target: f32,
    pub bundle_count: usize,
    pub slot_count: usize,
    pub written_regs: Vec<usize>,
    pub read_regs: Vec<usize>,
    pub optical_ops: usize,
    pub reversible_ops: usize,
    pub stdp_ops: usize,
    pub ai_isa_ops: usize,
    pub spiking_attn_ops: usize,
    pub swarm_ops: usize,
    pub homeostasis_ops: usize,
    pub landauer_joules: f64,
    pub raw_source: String,
}

impl ClKernelDoc {
    /// Generate rich Markdown representation for the microcode kernel
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!("# Microcode Kernel: `{}`\n\n", self.kernel_name));
        md.push_str(&format!("**Source File:** `{}` | **Target Silicon:** `{}` | **Target IPC:** `{:.1}`\n\n", 
            self.file_name, self.target_silicon, self.ipc_target));

        md.push_str("## 📊 Architectural Specifications\n\n");
        md.push_str("| Metric | Value | Description |\n");
        md.push_str("|---|---|---|\n");
        md.push_str(&format!("| **Total Bundles** | `{}` | 128-bit VLIW cycle bundles |\n", self.bundle_count));
        md.push_str(&format!("| **Total Slots** | `{}` | 10-character CRC-8 ATM micro-operations |\n", self.slot_count));
        md.push_str(&format!("| **Nominal IPC** | `{:.2}` | Instructions executed per clock cycle |\n", 
            if self.bundle_count > 0 { self.slot_count as f32 / self.bundle_count as f32 } else { 0.0 }));
        md.push_str(&format!("| **Thermal Dissipation** | `{:.3e} J` | Estimated Landauer thermodynamic cost |\n\n", self.landauer_joules));

        md.push_str("## 🧬 Silicon Coprocessor Subsystem Usage\n\n");
        md.push_str("| Coprocessor Domain | Ops Count | Description |\n");
        md.push_str("|---|---|---|\n");
        md.push_str(&format!("| 🔮 Photonic MZI Optical GEMM | `{}` | 0ns optical matrix dot-products |\n", self.optical_ops));
        md.push_str(&format!("| 🛡️ Reversible Logic (Fredkin/Toffoli) | `{}` | Zero-entropy state transformations |\n", self.reversible_ops));
        md.push_str(&format!("| 🧠 Neuromorphic Plasticity (STDP) | `{}` | Spike-timing synaptic weight adaptations |\n", self.stdp_ops));
        md.push_str(&format!("| ⚡ Dedicated AI Silicon ISA | `{}` | Softmax / SSM Mamba linear scans |\n", self.ai_isa_ops));
        md.push_str(&format!("| ⏱️ Temporal Spiking Attention | `{}` | Coincidence gating & pulse synchronization |\n", self.spiking_attn_ops));
        md.push_str(&format!("| 🐜 Stigmergy Swarm & NoC | `{}` | Pheromone diffusion & 4D-Torus spatial routing |\n", self.swarm_ops));
        md.push_str(&format!("| 🌿 Living Homeostasis | `{}` | Neuromodulation & metabolic energy balance |\n\n", self.homeostasis_ops));

        md.push_str("## 💾 Register Footprint\n\n");
        let write_str: Vec<String> = self.written_regs.iter().map(|r| format!("`R{}`", r)).collect();
        let read_str: Vec<String> = self.read_regs.iter().map(|r| format!("`R{}`", r)).collect();
        md.push_str(&format!("- **Written Registers:** {}\n", if write_str.is_empty() { "None".to_string() } else { write_str.join(", ") }));
        md.push_str(&format!("- **Read Registers:** {}\n\n", if read_str.is_empty() { "None".to_string() } else { read_str.join(", ") }));

        md.push_str("## 📜 Raw Microcode Listing\n\n```lisp\n");
        md.push_str(&self.raw_source);
        md.push_str("\n```\n");

        md
    }
}

/// Analyze a `.cl` file content and extract documentation metadata
pub fn analyze_cl_file(file_name: &str, content: &str) -> ClKernelDoc {
    let mut kernel_name = file_name.trim_end_matches(".cl").to_string();
    let mut target_silicon = "silicon.4d_torus".to_string();
    let mut ipc_target = 4.0f32;
    let mut bundle_count = 0;
    let mut slot_count = 0;
    let mut written_set = HashSet::new();
    let mut read_set = HashSet::new();
    let mut optical_ops = 0;
    let mut reversible_ops = 0;
    let mut stdp_ops = 0;
    let mut ai_isa_ops = 0;
    let mut spiking_attn_ops = 0;
    let mut swarm_ops = 0;
    let mut homeostasis_ops = 0;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("@kernel") {
            if let Some((_, name)) = trimmed.split_once(' ') {
                kernel_name = name.trim().to_string();
            }
        } else if trimmed.starts_with(".target") {
            if let Some((_, tgt)) = trimmed.split_once(' ') {
                target_silicon = tgt.trim().to_string();
            }
        } else if trimmed.starts_with(".ipc_target") {
            if let Some((_, ipc)) = trimmed.split_once(' ') {
                ipc_target = ipc.trim().parse::<f32>().unwrap_or(4.0);
            }
        } else if trimmed.starts_with('B') && trimmed.contains(':') {
            bundle_count += 1;
            if let Some((_, slots_part)) = trimmed.split_once(':') {
                for slot_str in slots_part.split_whitespace() {
                    slot_count += 1;
                    if let Ok(parsed) = parse_slot(slot_str) {
                        if let Some(d) = parsed.dest_reg {
                            written_set.insert((d % 16) as usize);
                        }
                        if let Some(s) = parsed.src_reg {
                            read_set.insert((s % 16) as usize);
                        }

                        match parsed.opcode.as_str() {
                            "OP" | "WD" => optical_ops += 1,
                            "RM" | "RV" | "RF" | "TO" | "BK" => reversible_ops += 1,
                            "ST" => stdp_ops += 1,
                            "SM" | "SN" | "SS" | "GE" | "SI" => ai_isa_ops += 1,
                            "LF" | "LI" | "CP" => spiking_attn_ops += 1,
                            "SB" | "DF" | "TX" | "RX" => swarm_ops += 1,
                            "DA" | "SE" | "NE" | "EE" => homeostasis_ops += 1,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    let mut written_regs: Vec<usize> = written_set.into_iter().collect();
    written_regs.sort_unstable();
    let mut read_regs: Vec<usize> = read_set.into_iter().collect();
    read_regs.sort_unstable();

    let landauer_joules = (slot_count as f64) * 2.87e-21;

    ClKernelDoc {
        file_name: file_name.to_string(),
        kernel_name,
        target_silicon,
        ipc_target,
        bundle_count,
        slot_count,
        written_regs,
        read_regs,
        optical_ops,
        reversible_ops,
        stdp_ops,
        ai_isa_ops,
        spiking_attn_ops,
        swarm_ops,
        homeostasis_ops,
        landauer_joules,
        raw_source: content.to_string(),
    }
}

/// Batch generate documentation for all `.cl` files in a source directory
pub fn generate_directory_docs(src_dir: &Path, out_dir: &Path) -> Result<Vec<PathBuf>, String> {
    fs::create_dir_all(out_dir).map_err(|e| format!("Failed to create output doc dir: {}", e))?;
    let mut generated_files = Vec::new();

    let entries = fs::read_dir(src_dir).map_err(|e| format!("Failed to read src dir {:?}: {}", src_dir, e))?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && path.extension().map(|e| e == "cl").unwrap_or(false) {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read {:?}: {}", path, e))?;
            let doc = analyze_cl_file(&file_name, &content);
            let md = doc.to_markdown();

            let out_file = out_dir.join(format!("{}.md", file_name.trim_end_matches(".cl")));
            fs::write(&out_file, md).map_err(|e| format!("Failed to write doc {:?}: {}", out_file, e))?;
            generated_files.push(out_file);
        }
    }

    Ok(generated_files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docgen_analyzes_agi_coprocessors() {
        let code = r#"
@kernel test_agi_coproc
.target silicon.4d_torus
.ipc_target 4.0

B0000: ==01#032' ==02#010' _LD03M100_ _LD04M200_
B0001: _SB05M304_ _CP06M501_ _LF07#080_ _TX08M700_
B0002: _DA09#010_ _SE0A#020_ _RM0BM900~ !HL00#0000
"#;
        let doc = analyze_cl_file("test_agi.cl", code);
        assert_eq!(doc.kernel_name, "test_agi_coproc");
        assert_eq!(doc.target_silicon, "silicon.4d_torus");
        assert_eq!(doc.ipc_target, 4.0);
        assert_eq!(doc.bundle_count, 3);
        assert_eq!(doc.slot_count, 12);
        assert_eq!(doc.spiking_attn_ops, 2); // CP and LF
        assert_eq!(doc.swarm_ops, 2); // SB and TX
        assert_eq!(doc.homeostasis_ops, 2); // DA and SE
        assert_eq!(doc.reversible_ops, 1); // RM

        let md = doc.to_markdown();
        assert!(md.contains("# Microcode Kernel: `test_agi_coproc`"));
        assert!(md.contains("Temporal Spiking Attention"));
        assert!(md.contains("Stigmergy Swarm & NoC"));
        assert!(md.contains("Living Homeostasis"));
    }
}
