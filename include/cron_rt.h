/**
 * ============================================================================
 * CRON Native Runtime & C-ABI Shared Library (cron_rt.h)
 * Standard C99/C11 Interface for 256-Core 4D-Torus Cognitive Processor
 * ============================================================================
 */

#ifndef CRON_RT_H
#define CRON_RT_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

#if defined(_WIN32) || defined(__CYGWIN__)
  #ifdef CRON_RT_EXPORTS
    #define CRON_API __declspec(dllexport)
  #else
    #define CRON_API __declspec(dllimport)
  #endif
#else
  #define CRON_API __attribute__((visibility("default")))
#endif

/* ------------------------------------------------------------------------- */
/* 1. Telemetry & Hardware Types                                             */
/* ------------------------------------------------------------------------- */

typedef struct {
    uint64_t total_cycles;
    uint32_t active_cores;
    uint64_t optical_gemm_ops;
    uint64_t reversible_gate_ops;
    uint64_t stdp_synapse_updates;
    uint64_t mesh_packets_routed;
    uint32_t peak_temperature_c;
    double   dram_bandwidth_saved_mb;
    bool     success;
} cron_telemetry_t;

typedef struct cron_vm_t cron_vm_t;

typedef enum {
    CRON_DTYPE_F32 = 0,
    CRON_DTYPE_F64 = 1,
    CRON_DTYPE_I32 = 2,
    CRON_DTYPE_I64 = 3
} cron_dtype_t;

typedef struct {
    int32_t shape[4];    /* [N, C, H, W] */
    int32_t strides[4];  /* [C*H*W, H*W, W, 1] */
    int32_t elem_type;   /* cron_dtype_t */
    void*   data;        /* 64-byte aligned raw pointer */
    size_t  total_elements;
    size_t  size_bytes;
} cron_tensor4d_t;

/* ------------------------------------------------------------------------- */
/* 2. Metadata & Versioning                                                  */
/* ------------------------------------------------------------------------- */

CRON_API const char* cron_version(void);
CRON_API const char* cron_target_info(void);
CRON_API void        cron_string_free(char* s);

/* ------------------------------------------------------------------------- */
/* 3. In-Memory Compilation API (Zero-Disk I/O)                             */
/* ------------------------------------------------------------------------- */

CRON_API char*   cron_compile_source(const char* cr_source, char** out_error);
CRON_API char*   cron_compile_to_c23(const char* cr_source, char** out_error);
CRON_API char*   cron_compile_to_llvm(const char* cr_source, char** out_error);
CRON_API bool    cron_check_syntax(const char* cr_source, char** out_error);
CRON_API int64_t cron_jit_execute(const char* cr_source, char** out_error);

/* ------------------------------------------------------------------------- */
/* 4. 256-Core 4D-Torus Simulator Lifecycle & Execution                     */
/* ------------------------------------------------------------------------- */

CRON_API cron_vm_t* cron_vm_create(void);
CRON_API void       cron_vm_destroy(cron_vm_t* vm);
CRON_API void       cron_vm_reset(cron_vm_t* vm);
CRON_API bool       cron_vm_run_cl(cron_vm_t* vm, const char* cl_code, cron_telemetry_t* out_telemetry);
CRON_API bool       cron_vm_run_cr(cron_vm_t* vm, const char* cr_source, cron_telemetry_t* out_telemetry);

/* ------------------------------------------------------------------------- */
/* 5. Zero-Copy 4D Tensor Memory Bridge                                      */
/* ------------------------------------------------------------------------- */

CRON_API cron_tensor4d_t* cron_tensor4d_create(int32_t n, int32_t c, int32_t h, int32_t w, int32_t elem_type);
CRON_API void*            cron_tensor4d_data_ptr(cron_tensor4d_t* tensor);
CRON_API void             cron_tensor4d_shape(const cron_tensor4d_t* tensor, int32_t* out_shape);
CRON_API void             cron_tensor4d_destroy(cron_tensor4d_t* tensor);

/* ------------------------------------------------------------------------- */
/* 6. Hardware Photonic MZI Optical Accelerator                              */
/* ------------------------------------------------------------------------- */

