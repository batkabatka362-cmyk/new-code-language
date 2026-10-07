// ============================================================================
// CRON Hardware Toolchain — Verilator VCD Waveform Tracer Emulation Header
// ============================================================================

#ifndef VERILATED_VCD_C_H_
#define VERILATED_VCD_C_H_

#include <cstdint>
#include <string>

class VerilatedVcdC {
public:
    void open(const char* filename) {
        (void)filename;
    }
    void dump(uint64_t time) {
        (void)time;
    }
    void close() {}
};

#endif // VERILATED_VCD_C_H_
