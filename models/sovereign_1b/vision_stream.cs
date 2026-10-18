// ============================================================================
// CRON C# (.NET 8/9 Native AOT) Transpiled from .cl Microcode
// ============================================================================
using System;
using System.Numerics;
using System.Runtime.CompilerServices;

namespace Cron.Generated
{
    public static unsafe class CronKernel
    {
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

        public static void Execute(uint* registers, uint* pgasMemory, int* sphereAcc, int coreId = 0)
        {
            Span<uint> r = stackalloc uint[16];
            Span<int> sAcc = stackalloc int[4];

            // Bundle 0000
            r[1] = 0u;
            r[2] = 4u;
            // Bundle 0001
            r[1] = (uint)TernaryDot(r[1], r[1]);
            // Bundle 0002
            // Bundle 0003
            if (pgasMemory != null) pgasMemory[coreId * 16 + 0] = r[1];
            // Bundle 0004
            r[1] = 0u;
            r[2] = 4u;
            // Bundle 0005
            r[1] = (uint)TernaryDot(r[1], r[1]);
            // Bundle 0006
            // Bundle 0007
            if (pgasMemory != null) pgasMemory[coreId * 16 + 0] = r[1];
            // Bundle 0008
            r[1] = 0u;
            r[2] = 4u;
            // Bundle 0009
            r[1] = (uint)TernaryDot(r[1], r[1]);
            // Bundle 000A
            // Bundle 000B
            if (pgasMemory != null) pgasMemory[coreId * 16 + 0] = r[1];
            // Bundle 000C
            r[1] = 0u;
            r[2] = 4u;
            // Bundle 000D
            r[1] = (uint)TernaryDot(r[1], r[1]);
            // Bundle 000E
            // Bundle 000F
            if (pgasMemory != null) pgasMemory[coreId * 16 + 0] = r[1];
            return;

            if (registers != null)
            {
                for (int i = 0; i < 16; i++) registers[coreId * 16 + i] = r[i];
            }
            if (sphereAcc != null)
            {
                for (int s = 0; s < 4; s++) sphereAcc[coreId * 4 + s] = sAcc[s];
            }
        }
    }
}
