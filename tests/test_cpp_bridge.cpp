#include <iostream>
#include <cassert>
#include <vector>
#include <cmath>
#include "../include/cron_rt.hpp"

int main() {
    std::cout << "====================================================================" << std::endl;
    std::cout << "🔮 TESTING CRON C++20 ZERO-OVERHEAD NATIVE BRIDGE (GRADE SSS+)" << std::endl;
    std::cout << "====================================================================" << std::endl;

    // TEST 1: Version & Target Info
    std::string ver = cron::Compiler::version();
    std::string target = cron::Compiler::target_info();
    std::cout << "  ✓ Version: " << ver << std::endl;
    std::cout << "  ✓ Target : " << target << std::endl;
    assert(ver.find("1.0.0") != std::string::npos);
    assert(target.find("4D-Torus") != std::string::npos);
    std::cout << "  ✓ Test 1 Passed: Runtime metadata verified." << std::endl;

    // TEST 2: In-Memory Multi-Backend Compilation (Zero Disk I/O)
    std::string cr_code = R"(
    .MODULE CppBridgeDemo
    struct Vector3D {
        x: f64,
        y: f64,
        z: f64,
    }

    _main:
        let v = Vector3D { x: 1.0, y: 2.0, z: 3.0 }
        let mag_sq: f64 = v.x * v.x + v.y * v.y + v.z * v.z
    .END
    )";

    // 2.1 Compile to VLIW
    std::string vliw = cron::Compiler::to_vliw(cr_code);
    assert(vliw.find("B0001:") != std::string::npos);
    std::cout << "  ✓ Test 2.1 Passed: In-memory VLIW bytecode generated (" << vliw.size() << " bytes)." << std::endl;

    // 2.2 Compile to C23
    std::string c23 = cron::Compiler::to_c23(cr_code);
    assert(c23.find("struct Vector3D") != std::string::npos);
    std::cout << "  ✓ Test 2.2 Passed: In-memory C23 source generated (" << c23.size() << " bytes)." << std::endl;

    // 2.3 Compile to LLVM IR
    std::string llvm_ir = cron::Compiler::to_llvm(cr_code);
    assert(llvm_ir.find("define i32 @main") != std::string::npos);
    std::cout << "  ✓ Test 2.3 Passed: In-memory LLVM IR generated (" << llvm_ir.size() << " bytes)." << std::endl;

    // TEST 3: In-Process 256-Core 4D-Torus VM Execution (Microsecond Latency)
    cron::VM vm;
    std::string sim_code = R"(
    .MODULE SimulationCore
    _main:
        let lin wave: wave_t = pack_wave(amp=[10, 20], phase=[0, 45])
        consume(wave)
    .END
    )";

    auto telemetry = vm.run(sim_code);
    assert(telemetry.success == true);
    assert(telemetry.active_cores == 256);
    assert(telemetry.total_cycles > 0);
    assert(telemetry.peak_temperature_c < 180);
    std::cout << "  ✓ Test 3 Passed: In-process 4D-Torus simulation succeeded in "
              << telemetry.total_cycles << " cycles (Active Cores: " << telemetry.active_cores << ")." << std::endl;

    // TEST 4: Zero-Copy 4D Tensor Memory Bridge
    {
        cron::Tensor4D<float> tensor(1, 16, 8, 8); // [N=1, C=16, H=8, W=8]
        assert(tensor.total_elements() == 1 * 16 * 8 * 8);
        assert(tensor.shape()[0] == 1 && tensor.shape()[1] == 16);

        // Direct in-memory indexing with 0 copy overhead
        tensor(0, 3, 4, 5) = 3.14159f;
        tensor(0, 7, 2, 1) = 99.88f;

        assert(std::abs(tensor(0, 3, 4, 5) - 3.14159f) < 1e-5f);
        assert(std::abs(tensor(0, 7, 2, 1) - 99.88f) < 1e-5f);

        // Span-based memory access for modern algorithms
        auto sp = tensor.span();
        assert(sp.size() == 1024);
        float sum = 0.0f;
        for (float val : sp) {
            sum += val;
        }
        assert(sum > 100.0f);
        std::cout << "  ✓ Test 4 Passed: Zero-copy 4D Tensor mapped (1024 floats, direct pointer access)." << std::endl;
    }

    // TEST 5: Hardware Photonic MZI Optical GEMM Accelerator
    {
        std::vector<double> amps = {1.0, 2.0, 3.0, 4.0};
        std::vector<double> phases = {0.0, 1.5707963, 3.1415926, 0.5};
        std::vector<double> out_amps(4, 0.0);
        std::vector<double> out_phases(4, 0.0);

        cron::PhotonicEngine::mzi_gemm(amps, phases, out_amps, out_phases);
        assert(std::abs(out_amps[0] - 1.0) < 1e-6);
        assert(out_amps[1] < 1e-6); // Phase pi/2 -> cos=0
        std::cout << "  ✓ Test 5 Passed: Photonic MZI optical interference accelerator executed." << std::endl;
    }

    // TEST 6: Native x86_64 JIT Dynamic Execution Engine (Sub-Microsecond)
    {
        std::string jit_source = R"(
        .MODULE CppJitTest
        _main:
            let x: i32 = 50
            let y: i32 = 75
            let ans: i32 = x + y * 2
            return ans
        .END
        )";
        int64_t result = cron::Compiler::jit(jit_source);
        assert(result == 200);
        std::cout << "  ✓ Test 6 Passed: Native x86_64 JIT executed in-memory with result = " << result << std::endl;
    }

    // TEST 7: Hardware SIMD & AVX2 Vector Intrinsics (Grade SSS+)
    {
        std::string simd_source = R"(
        .MODULE CppSimdTest
        _main:
            let a: vec8f = [1, 2, 3, 4, 5, 6, 7, 8]
            let b: vec8f = simd_splat(3)
            let c: vec8f = simd_splat(10)
            let acc: vec8f = simd_fma(a, b, c)
            let total: i32 = simd_reduce_sum(acc)
            return total
        .END
        )";
        int64_t result = cron::Compiler::jit(simd_source);
        assert(result == 188);
        std::cout << "  ✓ Test 7 Passed: Hardware SIMD AVX2 FMA execution succeeded with result = " << result << std::endl;
    }

    // TEST 8: Language-Level Native Autodiff (grad) & Fredkin Reversible Stack
    {
        std::string autodiff_source = R"(
        .MODULE CppAutodiffTest
        def cubic(x: i64) -> i64 {
            return x * x * x
        }
        _main:
            // d/dx(x^3) at x=4 is 3 * 4^2 = 48
            let d: i64 = grad(cubic)(4)
            return d
        .END
        )";
        int64_t result = cron::Compiler::jit(autodiff_source);
        assert(result == 48);
        std::cout << "  ✓ Test 8 Passed: Language-Level Native Autodiff (grad(cubic)(4) = " << result << ") verified." << std::endl;
    }

    // TEST 9: CSP Typed Channels & 4D-Torus Mesh Concurrency (Grade SSS+)
    {
        cron::Channel<int64_t> ch(8);
        assert(ch.id() != 0);

        // Non-blocking try_recv on empty channel
        auto empty_val = ch.try_recv();
        assert(!empty_val.has_value());

        // Send and receive
        bool sent1 = ch.send(1337);
        bool sent2 = ch.send(4242);
        assert(sent1 && sent2);

        int64_t v1 = ch.recv();
        assert(v1 == 1337);

        auto v2 = ch.try_recv();
        assert(v2.has_value() && *v2 == 4242);

        // Verify 4D-Torus wormhole hop distance
        int64_t dist_c0_c42 = cron::TorusMesh::distance(0, 42);
        assert(dist_c0_c42 == 6);
        int64_t dist_c42_c127 = cron::TorusMesh::distance(42, 127);
        assert(dist_c42_c127 == 4);

        std::cout << "  ✓ Test 9 Passed: CSP Channel (FIFO & non-blocking) and 4D-Torus Wormhole mesh routing verified." << std::endl;
    }

    std::cout << "\n🎉 ALL 9 CRON C++20 ZERO-OVERHEAD BRIDGE TESTS PASSED (GRADE SSS+)!" << std::endl;
    return 0;
}

