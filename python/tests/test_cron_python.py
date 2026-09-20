import unittest
import sys
import os
import tempfile

# Add python directory to path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import cron

class TestCronPythonSDK(unittest.TestCase):

    def test_find_executable(self):
        exe = cron.find_cron_executable()
        self.assertTrue(len(exe) > 0, "Should locate cron executable")

    def test_c23_compilation(self):
        source = """
        .MODULE PyBridgeTest
        .ENTRY _main
        _main:
            let val_x: i32 = 77
            let val_y: i32 = 33
            let val_sum: i32 = val_x + val_y
            export val_sum as final_sum
        .END
        """
        c23_code = cron.compile_c23(source)
        self.assertIn("#include <stdint.h>", c23_code)
        self.assertIn("val_sum", c23_code)
        self.assertIn("[CRON Native C23 Engine]", c23_code)

    def test_cl_compilation(self):
        source = """
        .MODULE VliwPyTest
        .ENTRY _main
        _main:
            let seed: i32 = 123
        .END
        """
        cl_code = cron.compile_source(source)
        self.assertIn("B0001:", cl_code)

    def test_simulation(self):
        source = """
        .MODULE SimTest
        .ENTRY _main
        _main:
            let lin seed: wave_t = pack_wave(amp=[10, 20], phase=[0, 32])
            consume(seed)
        .END
        """
        sim = cron.Simulator()
        telemetry = sim.run(source)
        self.assertEqual(telemetry["active_cores"], 256)
        self.assertGreaterEqual(telemetry["total_cycles"], 0)

    def test_photonic_bridge(self):
        bridge = cron.PhotonicBridge()
        res = bridge.execute_optical_gemm([64, 32, 16, 8], [0, 32, 64, 96])
        self.assertIn("output_amps", res)
        self.assertEqual(len(res["output_amps"]), 4)

    def test_kernel_execution(self):
        kernel = cron.CronKernel()
        self.assertEqual(kernel.language, "cron")
        self.assertIn("4D-Torus", kernel.banner)

        # Standard CRON cell execution
        source = """
        .MODULE KernelCellTest
        _main:
            let a: i32 = 100
            let b: i32 = 200
            let total: i32 = a + b
        .END
        """
        res = kernel.do_execute(source, silent=False)
        self.assertEqual(res["status"], "ok")

        # Test %check magic
        check_res = kernel.do_execute("%check " + source, silent=False)
        self.assertEqual(check_res["status"], "ok")

        # Test %c23 magic
        c23_res = kernel.do_execute("%c23 " + source, silent=False)
        self.assertEqual(c23_res["status"], "ok")

        # Test %info magic
        info_res = kernel.do_execute("%info", silent=False)
        self.assertEqual(info_res["status"], "ok")

        # Test %esoteric magic
        esoteric_res = kernel.do_execute("%esoteric demo", silent=False)
        self.assertEqual(esoteric_res["status"], "ok")

        # Test %tile magic
        tile_res = kernel.do_execute("%tile gemm --m 32 --n 32 --k 32 --systolic", silent=False)
        self.assertEqual(tile_res["status"], "ok")

        # Test %optic magic
        optic_res = kernel.do_execute("%optic", silent=False)
        self.assertEqual(optic_res["status"], "ok")

    def test_optical_budget_python_sdk(self):
        sample_cl = """
        B0000: _OP01$28F> _NO00#000> _NO00#000> _NO00#000>
        B0001: _WD00#100> _NO00#000> _NO00#000> _HL00#000!
        """
        # 1. Optical loss and laser budget analysis
        rep = cron.analyze_optical_budget(sample_cl, mesh_dim=16, wdm_channels=8, topology="clemens")
        self.assertEqual(rep["status"], "COMPLIANT")
        self.assertIn("insertion_loss_db", rep)
        self.assertIn("required_laser_power_mw", rep)
        self.assertEqual(len(rep["wdm_channels"]), 8)

        # 2. Optical closed-loop Verilog synthesis
        v = cron.synthesize_optical_verilog(sample_cl, mesh_dim=16, wdm_channels=8)
        self.assertIn("module optical_laser_power_controller", v)
        self.assertIn("laser_diode_enable", v)
        self.assertIn("pd_feedback_current_ua", v)

    def test_esoteric_coprocessor_python_sdk(self):
        # 1. Run esoteric simulation
        telemetry = cron.run_esoteric_sim("demo")
        self.assertEqual(telemetry["status"], "SUCCESS")
        self.assertIn("total_trit_macs", telemetry)
        self.assertIn("systolic_hops", telemetry)

        # 2. Synthesize esoteric Verilog RTL
        verilog = cron.synthesize_esoteric_coprocessor()
        self.assertIn("module esoteric_coprocessor", verilog)
        self.assertIn("trit_weights", verilog)

    def test_tile_gemm_systolic_sdk(self):
        # Run systolic loop tiling via Python SDK
        res = cron.tile_gemm_systolic(m=32, n=32, k=32, unroll=4, subbyte=True)
        self.assertIn("operation_type", res)
        self.assertIn("Systolic Wavefront", res["operation_type"])
        self.assertEqual(res["estimated_ipc"], 4.0)

    def test_comparison_benchmark_sdk(self):
        # 1. Run comparison benchmark via Python SDK
        res = cron.run_comparison_benchmark(workload="bitnet", baseline="all")
        self.assertIn("mean_speedup_vs_pytorch", res)
        self.assertIn("workloads", res)
        self.assertGreater(res["mean_speedup_vs_pytorch"], 1.0)
        self.assertEqual(res["workloads"][0]["workload"], "BitNetGemm")

        # 2. Generate Whitepaper via Python SDK
        temp_dir = tempfile.gettempdir()
        wp_path = os.path.join(temp_dir, "sdk_test_whitepaper.md")
        content = cron.generate_benchmark_whitepaper(output_path=wp_path)
        self.assertTrue(os.path.exists(wp_path))
        self.assertIn("CRON: A Spatial-Cognitive Architecture & Language", content)
        self.assertIn("Mathematical Foundations of CRON Superiority", content)
        try:
            os.remove(wp_path)
        except OSError:
            pass

if __name__ == "__main__":
    unittest.main()

