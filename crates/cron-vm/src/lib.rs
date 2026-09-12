pub mod core_engine;
pub mod simulator;
pub mod torus_mesh;

pub use core_engine::CoreEngine;
pub use simulator::{HardwareStats, Simulator, VliwInstruction};
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
}
