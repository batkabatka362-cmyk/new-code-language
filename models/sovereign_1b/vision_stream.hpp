// ============================================================================
// CRON C++20 Header-Only Native Transpiled from .cl Microcode
// Multiplier-Free 0 FLOPs BitNet b1.58 & 4D Sphere Geodesic Execution
// ============================================================================
#pragma once

#include <cstdint>
#include <cstddef>
#include <array>
#include <bit>

namespace cron::generated {

[[nodiscard]] inline int32_t ternary_dot(uint32_t a, uint32_t b) noexcept {
    const uint32_t lo_a = a & 0x55555555u;
    const uint32_t hi_a = (a >> 1u) & 0x55555555u;
    const uint32_t pos_a = lo_a & ~hi_a;
    const uint32_t neg_a = ~lo_a & hi_a;
    const uint32_t lo_b = b & 0x55555555u;
    const uint32_t hi_b = (b >> 1u) & 0x55555555u;
    const uint32_t pos_b = lo_b & ~hi_b;
    const uint32_t neg_b = ~lo_b & hi_b;
    const uint32_t plus_matches = (pos_a & pos_b) | (neg_a & neg_b);
    const uint32_t minus_matches = (pos_a & neg_b) | (neg_a & pos_b);
    return static_cast<int32_t>(std::popcount(plus_matches)) - static_cast<int32_t>(std::popcount(minus_matches));
}

inline int32_t sphere_step(uint32_t /*s_id*/, uint32_t /*w*/, uint32_t lat_x, uint32_t lon_z, uint32_t quad, uint32_t val, uint32_t& r_val, uint32_t alu_op, uint32_t r_shift) noexcept {
    const uint32_t cur_wt = ((lat_x + lon_z) >> 5u) + 1u;
    const int32_t q_sign = (quad == 1u || quad == 4u) ? 1 : -1;
    const int32_t geo_term = q_sign * static_cast<int32_t>((val * cur_wt) >> r_shift);
    const uint32_t scaled = val * cur_wt;
    switch (alu_op) {
        case 1u: r_val += scaled; break;
        case 2u: r_val -= scaled; break;
        case 3u: r_val ^= (scaled << r_shift); break;
        case 4u: r_val ^= scaled; break;
        case 5u: r_val = scaled; break;
        default: break;
    }
    return geo_term;
}

inline void execute_kernel(uint32_t* registers, uint32_t* pgas_mem, int32_t* sphere_acc, size_t core_id = 0) noexcept {
    std::array<uint32_t, 16> r{};
    std::array<int32_t, 4> s_acc{};

    // Bundle 0000
    r[1] = 0u;
    r[2] = 4u;
    // Bundle 0001
    r[1] = static_cast<uint32_t>(ternary_dot(r[1], r[1]));
    // Bundle 0002
    // Bundle 0003
    if (pgas_mem) pgas_mem[core_id * 16u + 0u] = r[1];
    // Bundle 0004
    r[1] = 0u;
    r[2] = 4u;
    // Bundle 0005
    r[1] = static_cast<uint32_t>(ternary_dot(r[1], r[1]));
    // Bundle 0006
    // Bundle 0007
    if (pgas_mem) pgas_mem[core_id * 16u + 0u] = r[1];
    // Bundle 0008
    r[1] = 0u;
    r[2] = 4u;
    // Bundle 0009
    r[1] = static_cast<uint32_t>(ternary_dot(r[1], r[1]));
    // Bundle 000A
    // Bundle 000B
    if (pgas_mem) pgas_mem[core_id * 16u + 0u] = r[1];
    // Bundle 000C
    r[1] = 0u;
    r[2] = 4u;
    // Bundle 000D
    r[1] = static_cast<uint32_t>(ternary_dot(r[1], r[1]));
    // Bundle 000E
    // Bundle 000F
    if (pgas_mem) pgas_mem[core_id * 16u + 0u] = r[1];
    return;

    if (registers) {
        for (size_t i = 0; i < 16; ++i) registers[core_id * 16 + i] = r[i];
    }
    if (sphere_acc) {
        for (size_t s = 0; s < 4; ++s) sphere_acc[core_id * 4 + s] = s_acc[s];
    }
}

} // namespace cron::generated
