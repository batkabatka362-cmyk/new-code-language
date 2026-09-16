import unittest
import sys
import os

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

    def test_kernel_spec_installed(self):
        dest = cron.install_kernel_spec()
        self.assertTrue(os.path.exists(dest), f"Kernel spec directory '{dest}' must exist")
        kernel_json = os.path.join(dest, "kernel.json")
        self.assertTrue(os.path.exists(kernel_json), "kernel.json must exist")

if __name__ == "__main__":
    unittest.main()
