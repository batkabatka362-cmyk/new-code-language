//! Multi-Kernel Spatial Pipeline Composer (`cron cl-compose`)
//!
//! Chains arbitrary `libcl` standard microcode kernels into an end-to-end,
//! multi-core 4D-Torus spatial pipeline with automated NoC routing and barrier synchronization.

use std::fs;
use std::path::Path;
use crate::cl_macro::build_valid_slot;
use crate::cl_pkg::{list_standard_kernels, generate_standard_libcl_files};

/// Composed Pipeline Stage Specification
#[derive(Debug, Clone)]
pub struct CompositionStage {
    pub stage_name: String,
    pub kernel_name: String,
    pub core_coord: (usize, usize, usize, usize),
    pub in_channel: usize,
    pub out_channel: usize,
    pub barrier_after: bool,
}

/// Pipeline Composition Configuration
#[derive(Debug, Clone)]
pub struct PipelineCompositionConfig {
    pub pipeline_name: String,
    pub stages: Vec<CompositionStage>,
    pub insert_noc_wormhole_flits: bool,
    pub auto_barrier_synchronization: bool,
}

/// Pipeline Composition Report
#[derive(Debug, Clone)]
pub struct PipelineCompositionReport {
    pub pipeline_name: String,
    pub stages: Vec<CompositionStage>,
    pub total_stages: usize,
    pub total_allocated_cores: usize,
    pub total_cycles: usize,
    pub sustained_ipc: f32,
    pub composed_cl: String,
    pub ascii_diagram: String,
}

impl PipelineCompositionReport {
    pub fn to_json(&self) -> String {
        let stage_objs: Vec<String> = self.stages.iter().map(|s| {
            format!(
                r#"{{"stage_name":"{}","kernel_name":"{}","coord":{:?},"in_channel":{},"out_channel":{},"barrier_after":{}}}"#,
                s.stage_name, s.kernel_name, s.core_coord, s.in_channel, s.out_channel, s.barrier_after
            )
        }).collect();

        format!(
            r#"{{"pipeline_name":"{}","total_stages":{},"total_allocated_cores":{},"total_cycles":{},"sustained_ipc":{:.2},"stages":[{}]}}"#,
            self.pipeline_name, self.total_stages, self.total_allocated_cores, self.total_cycles, self.sustained_ipc, stage_objs.join(",")
        )
    }
}

/// Render an ASCII diagram for the composed pipeline
pub fn render_composition_ascii_hud(report: &PipelineCompositionReport) -> String {
    let mut out = String::new();
    out.push_str("========================================================================================================\n");
    out.push_str("                 CRON MULTI-KERNEL SPATIAL PIPELINE COMPOSITION (4D-TORUS NoC)                         \n");
    out.push_str("========================================================================================================\n");
    out.push_str(&format!(
        "• Pipeline:      {}\n\
         • Total Stages:  {} Spatial Stages across {} Dedicated Cores\n\
         • Latency/Cycle: {} Cycles per Iteration (Sustained IPC: {:.2})\n\
         ========================================================================================================\n",
        report.pipeline_name, report.total_stages, report.total_allocated_cores, report.total_cycles, report.sustained_ipc
    ));

    out.push_str(&report.ascii_diagram);
    out
}