CRON_API void cron_photonic_mzi_gemm(
    const double* amps,
    const double* phases,
    size_t count,
    double* out_amps,
    double* out_phases
);

/* ------------------------------------------------------------------------- */
/* 7. CSP Typed Channels & 4D-Torus Mesh Concurrency                         */
/* ------------------------------------------------------------------------- */

CRON_API uint64_t cron_channel_create_c(uint32_t capacity);
CRON_API int64_t  cron_channel_send_c(uint64_t channel_id, int64_t val);
CRON_API int64_t  cron_channel_recv_c(uint64_t channel_id);
CRON_API int64_t  cron_channel_try_recv_c(uint64_t channel_id);
CRON_API void     cron_channel_close_c(uint64_t channel_id);
CRON_API int64_t  cron_torus_distance_c(int64_t c1, int64_t c2);

/* ------------------------------------------------------------------------- */
/* 8. AI BitNet 1.58-Bit Ternary & CL 2.0 Native Silicon C-ABI               */
/* ------------------------------------------------------------------------- */

CRON_API bool    cron_ternary_quantize(const float* weights, size_t count, uint8_t* out_packed, float* out_scale);
CRON_API float   cron_ternary_dot_product(const uint8_t* packed_weights, const float* activations, size_t count, float scale);
CRON_API bool    cron_cl_audit(const char* cl_source, char** out_report_json);

/* ------------------------------------------------------------------------- */
/* 9. BPE Tokenizer & Autoregressive Inference C-ABI                         */
/* ------------------------------------------------------------------------- */

typedef struct CronBpeTokenizerOpaque CronBpeTokenizerOpaque;

CRON_API CronBpeTokenizerOpaque* cron_bpe_create_default(void);
CRON_API bool cron_bpe_encode(CronBpeTokenizerOpaque* tokenizer, const char* text, uint32_t* out_ids, size_t max_ids, size_t* out_len);
CRON_API bool cron_bpe_decode(CronBpeTokenizerOpaque* tokenizer, const uint32_t* ids, size_t num_ids, char** out_str);
CRON_API void cron_bpe_free(CronBpeTokenizerOpaque* tokenizer);

/* ------------------------------------------------------------------------- */
/* 10. Multi-Modal Vision & Audio Silicon C-ABI                              */
/* ------------------------------------------------------------------------- */

CRON_API bool cron_vision_extract_patches(
    const uint8_t* rgb_data,
    size_t width,
    size_t height,
    size_t channels,
    size_t patch_size,
    size_t embed_dim,
    float* out_embeddings,
    size_t max_elements,
    size_t* out_patch_count
);

CRON_API bool cron_audio_mel_spectrogram(
    const float* pcm_samples,
    size_t num_samples,
    size_t sample_rate,
    size_t mel_bands,
    size_t embed_dim,
    float* out_embeddings,
    size_t max_elements,
    size_t* out_frame_count
);

/* ------------------------------------------------------------------------- */
/* 11. 256-Core Autonomous Swarm Mesh C-ABI                                  */
/* ------------------------------------------------------------------------- */

typedef struct CronSwarmOpaque CronSwarmOpaque;

CRON_API CronSwarmOpaque* cron_swarm_create_256(void);
CRON_API bool cron_swarm_execute_task(CronSwarmOpaque* swarm, const char* task_desc, char** out_report_json);
CRON_API void cron_swarm_free(CronSwarmOpaque* swarm);

/* ------------------------------------------------------------------------- */
/* 12. 4,096-Core Multi-Chip Distributed Swarm Cluster C-ABI                  */
/* ------------------------------------------------------------------------- */

typedef struct CronClusterSwarmOpaque CronClusterSwarmOpaque;

CRON_API CronClusterSwarmOpaque* cron_cluster_swarm_create_4096(void);
CRON_API bool cron_cluster_swarm_execute_task(CronClusterSwarmOpaque* cluster, const char* task_desc, char** out_report_json);
CRON_API void cron_cluster_swarm_free(CronClusterSwarmOpaque* cluster);

#ifdef __cplusplus
}
#endif

#endif /* CRON_RT_H */

