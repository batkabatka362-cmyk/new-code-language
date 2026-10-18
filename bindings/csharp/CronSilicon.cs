// ============================================================================
// CRON Sovereign Interop: C# (.NET 8/9 Native AOT) P/Invoke Bindings (CronNative.cs)
// ============================================================================
using System;
using System.Runtime.InteropServices;

namespace Cron.Interop
{
    public static unsafe class CronNative
    {
        private const string LibName = "cronc";

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl, EntryPoint = "cron_mind_create")]
        public static extern void* MindCreate();

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl, EntryPoint = "cron_mind_free")]
        public static extern void MindFree(void* ptr);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl, EntryPoint = "cron_mind_process")]
        public static extern int MindProcess(void* ptr, byte* inputCstr, byte* outBuf, nuint maxLen);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl, EntryPoint = "cron_cl_jit_exec")]
        public static extern int ClJitExec(byte* clSourceCstr, uint* outRegs);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl, EntryPoint = "cron_hdc_similarity")]
        public static extern float HdcSimilarity(sbyte* vecA, sbyte* vecB);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl, EntryPoint = "cron_infer_forward_ffi")]
        public static extern int InferForward(byte* packedWeights, float* inputs, float scale, nuint rows, nuint cols, float* outActivations);
    }
}
