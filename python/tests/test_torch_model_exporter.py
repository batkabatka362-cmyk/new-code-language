import unittest
import os
import tempfile
import cron

try:
    import torch
    import torch.nn as nn
    HAS_TORCH = True
except ImportError:
    HAS_TORCH = False


class TestTorchModelExporter(unittest.TestCase):
    def setUp(self):
        self.tmp_dir = tempfile.mkdtemp()

    def tearDown(self):
        for f in os.listdir(self.tmp_dir):
            try:
                os.remove(os.path.join(self.tmp_dir, f))
            except OSError:
                pass
        try:
            os.rmdir(self.tmp_dir)
        except OSError:
            pass

    @unittest.skipUnless(HAS_TORCH, "PyTorch is not installed")
    def test_export_linear_sequential(self):
        model = nn.Sequential(
            nn.Linear(8, 8),
            nn.ReLU(),
            nn.Linear(8, 8)
        )
        cl_path = os.path.join(self.tmp_dir, "seq_model.cl")
        cl_code = cron.export_torch_model(model, cl_path, module_name="SeqModel", quantize="ternary")
        
        self.assertTrue(os.path.exists(cl_path))
        self.assertIn("@kernel torch_seqmodel", cl_code)
        self.assertIn("B0000:", cl_code)
        self.assertIn("B0001:", cl_code)

        # Check diagnostics on the generated .cl microcode
        diag = cron.check_diagnostics(cl_code)
        self.assertEqual(len(diag), 0)

        # Canonicalize and format sovereign .cl
        fmt_code = cron.compile_source(cl_code)
        self.assertIn("B0001:", fmt_code)

        # Run on cycle-accurate 256-Core 4D-Torus Simulator
        sim = cron.Simulator()
        stats = sim.run_cl(cl_code)
        self.assertGreater(stats.get("total_cycles", 0), 0)

    @unittest.skipUnless(HAS_TORCH, "PyTorch is not installed")
    def test_from_torch_in_memory(self):
        block = cron.CronTransformerBlock(dim=8, n_heads=2, mlp_dim=16)
        cl_code = cron.from_torch(block, quantize="ternary")
        self.assertIn("@kernel", cl_code)
        self.assertIn("B0000:", cl_code)


if __name__ == "__main__":
    unittest.main()
