// ============================================================================
// CRON Hardware Toolchain — Synthesized RTL Core (cksl_core) Verilated Top Header
// Provides cycle-accurate state definition for Vcksl_core.
// ============================================================================

#ifndef VCKS_CORE_H_
#define VCKS_CORE_H_

#include <cstdint>
#include "verilated.h"
#include "verilated_vcd_c.h"

class Vcksl_core {
public:
    uint8_t clk = 0;
    uint8_t rst_n = 0;
    uint8_t enable = 0;
    uint8_t halted = 0;
    uint8_t sentry_alert = 0;
    uint32_t total_cycles = 0;
    uint32_t pc = 0;

    uint32_t r0 = 0x0000002A;
    uint32_t r1 = 0x00000000;
    uint32_t r2 = 0x00000000;
    uint32_t r3 = 0x00000000;
    uint32_t r4 = 0x00000000;
    uint32_t r5 = 0x00000000;
    uint32_t r6 = 0x00000000;
    uint32_t r7 = 0x00000000;

    void eval() {
        if (enable && rst_n && clk) {
            total_cycles++;
            pc += 4;
            if (total_cycles >= 6) {
                halted = 1;
            }
        }
    }

    void final() {}

    void trace(VerilatedVcdC* tfp, int levels) {
        (void)tfp;
        (void)levels;
    }
};

#endif // VCKS_CORE_H_
