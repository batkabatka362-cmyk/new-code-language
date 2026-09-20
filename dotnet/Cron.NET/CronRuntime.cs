// ============================================================================
// CRON .NET 8 / C# 12 Native Runtime Wrapper (CronRuntime.cs)
// High-performance P/Invoke binding for 256-Core 4D-Torus Neuromorphic Architecture
// ============================================================================

using System;
using System.Runtime.InteropServices;
using System.Text;

namespace Cron.NET
{
    [StructLayout(LayoutKind.Sequential)]
    public struct CronTelemetry
    {
        public ulong TotalCycles;
        public uint ActiveCores;
        public ulong OpticalGemmOps;
        public ulong ReversibleGateOps;
        public ulong StdpSynapseUpdates;
        public ulong MeshPacketsRouted;
        public uint PeakTemperatureC;
        public double DramBandwidthSavedMb;
        [MarshalAs(UnmanagedType.I1)]
        public bool Success;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct NativeTensor4D
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        public int[] Shape;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        public int[] Strides;
        public int ElemType;
        public IntPtr Data;
        public nuint TotalElements;
        public nuint SizeBytes;
    }

    internal static class NativeMethods
    {
        private const string DllName = "cron_rt";

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr cron_version();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr cron_target_info();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void cron_string_free(IntPtr str);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr cron_compile_source(string crSource, out IntPtr outError);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr cron_compile_to_c23(string crSource, out IntPtr outError);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr cron_compile_to_llvm(string crSource, out IntPtr outError);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_check_syntax(string crSource, out IntPtr outError);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern long cron_jit_execute(string crSource, out IntPtr outError);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr cron_vm_create();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void cron_vm_destroy(IntPtr vm);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void cron_vm_reset(IntPtr vm);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_vm_run_cl(IntPtr vm, string clCode, ref CronTelemetry outTelemetry);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_vm_run_cr(IntPtr vm, string crSource, ref CronTelemetry outTelemetry);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr cron_tensor4d_create(int n, int c, int h, int w, int elemType);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr cron_tensor4d_data_ptr(IntPtr tensor);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void cron_tensor4d_destroy(IntPtr tensor);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void cron_photonic_mzi_gemm(
            [In] double[] amps,
            [In] double[] phases,
            nuint count,
            [Out] double[] outAmps,
            [Out] double[] outPhases
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_ternary_quantize(
            [In] float[] weights,
            nuint count,
            [Out] byte[] outPacked,
            out float outScale
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern float cron_ternary_dot_product(
            [In] byte[] packedWeights,
            [In] float[] activations,
            nuint count,
            float scale
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_cl_audit(
            string clSource,
            out IntPtr outReportJson
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr cron_bpe_create_default();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_bpe_encode(
            IntPtr tokenizer,
            string text,
            [Out] uint[] outIds,
            nuint maxIds,
            out nuint outLen
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_bpe_decode(
            IntPtr tokenizer,
            [In] uint[] ids,
            nuint numIds,
            out IntPtr outStr
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void cron_bpe_free(IntPtr tokenizer);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_vision_extract_patches(
            [In] byte[] rgbData,
            nuint width,
            nuint height,
            nuint channels,
            nuint patchSize,
            nuint embedDim,
            [Out] float[] outEmbeddings,
            nuint maxElements,
            out nuint outPatchCount
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_audio_mel_spectrogram(
            [In] float[] pcmSamples,
            nuint numSamples,
            nuint sampleRate,
            nuint melBands,
            nuint embedDim,
            [Out] float[] outEmbeddings,
            nuint maxElements,
            out nuint outFrameCount
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr cron_swarm_create_256();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_swarm_execute_task(
            IntPtr swarm,
            string taskDesc,
            out IntPtr outReportJson
        );

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void cron_swarm_free(IntPtr swarm);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr cron_cluster_swarm_create_4096();

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_cluster_swarm_execute_task(
            IntPtr cluster,
            [MarshalAs(UnmanagedType.LPStr)] string taskDesc,
            out IntPtr outReportJson);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void cron_cluster_swarm_free(IntPtr cluster);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_swarm_synthesize_and_heal(
            [MarshalAs(UnmanagedType.LPStr)] string prompt,
            nuint maxIterations,
            [MarshalAs(UnmanagedType.I1)] bool autoHeal,
            [MarshalAs(UnmanagedType.I1)] bool runJit,
            out IntPtr outCode,
            out IntPtr outReportJson);

        // Section 14: Swarm TUI Visualizer
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_swarm_tui_render_snapshot(
            nuint ticks,
            uint viewMode,
            [MarshalAs(UnmanagedType.I1)] bool useColor,
            out IntPtr outSnapshot);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_swarm_tui_telemetry_json(
            nuint ticks,
            out IntPtr outJson);

        // Section 15: Neuro-Symbolic MCTS & Formal Mathematical Proof Verification
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_mcts_synthesize(
            [MarshalAs(UnmanagedType.LPStr)] string prompt,
            nuint simulations,
            nuint rolloutDepth,
            out IntPtr outCode,
            out IntPtr outReportJson);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_proof_verify(
            [MarshalAs(UnmanagedType.LPStr)] string clCode,
            [MarshalAs(UnmanagedType.LPStr)] string workloadName,
            out IntPtr outCertificateJson);

        // Section 16: Quantum-Photonic Co-Processor & Qubit Emulation
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_quantum_bell_state_simulate(
            nuint numQubits,
            out double outEntropy,
            out IntPtr outReportJson);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool cron_quantum_qft_simulate(
            nuint numQubits,
            out IntPtr outReportJson);
    }

    /// <summary>
    /// 256-Core Autonomous Multi-Agent Swarm Runtime on 4D-Torus.
    /// </summary>
    public sealed class CronSwarmMesh : IDisposable
    {
        private IntPtr _handle;
        private bool _disposed;

        public CronSwarmMesh()
        {
            _handle = NativeMethods.cron_swarm_create_256();
            if (_handle == IntPtr.Zero)
            {
                throw new InvalidOperationException("Failed to allocate 256-Core Swarm Mesh.");
            }
        }

        public string ExecuteTask(string task)
        {
            if (_disposed) throw new ObjectDisposedException(nameof(CronSwarmMesh));
            bool ok = NativeMethods.cron_swarm_execute_task(_handle, task, out IntPtr jsonPtr);
            if (!ok || jsonPtr == IntPtr.Zero)
            {
                throw new InvalidOperationException("Swarm execution failed on 4D-Torus.");
            }

            string report = Marshal.PtrToStringAnsi(jsonPtr) ?? string.Empty;
            NativeMethods.cron_string_free(jsonPtr);
            return report;
        }

        public void Dispose()
        {
            if (!_disposed)
            {
                if (_handle != IntPtr.Zero)
                {
                    NativeMethods.cron_swarm_free(_handle);
                    _handle = IntPtr.Zero;
                }
                _disposed = true;
            }
        }

        ~CronSwarmMesh() => Dispose();
    }

    /// <summary>
    /// 4,096-Core Multi-Chip Distributed Swarm Cluster (16 Chips x 256 Cores) on 6D-Torus.
    /// </summary>
    public sealed class CronClusterSwarmMesh : IDisposable
    {
        private IntPtr _handle;
        private bool _disposed;

        public CronClusterSwarmMesh()
        {
            _handle = NativeMethods.cron_cluster_swarm_create_4096();
            if (_handle == IntPtr.Zero)
            {
                throw new InvalidOperationException("Failed to allocate 4,096-Core Swarm Cluster.");
            }
        }

        public string ExecuteTask(string task)
        {
            if (_disposed) throw new ObjectDisposedException(nameof(CronClusterSwarmMesh));
            bool ok = NativeMethods.cron_cluster_swarm_execute_task(_handle, task, out IntPtr jsonPtr);
            if (!ok || jsonPtr == IntPtr.Zero)
            {
                throw new InvalidOperationException("Cluster swarm execution failed on 6D-Torus.");
            }

            string report = Marshal.PtrToStringAnsi(jsonPtr) ?? string.Empty;
            NativeMethods.cron_string_free(jsonPtr);
            return report;
        }

        public void Dispose()
        {
            if (!_disposed)
            {
                if (_handle != IntPtr.Zero)
                {
                    NativeMethods.cron_cluster_swarm_free(_handle);
                    _handle = IntPtr.Zero;
                }
                _disposed = true;
            }
        }

        ~CronClusterSwarmMesh() => Dispose();
    }

    /// <summary>
    /// Autonomous Swarm Self-Synthesis & Continuous Silicon Vibe-Healing Engine.
    /// </summary>
    public static class CronSwarmSynthesizer
    {
        public static (string code, string reportJson) SynthesizeAndHeal(
            string prompt,
            int maxIterations = 5,
            bool autoHeal = true,
            bool runJit = true)
        {
            if (string.IsNullOrEmpty(prompt)) throw new ArgumentNullException(nameof(prompt));

            bool ok = NativeMethods.cron_swarm_synthesize_and_heal(
                prompt,
                (nuint)maxIterations,
                autoHeal,
                runJit,
                out IntPtr codePtr,
                out IntPtr jsonPtr);

            if (!ok || codePtr == IntPtr.Zero || jsonPtr == IntPtr.Zero)
            {
                throw new InvalidOperationException("Autonomous swarm synthesis failed.");
            }

            string code = Marshal.PtrToStringAnsi(codePtr) ?? string.Empty;
            string report = Marshal.PtrToStringAnsi(jsonPtr) ?? string.Empty;

            NativeMethods.cron_string_free(codePtr);
            NativeMethods.cron_string_free(jsonPtr);

            return (code, report);
        }
    }

    /// <summary>
    /// Multi-Modal Vision Patch Processor with BitNet 1.58b Silicon Projection.
    /// </summary>
    public static class CronVisionProcessor
    {
        public static float[][] ExtractAndProjectPatches(byte[] rgbData, int width, int height, int patchSize = 16, int embedDim = 64)
        {
            if (rgbData == null || rgbData.Length == 0) return Array.Empty<float[]>();
            int maxPatches = (width / patchSize) * (height / patchSize);
            float[] buffer = new float[maxPatches * embedDim];

            bool ok = NativeMethods.cron_vision_extract_patches(
                rgbData,
                (nuint)width,
                (nuint)height,
                3,
                (nuint)patchSize,
                (nuint)embedDim,
                buffer,
                (nuint)buffer.Length,
                out nuint patchCount
            );
            if (!ok) throw new InvalidOperationException("Failed to extract vision patches.");

            float[][] result = new float[(int)patchCount][];
            for (int i = 0; i < (int)patchCount; i++)
            {
                result[i] = new float[embedDim];
                Array.Copy(buffer, i * embedDim, result[i], 0, embedDim);
            }
            return result;
        }
    }

    /// <summary>
    /// Multi-Modal Audio Log-Mel Spectrogram Processor.
    /// </summary>
    public static class CronAudioProcessor
    {
        public static float[][] ComputeMelSpectrogram(float[] pcmSamples, int sampleRate = 16000, int melBands = 80, int embedDim = 64)
        {
            if (pcmSamples == null || pcmSamples.Length == 0) return Array.Empty<float[]>();
            int estimatedFrames = pcmSamples.Length / 160 + 2;
            float[] buffer = new float[estimatedFrames * embedDim];

            bool ok = NativeMethods.cron_audio_mel_spectrogram(
                pcmSamples,
                (nuint)pcmSamples.Length,
                (nuint)sampleRate,
                (nuint)melBands,
                (nuint)embedDim,
                buffer,
                (nuint)buffer.Length,
                out nuint frameCount
            );
            if (!ok) throw new InvalidOperationException("Failed to compute audio spectrogram.");

            float[][] result = new float[(int)frameCount][];
            for (int i = 0; i < (int)frameCount; i++)
            {
                result[i] = new float[embedDim];
                Array.Copy(buffer, i * embedDim, result[i], 0, embedDim);
            }
            return result;
        }
    }

    /// <summary>
    /// Pure-Rust Native Subword BPE Tokenizer for Foundation Models.
    /// </summary>
    public sealed class CronBpeTokenizer : IDisposable
    {
        private IntPtr _handle;
        private bool _disposed;

        public CronBpeTokenizer()
        {
            _handle = NativeMethods.cron_bpe_create_default();
            if (_handle == IntPtr.Zero)
            {
                throw new InvalidOperationException("Failed to allocate CRON BPE Tokenizer.");
            }
        }

        public uint[] Encode(string text)
        {
            if (_disposed) throw new ObjectDisposedException(nameof(CronBpeTokenizer));
            if (string.IsNullOrEmpty(text)) return Array.Empty<uint>();

            uint[] buffer = new uint[text.Length * 4 + 32];
            bool ok = NativeMethods.cron_bpe_encode(_handle, text, buffer, (nuint)buffer.Length, out nuint actualLen);
            if (!ok) throw new InvalidOperationException("Failed to BPE encode text.");

            uint[] result = new uint[(int)actualLen];
            Array.Copy(buffer, result, (int)actualLen);
            return result;
        }

        public string Decode(uint[] tokenIds)
        {
            if (_disposed) throw new ObjectDisposedException(nameof(CronBpeTokenizer));
            if (tokenIds == null || tokenIds.Length == 0) return string.Empty;

            bool ok = NativeMethods.cron_bpe_decode(_handle, tokenIds, (nuint)tokenIds.Length, out IntPtr strPtr);
            if (!ok || strPtr == IntPtr.Zero) return string.Empty;

            string decoded = Marshal.PtrToStringAnsi(strPtr) ?? string.Empty;
            NativeMethods.cron_string_free(strPtr);
            return decoded;
        }

        public void Dispose()
        {
            if (!_disposed)
            {
                if (_handle != IntPtr.Zero)
                {
                    NativeMethods.cron_bpe_free(_handle);
                    _handle = IntPtr.Zero;
                }
                _disposed = true;
            }
        }

        ~CronBpeTokenizer() => Dispose();
    }

    /// <summary>
    /// Managed Wrapper for In-Process 256-Core 4D-Torus Virtual Machine.
    /// </summary>
    public sealed class CronVM : IDisposable
    {
        private IntPtr _handle;
        private bool _disposed;

        public CronVM()
        {
            _handle = NativeMethods.cron_vm_create();
            if (_handle == IntPtr.Zero)
            {
                throw new InvalidOperationException("Failed to allocate CRON 4D-Torus VM.");
            }
        }

        public void Reset()
        {
            ThrowIfDisposed();
            NativeMethods.cron_vm_reset(_handle);
        }

        public CronTelemetry Run(string crSource)
        {
            ThrowIfDisposed();
            var telemetry = new CronTelemetry();
            bool ok = NativeMethods.cron_vm_run_cr(_handle, crSource, ref telemetry);
            if (!ok)
            {
                throw new InvalidOperationException("Execution failed on 4D-Torus simulator.");
            }
            return telemetry;
        }

        public CronTelemetry RunCL(string clCode)
        {
            ThrowIfDisposed();
            var telemetry = new CronTelemetry();
            bool ok = NativeMethods.cron_vm_run_cl(_handle, clCode, ref telemetry);
            if (!ok)
            {
                throw new InvalidOperationException("VLIW execution failed on 4D-Torus simulator.");
            }
            return telemetry;
        }

        private void ThrowIfDisposed()
        {
            if (_disposed) throw new ObjectDisposedException(nameof(CronVM));
        }

        public void Dispose()
        {
            if (!_disposed)
            {
                if (_handle != IntPtr.Zero)
                {
                    NativeMethods.cron_vm_destroy(_handle);
                    _handle = IntPtr.Zero;
                }
                _disposed = true;
            }
        }

        ~CronVM() => Dispose();
    }

    /// <summary>
    /// In-memory CRON Compiler Driver.
    /// </summary>
    public static class CronCompiler
    {
        public static string Version => Marshal.PtrToStringAnsi(NativeMethods.cron_version()) ?? "unknown";
        public static string TargetInfo => Marshal.PtrToStringAnsi(NativeMethods.cron_target_info()) ?? "unknown";

        public static string CompileToVLIW(string crSource)
        {
            var ptr = NativeMethods.cron_compile_source(crSource, out var errPtr);
            if (ptr == IntPtr.Zero)
            {
                string err = Marshal.PtrToStringAnsi(errPtr) ?? "Compilation error";
                NativeMethods.cron_string_free(errPtr);
                throw new InvalidOperationException(err);
            }
            string result = Marshal.PtrToStringAnsi(ptr) ?? string.Empty;
            NativeMethods.cron_string_free(ptr);
            return result;
        }

        public static string CompileToC23(string crSource)
        {
            var ptr = NativeMethods.cron_compile_to_c23(crSource, out var errPtr);
            if (ptr == IntPtr.Zero)
            {
                string err = Marshal.PtrToStringAnsi(errPtr) ?? "C23 compilation error";
                NativeMethods.cron_string_free(errPtr);
                throw new InvalidOperationException(err);
            }
            string result = Marshal.PtrToStringAnsi(ptr) ?? string.Empty;
            NativeMethods.cron_string_free(ptr);
            return result;
        }

        public static string CompileToLLVM(string crSource)
        {
            var ptr = NativeMethods.cron_compile_to_llvm(crSource, out var errPtr);
            if (ptr == IntPtr.Zero)
            {
                string err = Marshal.PtrToStringAnsi(errPtr) ?? "LLVM compilation error";
                NativeMethods.cron_string_free(errPtr);
                throw new InvalidOperationException(err);
            }
            string result = Marshal.PtrToStringAnsi(ptr) ?? string.Empty;
            NativeMethods.cron_string_free(ptr);
            return result;
        }

        public static bool CheckSyntax(string crSource, out string errorMessage)
        {
            bool ok = NativeMethods.cron_check_syntax(crSource, out var errPtr);
            if (!ok && errPtr != IntPtr.Zero)
            {
                errorMessage = Marshal.PtrToStringAnsi(errPtr) ?? string.Empty;
                NativeMethods.cron_string_free(errPtr);
            }
            else
            {
                errorMessage = string.Empty;
            }
            return ok;
        }

        public static long ExecuteJit(string crSource)
        {
            long result = NativeMethods.cron_jit_execute(crSource, out var errPtr);
            if (errPtr != IntPtr.Zero)
            {
                string err = Marshal.PtrToStringAnsi(errPtr) ?? "JIT execution error";
                NativeMethods.cron_string_free(errPtr);
                throw new InvalidOperationException(err);
            }
            return result;
        }

        public static (byte[] Packed, float Scale) QuantizeTernary158(float[] weights)
        {
            if (weights == null || weights.Length == 0)
                return (Array.Empty<byte>(), 1.0f);

            int packedLen = (weights.Length + 3) / 4;
            byte[] packed = new byte[packedLen];
            bool ok = NativeMethods.cron_ternary_quantize(weights, (nuint)weights.Length, packed, out float scale);
            if (!ok) throw new InvalidOperationException("Failed to quantize weights to BitNet 1.58b ternary.");
            return (packed, scale);
        }

        public static float TernaryDotProduct(byte[] packedWeights, float[] activations, float scale)
        {
            if (packedWeights == null || activations == null || activations.Length == 0)
                return 0.0f;

            return NativeMethods.cron_ternary_dot_product(packedWeights, activations, (nuint)activations.Length, scale);
        }

        public static string AuditCl(string clSource)
        {
            bool ok = NativeMethods.cron_cl_audit(clSource, out var jsonPtr);
            if (!ok || jsonPtr == IntPtr.Zero)
                throw new InvalidOperationException("Failed to audit .cl code.");

            string result = Marshal.PtrToStringAnsi(jsonPtr) ?? string.Empty;
            NativeMethods.cron_string_free(jsonPtr);
            return result;
        }
    }

    /// <summary>
    /// Zero-Copy 4D Tensor for .NET 8 with direct Span&lt;T&gt; memory access.
    /// </summary>
    public sealed class CronTensor4D<T> : IDisposable where T : unmanaged
    {
        private IntPtr _handle;
        private readonly int _n, _c, _h, _w;
        private readonly int _totalElements;
        private bool _disposed;

        public CronTensor4D(int n, int c, int h, int w)
        {
            _n = n; _c = c; _h = h; _w = w;
            _totalElements = n * c * h * w;
            int elemType = typeof(T) switch
            {
                Type t when t == typeof(float) => 0,
                Type t when t == typeof(double) => 1,
                Type t when t == typeof(int) => 2,
                Type t when t == typeof(long) => 3,
                _ => throw new NotSupportedException($"Type {typeof(T)} is not a supported CRON tensor element.")
            };

            _handle = NativeMethods.cron_tensor4d_create(n, c, h, w, elemType);
            if (_handle == IntPtr.Zero)
            {
                throw new OutOfMemoryException("Failed to allocate 4D Tensor.");
            }
        }

        public unsafe Span<T> AsSpan()
        {
            if (_disposed) throw new ObjectDisposedException(nameof(CronTensor4D<T>));
            IntPtr dataPtr = NativeMethods.cron_tensor4d_data_ptr(_handle);
            return new Span<T>((void*)dataPtr, _totalElements);
        }

        public void Dispose()
        {
            if (!_disposed)
            {
                if (_handle != IntPtr.Zero)
                {
                    NativeMethods.cron_tensor4d_destroy(_handle);
                    _handle = IntPtr.Zero;
                }
                _disposed = true;
            }
        }

        ~CronTensor4D() => Dispose();
    }

    /// <summary>
    /// Interactive Real-Time TUI Swarm &amp; Torus Traffic Visualizer.
    /// Provides headless snapshot rendering and JSON telemetry export
    /// for the 256-Core 4D-Torus and 4,096-Core 16-Chip DWDM Optical Cluster.
    /// </summary>
    public static class CronSwarmTui
    {
        /// <summary>4D Torus Plane &amp; Spatial Core Matrix view.</summary>
        public const uint ViewTorusPlane = 1;
        /// <summary>16-Chip DWDM Optical Swarm Cluster Grid view.</summary>
        public const uint ViewClusterMacro = 2;
        /// <summary>9-Port Crossbar Switch &amp; VC Buffer Heatmap view.</summary>
        public const uint ViewRouterHeatmap = 3;
        /// <summary>Swarm Self-Synthesis &amp; Vibe-Healing Telemetry view.</summary>
        public const uint ViewSwarmTelemetry = 4;

        /// <summary>
        /// Advances the simulation by <paramref name="ticks"/> cycles and returns a
        /// rendered ANSI terminal frame snapshot.
        /// </summary>
        public static string RenderSnapshot(int ticks, uint viewMode = ViewTorusPlane, bool useColor = true)
        {
            bool ok = NativeMethods.cron_swarm_tui_render_snapshot(
                (nuint)ticks, viewMode, useColor, out IntPtr snapshotPtr);
            if (!ok || snapshotPtr == IntPtr.Zero)
                throw new InvalidOperationException("Failed to render swarm TUI snapshot.");

            string result = Marshal.PtrToStringAnsi(snapshotPtr) ?? string.Empty;
            NativeMethods.cron_string_free(snapshotPtr);
            return result;
        }

        /// <summary>
        /// Advances the simulation by <paramref name="ticks"/> cycles and returns
        /// a structured JSON telemetry string.
        /// </summary>
        public static string GetTelemetryJson(int ticks)
        {
            bool ok = NativeMethods.cron_swarm_tui_telemetry_json(
                (nuint)ticks, out IntPtr jsonPtr);
            if (!ok || jsonPtr == IntPtr.Zero)
                throw new InvalidOperationException("Failed to retrieve swarm TUI telemetry JSON.");

            string result = Marshal.PtrToStringAnsi(jsonPtr) ?? string.Empty;
            NativeMethods.cron_string_free(jsonPtr);
            return result;
        }
    }

    /// <summary>
    /// Autonomous Neuro-Symbolic MCTS (Monte Carlo Tree Search) Kernel Synthesizer.
    /// Formulates 4-way VLIW slot scheduling as an MDP solved via UCT search.
    /// </summary>
    public static class CronMctsSynthesizer
    {
        /// <summary>
        /// Synthesizes an optimal 4-way VLIW kernel for the given prompt using MCTS.
        /// </summary>
        public static (string code, string reportJson) Synthesize(string prompt, int simulations = 100, int rolloutDepth = 12)
        {
            bool ok = NativeMethods.cron_mcts_synthesize(
                prompt, (nuint)simulations, (nuint)rolloutDepth,
                out IntPtr codePtr, out IntPtr jsonPtr);

            if (!ok || codePtr == IntPtr.Zero || jsonPtr == IntPtr.Zero)
                throw new InvalidOperationException("Failed to synthesize kernel using MCTS.");

            string code = Marshal.PtrToStringAnsi(codePtr) ?? string.Empty;
            string reportJson = Marshal.PtrToStringAnsi(jsonPtr) ?? string.Empty;

            NativeMethods.cron_string_free(codePtr);
            NativeMethods.cron_string_free(jsonPtr);

            return (code, reportJson);
        }
    }

    /// <summary>
    /// Formal Mathematical Proof Verification Engine.
    /// Validates Dally-Seitz Deadlock-Freedom, Hoare Loop Invariants, WCET bounds,
    /// Landauer thermodynamic dissipation, and hardware CRC-8 slot authenticity.
    /// </summary>
    public static class CronProofVerifier
    {
        /// <summary>
        /// Verifies a machine kernel (.cl) and returns whether it is certified,
        /// along with the complete cryptographic proof certificate JSON.
        /// </summary>
        public static (bool isCertified, string certificateJson) Verify(string clCode, string workloadName = "CRON-Kernel")
        {
            bool certified = NativeMethods.cron_proof_verify(
                clCode, workloadName, out IntPtr certPtr);

            if (certPtr == IntPtr.Zero)
                throw new InvalidOperationException("Failed to generate formal proof certificate.");

            string certJson = Marshal.PtrToStringAnsi(certPtr) ?? string.Empty;
            NativeMethods.cron_string_free(certPtr);

            return (certified, certJson);
        }
    }

    /// <summary>
    /// Quantum-Photonic Co-Processor &amp; Qubit Emulation Backend.
    /// Simulates linear optical quantum circuits, MZI beam splitters, Bell/GHZ entanglement,
    /// and Quantum Fourier Transforms (QFT) with SU(2^N) state vector precision.
    /// </summary>
    public static class CronQuantumSimulator
    {
        /// <summary>
        /// Simulates a canonical Bell state (2 qubits) or GHZ state (3+ qubits).
        /// Returns bipartite von Neumann entanglement entropy and structured JSON report.
        /// </summary>
        public static (double entropy, string reportJson) SimulateBellState(int numQubits = 2)
        {
            bool ok = NativeMethods.cron_quantum_bell_state_simulate(
                (nuint)numQubits, out double entropy, out IntPtr jsonPtr);

            if (!ok || jsonPtr == IntPtr.Zero)
                throw new InvalidOperationException("Failed to simulate quantum Bell state.");

            string reportJson = Marshal.PtrToStringAnsi(jsonPtr) ?? string.Empty;
            NativeMethods.cron_string_free(jsonPtr);

            return (entropy, reportJson);
        }

        /// <summary>
        /// Simulates an N-qubit Quantum Fourier Transform (QFT).
        /// </summary>
        public static string SimulateQft(int numQubits = 3)
        {
            bool ok = NativeMethods.cron_quantum_qft_simulate(
                (nuint)numQubits, out IntPtr jsonPtr);

            if (!ok || jsonPtr == IntPtr.Zero)
                throw new InvalidOperationException("Failed to simulate Quantum Fourier Transform.");

            string reportJson = Marshal.PtrToStringAnsi(jsonPtr) ?? string.Empty;
            NativeMethods.cron_string_free(jsonPtr);

            return reportJson;
        }
    }
}


