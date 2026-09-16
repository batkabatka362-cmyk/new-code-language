#![allow(
    clippy::manual_checked_ops,
    clippy::needless_range_loop
)]

pub mod cluster;
pub mod core_engine;
pub mod simulator;
pub mod torus_mesh;

pub use cluster::{ClusterCoord, ClusterNode, ClusterSimulator, ClusterStats, InterChipLink, InterChipPacket};
pub use core_engine::CoreEngine;
pub use simulator::{BundleProfile, ExecutionProfile, FiberTask, HardwareStats, Simulator, VliwInstruction};
pub use torus_mesh::{Coord4D, TorusMesh};


pub fn run_cl(cl_code: &str) -> HardwareStats {
    let mut sim = Simulator::new();
    sim.load_machine_code(cl_code);
    sim.run();
    sim.stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4d_torus_coordinates_and_wrap() {
        let origin = Coord4D::new(0, 0, 0, 0);
        assert_eq!(origin.to_core_id(), 0);

        let neighbor_x_minus = origin.neighbor("X-");
        assert_eq!(neighbor_x_minus.x, 3); // 0 wrapped around to 3
        assert_eq!(neighbor_x_minus.to_core_id(), 3);

        let c255 = Coord4D::new(3, 3, 3, 3);
        assert_eq!(c255.to_core_id(), 255);
        let wrapped_w = c255.neighbor("W+");
        assert_eq!(wrapped_w.w, 0); // 3 wrapped around to 0
    }

    #[test]
    fn test_vliw_bundle_execution() {
        let cl_code = r#"
        ; Sample VLIW Test
        B0001: '=00#0A04> '=10#040B> _SH00$01B4> _SP00#0088>
        B0002: _OP041$0E> _FA054$20> _BK095$01> _HLT_____8!
        "#;
        let stats = run_cl(cl_code);
        assert_eq!(stats.total_cycles, 2);
        assert!(stats.optical_gemm_ops > 0);
        assert!(stats.reversible_gate_ops > 0);
    }

    #[test]
    fn test_branchless_16way_hyper_tree_traversal() {
        use crate::torus_mesh::{Coord4D, TorusMesh};
        let base = Coord4D::new(3, 3, 3, 3);
        // Test all 16 directions active
        let active_mask = 0xFFFF;
        let branches = TorusMesh::branchless_traverse_16way(base, active_mask);

        assert_eq!(branches.len(), 16);
        // Branch 0 (dx=0, dy=0, dz=0, dw=0) -> stays at (3,3,3,3)
        assert_eq!(branches[0].0, Coord4D::new(3, 3, 3, 3));
        // Branch 15 (dx=1, dy=1, dz=1, dw=1) -> wraps around to (0,0,0,0)
        assert_eq!(branches[15].0, Coord4D::new(0, 0, 0, 0));

        // Test inactive mask filters hash
        let sparse_mask = 0x0001; // only branch 0 active
        let sparse_branches = TorusMesh::branchless_traverse_16way(base, sparse_mask);
        assert!(sparse_branches[0].1 > 0);
        assert_eq!(sparse_branches[1].1, 0);
    }
}
