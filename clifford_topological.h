#ifndef CLIFFORD_TOPOLOGICAL_ROTOR_H
#define CLIFFORD_TOPOLOGICAL_ROTOR_H

#include <stdint.h>

typedef struct __attribute__((aligned(16))) {
    uint64_t low64;
    uint64_t high64;
} cl_universe_128_t;

static const cl_universe_128_t topological_rotor_rom[2] = {
    { 0x2864F05163C28A1EULL, 0x0191919191000000ULL }, // BalanceState
    { 0x2966F45973E2CA9FULL, 0x0002121223900000ULL }, // TriSpiralParity
};

#endif // CLIFFORD_TOPOLOGICAL_ROTOR_H
