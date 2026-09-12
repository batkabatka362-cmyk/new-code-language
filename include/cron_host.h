/**
 * CRON AGI 4D-Torus Processor: Official C/C++ Host Driver Header
 * Target: 256-Core (4x4x4x4) Neuromorphic / Photonic Silicon
 * 
 * Provides C-compatible ABI interfaces to load, simulate, inspect, and 
 * coordinate execution on the 256-core 4D-Torus cognitive accelerator.
 */

#ifndef CRON_HOST_H
#define CRON_HOST_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ========================================================================= */
/* 1. Core ABI Register Aliases (Stackless Register Allocation)               */
/* ========================================================================= */
#define CRON_REG_RV   0   /* $rv  : Return Value / Primary Accumulator       */
#define CRON_REG_A0   1   /* $a0  : Function Argument 0 / Graph Pointer      */
#define CRON_REG_A1   2   /* $a1  : Function Argument 1 / Phase Key          */
#define CRON_REG_A2   3   /* $a2  : Function Argument 2 / Quad-Pack Reference*/
#define CRON_REG_T0   4   /* $t0  : Caller-Saved Temp 0 / Neural Intermediary*/
#define CRON_REG_T1   5   /* $t1  : Caller-Saved Temp 1                      */
#define CRON_REG_T2   6   /* $t2  : Caller-Saved Temp 2                      */
#define CRON_REG_S0   7   /* $s0  : Callee-Saved 0 / Spatial Pipe Latch      */
#define CRON_REG_S1   8   /* $s1  : Callee-Saved 1                           */
#define CRON_REG_S2   9   /* $s2  : Callee-Saved 2                           */
#define CRON_REG_IM0  10  /* $im0 : Brain 5 Chaos Latent State (Lorenz X)    */
#define CRON_REG_IM1  11  /* $im1 : Brain 5 Chaos Latent State (Lorenz Y)    */
#define CRON_REG_IM2  12  /* $im2 : Brain 5 Chaos Latent State (Lorenz Z)    */
#define CRON_REG_AR0  13  /* $ar0 : Brain 6 Arbiter Decision Meta Weight 0   */
#define CRON_REG_AR1  14  /* $ar1 : Brain 6 Arbiter Decision Meta Weight 1   */
#define CRON_REG_BP   15  /* $bp  : Base Tile Pointer / Region Arena Base    */

/* ========================================================================= */
/* 2. Control and Status Registers (CSR) IDs                                 */
/* ========================================================================= */
#define CRON_CSR_CYCLE_CNT     0x00  /* Total execution cycles               */
#define CRON_CSR_PRED_EXEC_CNT 0x01  /* Predicated instructions executed     */
#define CRON_CSR_STALL_CNT     0x02  /* Hazard / NOP stall cycles            */
#define CRON_CSR_VEC_BURST_CNT 0x03  /* Full 128-bit vector burst operations */
#define CRON_CSR_TRAP_CNT      0x04  /* Hardware exception / trap events     */
#define CRON_CSR_LFSR_STATE    0x05  /* 32-bit Galois LFSR PRNG state        */
#define CRON_CSR_MCAUSE        0x06  /* Machine Trap Cause                   */
#define CRON_CSR_MEPC          0x07  /* Machine Exception Program Counter    */
#define CRON_CSR_THERMAL_LVL   0x08  /* Dynamic Thermal Level (°C)           */

/* ========================================================================= */
/* 3. 6-Brain Cognitive Engine Identifiers                                   */
/* ========================================================================= */
typedef enum {
    CRON_BRAIN_SYMBOLIC     = 1, /* Brain 1: Causal Hyper-Edge Graph Unifier */
    CRON_BRAIN_PHOTONIC     = 2, /* Brain 2: BitNet 1.58b Photonic MZI GEMM  */
    CRON_BRAIN_REVERSIBLE   = 3, /* Brain 3: Quantum Phase / 0-Entropy Memory*/
    CRON_BRAIN_NEUROMORPHIC = 4, /* Brain 4: SNN LIF Spikes & STDP Plasticity*/
    CRON_BRAIN_CHAOS        = 5, /* Brain 5: Lorenz Attractor Chaos Diffusion*/
    CRON_BRAIN_ARBITER      = 6  /* Brain 6: AGI Master Arbiter & Sentry     */
} cron_brain_id_t;

/* ========================================================================= */
/* 4. 4D Torus Mesh Coordinates                                              */
/* ========================================================================= */
typedef struct {
    uint8_t x; /* 0..3 */
    uint8_t y; /* 0..3 */
    uint8_t z; /* 0..3 */
    uint8_t w; /* 0..3 */
} cron_coord_4d_t;

/* ========================================================================= */
/* 5. Driver Telemetry & Performance Statistics                              */
/* ========================================================================= */
typedef struct {
    uint64_t total_cycles;
    uint64_t optical_gemm_ops;
    uint64_t reversible_ops;
    uint64_t stdp_plasticity_updates;
    uint64_t mesh_packets_routed;
    uint64_t predicated_ops;
    uint64_t trap_events;
    double   dram_bandwidth_saved_mb;
    double   slot_utilization_pct;
} cron_hw_stats_t;

/* ========================================================================= */
/* 6. Host Driver Function Declarations                                      */
/* ========================================================================= */

/**
 * Initialize the 256-core 4D-Torus execution runtime.
 * @return Opaque handle to simulator instance, or NULL on error.
 */
void* cron_driver_init(void);

/**
 * Load a 128-bit VLIW machine code (.cl) program string into the simulator.
 */
int cron_driver_load_cl(void* handle, const char* cl_source);

/**
 * Single-step the 256-core processor by 1 clock cycle.
 * @return true if program is continuing, false if halted.
 */
bool cron_driver_step(void* handle);

/**
 * Run the simulator to completion or breakpoint.
 */
int cron_driver_run(void* handle);

/**
 * Read register value for a specific core.
 * @param core_id 0..255
 * @param reg_idx 0..15 (CRON_REG_*)
 */
uint32_t cron_driver_read_reg(void* handle, uint32_t core_id, uint32_t reg_idx);

/**
 * Read Control and Status Register (CSR) value for a specific core.
 * @param core_id 0..255
 * @param csr_id  CRON_CSR_*
 */
uint32_t cron_driver_read_csr(void* handle, uint32_t core_id, uint32_t csr_id);

/**
 * Retrieve aggregated hardware performance and cognitive telemetry.
 */
int cron_driver_get_stats(void* handle, cron_hw_stats_t* out_stats);

/**
 * Trigger 1-cycle hardware shadow bank checkpoint on a core.
 */
void cron_driver_checkpoint(void* handle, uint32_t core_id);

/**
 * Clean up and free the runtime simulator.
 */
void cron_driver_teardown(void* handle);

#ifdef __cplusplus
}
#endif

#endif /* CRON_HOST_H */
