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
}
