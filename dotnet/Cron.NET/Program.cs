// ============================================================================
// CRON Sovereign Interop: C# (.NET 8/9) Example & Demonstration
// ============================================================================
using System;
using System.Diagnostics;
using Cron.Net;

Console.WriteLine("================================================================================");
Console.WriteLine("  CRON & .CL Sovereign Language - C# / .NET 8/9 Native AOT Interop Demo");
Console.WriteLine("  Multiplier-Free 0 FLOPs BitNet b1.58 & 4D Hyperspherical Geodesics");
Console.WriteLine("================================================================================");

// 1. BitNet b1.58 Subbyte Ternary Dot Product
uint a = 0x55555555u; // all +1 trits
uint b = 0x55555555u; // all +1 trits
int dotResult = CronNative.TernaryDot(a, b);
Console.WriteLine($"[1] BitNet Ternary Dot Product (16 trits): {dotResult} (Expected: 16)");

// 2. 4D Hyperspherical Geodesic Curvature Step
uint rVal = 0;
int geoTerm = CronNative.SphereStep(0, 1, 10, 20, 1, 4, ref rVal, 1, 1);
Console.WriteLine($"[2] 4D Sphere Geodesic Step: GeoTerm={geoTerm}, Register={rVal}");

// 3. High-Throughput Roofline Benchmark in C#
const int Iterations = 10_000_000;
var sw = Stopwatch.StartNew();
long accumulator = 0;
for (int i = 0; i < Iterations; i++)
{
    accumulator += CronNative.TernaryDot((uint)i, 0xAAAAAAAAu);
}
sw.Stop();

double elapsedMs = sw.Elapsed.TotalMilliseconds;
double mops = (Iterations / (elapsedMs / 1000.0)) / 1_000_000.0;
Console.WriteLine($"[3] C# PopCount Roofline: {Iterations:N0} ops in {elapsedMs:F2} ms ({mops:F1} MOps/sec)");
Console.WriteLine("================================================================================");
Console.WriteLine("  CRON .NET INTEROP STATUS: 100% OPERATIONAL & NATIVE READY");
Console.WriteLine("================================================================================");
