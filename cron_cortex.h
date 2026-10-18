#ifndef CRON_CORTEX_H
#define CRON_CORTEX_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

#ifdef _WIN32
  #ifdef CRON_EXPORTS
    #define CRON_API __declspec(dllexport)
  #else
    #define CRON_API __declspec(dllimport)
  #endif
#else
  #define CRON_API __attribute__((visibility("default")))
#endif

typedef struct CronSiliconCore CronSiliconCore;

typedef struct {
    uint64_t cycle_count;
    uint64_t optical_gemm_count;
    uint64_t reversible_ops_count;
    uint64_t stdp_updates_count;
    uint64_t spatial_broadcast_count;
    uint64_t barrier_count;
    uint64_t wave_interference_ops_count;
    uint64_t deflections_count;
    uint32_t hamiltonian;
    uint32_t sensory_w;
    uint32_t sensory_x;
    uint32_t sensory_z;
    uint32_t sensory_spin;
} CronCortexTelemetry;

CRON_API CronSiliconCore* cron_cortex_create(void);
CRON_API void cron_cortex_destroy(CronSiliconCore* core);
CRON_API void cron_cortex_reset(CronSiliconCore* core);
CRON_API void cron_cortex_step(CronSiliconCore* core);
CRON_API void cron_cortex_run(CronSiliconCore* core, uint64_t cycles);
CRON_API uint32_t cron_cortex_get_reg(CronSiliconCore* core, uint32_t reg_idx);
CRON_API void cron_cortex_set_reg(CronSiliconCore* core, uint32_t reg_idx, uint32_t val);
CRON_API void cron_cortex_inject_token(CronSiliconCore* core, uint8_t token_byte);
CRON_API uint32_t cron_cortex_get_hamiltonian(CronSiliconCore* core);
CRON_API void cron_cortex_get_sensory(CronSiliconCore* core, uint32_t* out_w, uint32_t* out_x, uint32_t* out_z, uint32_t* out_spin);
CRON_API void cron_cortex_get_telemetry(CronSiliconCore* core, CronCortexTelemetry* out_tel);
CRON_API void cron_sphere_linear_forward(int32_t* out, const int32_t* in, const int8_t* ternary_weights, uint32_t batch_size, uint32_t in_dim, uint32_t out_dim);
CRON_API void cron_sphere_swiglu(int32_t* out, const int32_t* gate, const int32_t* up, uint32_t size);
CRON_API void cron_sphere_quantize_ternary(int8_t* out_ternary, float* out_scale, const float* in_floats, uint32_t count);

#ifdef __cplusplus
}
#endif

#endif // CRON_CORTEX_H
