#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <assert.h>
#include "clifford_rom.h"

// 8 Bundles ROM matching clifford_core_top.v
static const uint32_t ROM_WORDS[8] = {
    0x1E000001, // B0000: Mask 1111, Mode 00 (4 active -> 25% each), Next 1
    0x3A800002, // B0001: Mask 1101, Mode 01 (3 active -> [34,33,33]%), Next 2
    0x55000003, // B0002: Mask 1010, Mode 10 (2 active -> 50% each), Next 3
    0x71800004, // B0003: Mask 1000, Mode 11 (1 active -> 100%), Next 4
    0x80000005, // B0004: Mask 0000, Mode 00 (0 active -> Crystal Freeze), Next 5
    0xA9800006, // B0005: Mask 0100, Mode 11 (1 active -> 100%), Next 6
    0xCB000007, // B0006: Mask 0101, Mode 10 (2 active -> 50% each), Next 7
    0xFA800000  // B0007: Mask 1101, Mode 01 (3 active -> [34,33,33]%), Next 0
};

typedef struct {
    uint8_t w;
    uint8_t x;
    uint8_t z;
    uint8_t spin_phase;
    uint8_t volume;
} LaneRegister;

typedef struct {
    uint8_t pc;
    uint8_t r_depth;
    uint32_t cycle_cnt;
    LaneRegister lanes[4];
} CliffordCoreCPU;

void init_cpu(CliffordCoreCPU *cpu) {
    cpu->pc = 0;
    cpu->r_depth = 50;
    cpu->cycle_cnt = 0;
    for (int i = 0; i < 4; i++) {
        cpu->lanes[i].w = 50;
        cpu->lanes[i].x = i * 25;
        cpu->lanes[i].z = (3 - i) * 25;
        cpu->lanes[i].spin_phase = 0;
        cpu->lanes[i].volume = 25;
    }
}

void step_cycle(CliffordCoreCPU *cpu) {
    uint32_t instr = ROM_WORDS[cpu->pc];
    uint8_t act_mask = (instr >> 25) & 0x0F;
    uint8_t vol_mode = (instr >> 23) & 0x03;
    uint8_t next_pc  = instr & 0x07;

    // 1. Zero-Anchor Exact Bit Parity Dynamic Volume Balancer
    switch (vol_mode) {
        case 0: // 4 Active (25% each)
            for (int i = 0; i < 4; i++) {
                cpu->lanes[i].volume = (act_mask & (1 << (3 - i))) ? 25 : (act_mask == 0 ? 25 : 0);
            }
            break;
        case 1: { // 3 Active -> Rotating Parity [34, 33, 33]
            uint8_t rot = cpu->cycle_cnt % 3;
            uint8_t active_idx = 0;
            for (int i = 0; i < 4; i++) {
                if (act_mask & (1 << (3 - i))) {
                    cpu->lanes[i].volume = (active_idx == rot) ? 34 : 33;
                    active_idx++;
                } else {
                    cpu->lanes[i].volume = 0;
                }
            }
            break;
        }
        case 2: // 2 Active (50% each)
            for (int i = 0; i < 4; i++) {
                cpu->lanes[i].volume = (act_mask & (1 << (3 - i))) ? 50 : 0;
            }
            break;
        case 3: // 1 Active (100%)
            for (int i = 0; i < 4; i++) {
                cpu->lanes[i].volume = (act_mask & (1 << (3 - i))) ? 100 : 0;
            }
            break;
    }

    // 2. Spin-Orbit Coupling Step
    if (act_mask == 0 && cpu->r_depth < 99) {
        cpu->r_depth++;
    }

    for (int i = 0; i < 4; i++) {
        if (act_mask & (1 << (3 - i))) {
            // Spatial step
            cpu->lanes[i].x = (cpu->lanes[i].x + 1) % 100;
            cpu->lanes[i].z = (cpu->lanes[i].z + 1) % 100;
            cpu->lanes[i].w = cpu->r_depth;
        } else {
            // Spin step
            cpu->lanes[i].spin_phase = (cpu->lanes[i].spin_phase + 1) % 4;
        }
    }

    // 3. Mathematical Invariant Verification
    uint32_t total_vol = 0;
    for (int i = 0; i < 4; i++) total_vol += cpu->lanes[i].volume;
    assert(total_vol == 100);

    // Collision check
    for (int i = 0; i < 4; i++) {
        for (int j = i + 1; j < 4; j++) {
            bool surface_overlap = (cpu->lanes[i].x == cpu->lanes[j].x) && (cpu->lanes[i].z == cpu->lanes[j].z);
            if (surface_overlap) {
                // Must have depth difference >= 1
                assert(cpu->lanes[i].w != cpu->lanes[j].w);
            }
        }
    }

    printf("Cycle T%02u | Mask: %c%c%c%c | Depth: %02u | V: [%3u%%, %3u%%, %3u%%, %3u%%] | Sum: %3u%% | Spins: [%u,%u,%u,%u] | OK\n",
        cpu->cycle_cnt,
        (act_mask & 8) ? '1':'0', (act_mask & 4) ? '1':'0',
        (act_mask & 2) ? '1':'0', (act_mask & 1) ? '1':'0',
        cpu->r_depth,
        cpu->lanes[0].volume, cpu->lanes[1].volume, cpu->lanes[2].volume, cpu->lanes[3].volume,
        total_vol,
        cpu->lanes[0].spin_phase, cpu->lanes[1].spin_phase, cpu->lanes[2].spin_phase, cpu->lanes[3].spin_phase
    );

    cpu->pc = next_pc;
    cpu->cycle_cnt++;
}

int main() {
    CliffordCoreCPU cpu;
    init_cpu(&cpu);

    printf("================================================================================\n");
    printf(" C CYCLE-ACCURATE SIMULATOR: 4D CLIFFORD VERILOG HARDWARE CORE\n");
    printf("================================================================================\n");

    for (int i = 0; i < 16; i++) {
        step_cycle(&cpu);
    }

    printf("================================================================================\n");
    printf("[SUCCESS] All 16 cycles executed with EXACT 100%% Volume and ZERO bit leak!\n");
    printf("================================================================================\n");
    return 0;
}