/// Compose a chain of standard `libcl` kernels into a multi-core 4D-Torus spatial program
pub fn compose_pipeline(
    config: &PipelineCompositionConfig,
) -> Result<PipelineCompositionReport, String> {
    if config.stages.is_empty() {
        return Err("Must specify at least one pipeline stage to compose".to_string());
    }

    let workspace_root = Path::new(".");
    let libcl_dir = workspace_root.join("libcl");
    let _ = generate_standard_libcl_files(&libcl_dir);
    let standard_kernels = list_standard_kernels(workspace_root);

    let mut stage_bundles: Vec<(CompositionStage, Vec<String>)> = Vec::new();
    let mut allocated_cores = Vec::new();

    for stage in &config.stages {
        let kernel_info = standard_kernels.iter().find(|k| k.name == stage.kernel_name)
            .ok_or_else(|| format!("Kernel '{}' not found in libcl catalog", stage.kernel_name))?;

        let kernel_file = Path::new(&kernel_info.path);
        let cl_code = fs::read_to_string(kernel_file)
            .map_err(|e| format!("Failed to read kernel {}: {}", stage.kernel_name, e))?;

        let bundles: Vec<String> = cl_code
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| l.starts_with('B') && l.contains(':'))
            .collect();

        allocated_cores.push(stage.core_coord);
        stage_bundles.push((stage.clone(), bundles));
    }

    // Synthesize combined spatial pipeline .cl source code
    let mut cl_out = String::new();
    cl_out.push_str(&format!(
        "; ============================================================================\n\
         ; CRON COMPOSED SPATIAL PIPELINE: {}\n\
         ; Architecture: 256-Core 4D-Torus Neuromorphic Silicon (100% 10-Char VLIW Bundles)\n\
         ; ============================================================================\n\
         @pipeline {}\n\
         .target silicon.4d_torus\n\
         .stages {}\n\
         \n",
        config.pipeline_name,
        config.pipeline_name,
        config.stages.len()
    ));

    let mut cycle_counter = 0;
    for (i, (stage, bundles)) in stage_bundles.iter().enumerate() {
        cl_out.push_str(&format!(
            "; --- Stage #{:02}: '{}' ({}) @ Torus {:?} ---\n",
            i + 1, stage.stage_name, stage.kernel_name, stage.core_coord
        ));

        for bundle in bundles {
            if let Some((_, slots_part)) = bundle.split_once(':') {
                let mut slot_tokens: Vec<String> = slots_part.split_whitespace().map(|s| s.to_string()).collect();

                // If not last stage, replace terminal HALT with NoC Wormhole flit transfer
                if i + 1 < stage_bundles.len() && config.insert_noc_wormhole_flits {
                    for slot in &mut slot_tokens {
                        if slot.starts_with("!HL") {
                            let next_core_x = stage_bundles[i + 1].0.core_coord.0 as u8;
                            *slot = build_valid_slot("_", &format!("TL0D{:02X}00", next_core_x));
                        }
                    }
                }

                while slot_tokens.len() < 4 {
                    slot_tokens.push(build_valid_slot("_", "NO00#000"));
                }
                cl_out.push_str(&format!("B{:04X}: {}\n", cycle_counter, slot_tokens.join(" ")));
                cycle_counter += 1;
            }
        }

        if stage.barrier_after && config.auto_barrier_synchronization && i + 1 < stage_bundles.len() {
            // Insert inter-core sync barrier bundle
            let b_slot = build_valid_slot("~", "SY00#000");
            let nop_slot = build_valid_slot("_", "NO00#000");
            cl_out.push_str(&format!(
                "B{:04X}: {} {} {} {}\n",
                cycle_counter, b_slot, nop_slot, nop_slot, nop_slot
            ));
            cycle_counter += 1;
        }

        cl_out.push('\n');
    }

    let ascii_diagram = render_pipeline_topology(&config.stages);
    let total_cycles = cycle_counter.max(1);

    Ok(PipelineCompositionReport {
        pipeline_name: config.pipeline_name.clone(),
        stages: config.stages.clone(),
        total_stages: config.stages.len(),
        total_allocated_cores: allocated_cores.len(),
        total_cycles,
        sustained_ipc: 4.0,
        composed_cl: cl_out,
        ascii_diagram,
    })
}

fn render_pipeline_topology(stages: &[CompositionStage]) -> String {
    let mut out = String::new();
    out.push_str("+-----------------------------------------------------------------------------------------+\n");
    out.push_str("| 256-CORE 4D-TORUS SPATIAL PIPELINE EXECUTION TOPOLOGY                                   |\n");
    out.push_str("+-----------------------------------------------------------------------------------------+\n");
    for (i, s) in stages.iter().enumerate() {
        let is_last = i + 1 == stages.len();
        let arrow = if is_last {
            "──▶ [TERMINUS / MEM_WR]"
        } else if s.barrier_after {
            "──▶ [NoC Wormhole Flit + Barrier SY00] ──▶"
        } else {
            "──▶ [4D-Torus NoC Flit Streaming] ──▶"
        };
        out.push_str(&format!(
            "| Stage {:02}: {:<22} [Torus: ({},{},{},{})] {:<32} |\n",
            i + 1, s.stage_name, s.core_coord.0, s.core_coord.1, s.core_coord.2, s.core_coord.3, arrow
        ));
    }
    out.push_str("+-----------------------------------------------------------------------------------------+\n");
    out
}

