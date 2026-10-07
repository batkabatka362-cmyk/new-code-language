//! `.cl` Hazard-Free VLIW Bundle Scheduler (`cron cl-fix`)
//!
//! Rewrites straight-line `.cl` microcode so that no bundle contains an
//! intra-bundle RAW (read-after-write) or WAW (write-after-write) register
//! hazard, as detected by `cl_lint`. Slots keep their original program order;
//! a new bundle is opened only when the next slot would conflict with a slot
//! already in the current bundle. Bundles are then renumbered `B0000..`.
//!
//! Only straight-line kernels (no branch targets referencing bundle labels)
//! should be scheduled; all `libcl` standard kernels satisfy this.

use crate::cl_lang::parse_slot;

/// Result of a scheduling pass.
#[derive(Debug, Clone)]
pub struct ClScheduleReport {
    pub source: String,
    pub bundles_before: usize,
    pub bundles_after: usize,
    pub hazards_resolved: usize,
}

fn is_bundle_line(line: &str) -> bool {
    let t = line.trim();
    t.starts_with('B') && t.contains(':')
}

/// Split bundles at every intra-bundle hazard and renumber labels.
pub fn resolve_hazards(cl_source: &str) -> ClScheduleReport {
    let mut header: Vec<String> = Vec::new();
    let mut bundles: Vec<Vec<String>> = Vec::new();
    let mut bundles_before = 0usize;
    let mut hazards_resolved = 0usize;

    let mut cur: Vec<String> = Vec::new();
    let mut writes: Vec<usize> = Vec::new();
    let mut seen_bundle = false;

    for line in cl_source.lines() {
        if !is_bundle_line(line) {
            // Preserve comments/directives/blank lines ahead of code; drop nothing.
            if !seen_bundle {
                header.push(line.to_string());
            }
            continue;
        }
        seen_bundle = true;
        bundles_before += 1;
        let (_, slots_part) = line.trim().split_once(':').unwrap();
        for slot in slots_part.split_whitespace() {
            let mut conflict = false;
            let mut dest = None;
            if let Ok(p) = parse_slot(slot) {
                if p.opcode != "NO" && p.opcode != "HL" {
                    if let Some(d) = p.dest_reg {
                        if writes.contains(&d) {
                            conflict = true;
                        }
                        dest = Some(d);
                    }
                    if let Some(s) = p.src_reg {
                        if writes.contains(&s) && p.opcode != "==" {
                            conflict = true;
                        }
                    }
                }
            }
            if conflict && !cur.is_empty() {
                bundles.push(std::mem::take(&mut cur));
                writes.clear();
                hazards_resolved += 1;
            }
            if let Some(d) = dest {
                writes.push(d);
            }
            cur.push(slot.to_string());
            if cur.len() == 4 {
                bundles.push(std::mem::take(&mut cur));
                writes.clear();
            }
        }
        if !cur.is_empty() {
            bundles.push(std::mem::take(&mut cur));
            writes.clear();
        }
    }

    let mut out = String::new();
    for h in &header {
        out.push_str(h);
        out.push('\n');
    }
    for (i, b) in bundles.iter().enumerate() {
        out.push_str(&format!("B{:04X}: {}\n", i, b.join(" ")));
    }

    ClScheduleReport {
        source: out,
        bundles_before,
        bundles_after: bundles.len(),
        hazards_resolved,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cl_lint::lint_cl_source;

    #[test]
    fn test_schedule_removes_all_raw_waw_warnings() {
        let dir = std::env::temp_dir().join("cron_cl_schedule_test");
        let _ = std::fs::remove_dir_all(&dir);
        crate::cl_pkg::generate_standard_libcl_files(&dir).unwrap();
        for entry in std::fs::read_dir(&dir).unwrap() {
            let p = entry.unwrap().path();
            if p.extension().map(|e| e == "cl").unwrap_or(false) {
                let src = std::fs::read_to_string(&p).unwrap();
                let fixed = resolve_hazards(&src);
                let rep = lint_cl_source(&fixed.source, "k.cl");
                assert_eq!(rep.error_count, 0, "{:?}", p);
                assert_eq!(rep.warning_count, 0, "hazards remain in {:?}", p);
            }
        }
    }

    #[test]
    fn test_resolve_hazards_is_idempotent() {
        let dir = std::env::temp_dir().join("cron_cl_schedule_idem");
        let _ = std::fs::remove_dir_all(&dir);
        crate::cl_pkg::generate_standard_libcl_files(&dir).unwrap();
        let src = std::fs::read_to_string(dir.join("backprop_gemm.cl")).unwrap();
        let once = resolve_hazards(&src);
        let twice = resolve_hazards(&once.source);
        assert_eq!(twice.hazards_resolved, 0);
        assert_eq!(twice.source.trim_end(), once.source.trim_end());
    }
}
