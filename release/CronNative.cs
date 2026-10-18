// ============================================================================
// CRON Sovereign Interop: C# (.NET 8/9 Native AOT) P/Invoke Bindings (CronNative.cs)
// Multiplier-Free BitNet b1.58 & 4D Hyperspherical Geodesic Engine
// ============================================================================
using System;
using System.Numerics;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Cron.Net
{
    public static unsafe class CronNative
    {
        private const string LibName = "cronc";

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static int TernaryDot(uint a, uint b)
        {
            uint loA = a & 0x55555555u;
            uint hiA = (a >> 1) & 0x55555555u;
            uint posA = loA & ~hiA;
            uint negA = ~loA & hiA;
            uint loB = b & 0x55555555u;
            uint hiB = (b >> 1) & 0x55555555u;
            uint posB = loB & ~hiB;
            uint negB = ~loB & hiB;
            uint plus = (posA & posB) | (negA & negB);
            uint minus = (posA & negB) | (negA & posB);
            return BitOperations.PopCount(plus) - BitOperations.PopCount(minus);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static int SphereStep(uint sId, uint w, uint latX, uint lonZ, uint quad, uint val, ref uint rVal, uint aluOp, int rShift)
        {
            uint curWt = ((latX + lonZ) >> 5) + 1u;
            int qSign = (quad == 1u || quad == 4u) ? 1 : -1;
            int geoTerm = qSign * (int)((val * curWt) >> rShift);
            uint scaled = val * curWt;
            switch (aluOp)
            {
                case 1u: rVal += scaled; break;
                case 2u: rVal -= scaled; break;
                case 3u: rVal ^= (scaled << rShift); break;
                case 4u: rVal ^= scaled; break;
                case 5u: rVal = scaled; break;
            }
            return geoTerm;
        }

        // Native C-ABI P/Invoke exports
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
