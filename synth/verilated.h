// ============================================================================
// CRON Hardware Toolchain — Verilator Core Emulation Header
// Provides fallback definitions when Verilator is not locally installed.
// ============================================================================

#ifndef VERILATED_H_
#define VERILATED_H_

#include <cstdint>
#include <string>

typedef uint64_t vluint64_t;

class Verilated {
public:
    static void commandArgs(int argc, char** argv) {
        (void)argc;
        (void)argv;
    }
    static void traceEverOn(bool flag) {
        (void)flag;
    }
    static bool gotFinish() {
        return false;
    }
};

#endif // VERILATED_H_
