#include "tetra_engine.h"
#include <cstring>
#include <string>
#include <vector>
#include <memory>
#include <iostream>

#ifdef _WIN32
#include <windows.h>
#define EXPORT_API __declspec(dllexport)
#else
#define EXPORT_API __attribute__((visibility("default")))
#endif

class TetraEngineImpl {
private:
#ifdef _WIN32
    HANDLE m_file_handle = INVALID_HANDLE_VALUE;
    HANDLE m_map_handle = NULL;
#else
    int m_file_fd = -1;
#endif
    size_t m_file_size = 0;
    uint8_t* m_mmap_weights = nullptr;
    TetraModelHeader m_header;

    std::vector<GpuTetraEdge> m_k_cache;
    size_t m_current_seq_len = 0;
    float m_temperature = 0.0f;
    uint32_t m_rng_state = 0x87654321;

public:
    TetraEngineImpl() = default;

    void set_temperature(float temp) { m_temperature = temp; }
    void set_seed(uint32_t seed) { m_rng_state = seed ? seed : 0x87654321; }

    ~TetraEngineImpl() {
#ifdef _WIN32
        if (m_mmap_weights) UnmapViewOfFile(m_mmap_weights);
        if (m_map_handle) CloseHandle(m_map_handle);
        if (m_file_handle != INVALID_HANDLE_VALUE) CloseHandle(m_file_handle);
#endif
    }

    bool init(const char* model_path) {
#ifdef _WIN32
        m_file_handle = CreateFileA(model_path, GENERIC_READ, FILE_SHARE_READ, NULL,
                                    OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
        if (m_file_handle == INVALID_HANDLE_VALUE) return false;

        LARGE_INTEGER size_li;
        GetFileSizeEx(m_file_handle, &size_li);
        m_file_size = (size_t)size_li.QuadPart;

        m_map_handle = CreateFileMappingA(m_file_handle, NULL, PAGE_READONLY, 0, 0, NULL);
        if (!m_map_handle) {
            CloseHandle(m_file_handle);
            return false;
        }

        m_mmap_weights = (uint8_t*)MapViewOfFile(m_map_handle, FILE_MAP_READ, 0, 0, 0);
        if (!m_mmap_weights) {
            CloseHandle(m_map_handle);
            CloseHandle(m_file_handle);
            return false;
        }
#endif
        std::memcpy(&m_header, m_mmap_weights, sizeof(TetraModelHeader));
        return (std::strncmp(m_header.magic, "TETRA1T", 7) == 0);
    }

    size_t get_num_blocks() const {
        return (m_header.d_model >= TETRA_BLOCK_WEIGHTS) ? (m_header.d_model / TETRA_BLOCK_WEIGHTS) : 1;
    }

    char step(char input_char) {
        uint8_t uc = (uint8_t)input_char;
        uint32_t k = (uc >= DPSTA_CHAR_BASE && uc < DPSTA_CHAR_BASE + DPSTA_CHAR_COUNT) ? 
                     (uc - DPSTA_CHAR_BASE) : 0;
        uint32_t char_phase = k * DPSTA_HARMONIC_MULT;
        uint32_t pos_phase = rotl32((uint32_t)m_current_seq_len * 0x9E3779B9, (unsigned int)(m_current_seq_len % 32));
        uint32_t total_phase = char_phase ^ pos_phase;

        size_t num_blocks = get_num_blocks();
        std::vector<GpuTetraEdge> current_wave(num_blocks);
        std::vector<GpuTetraEdge> next_wave(num_blocks);

        for (size_t b = 0; b < num_blocks; b++) {
            uint32_t block_hash = rotl32(total_phase, (unsigned int)(b * 3));
            current_wave[b].e0 = block_hash ^ 0xAAAAAAAA;
            current_wave[b].e1 = block_hash ^ 0x55555555;
            current_wave[b].e2 = block_hash ^ 0x33333333;
            current_wave[b].e3 = block_hash ^ 0x0F0F0F0F;
            current_wave[b].e4 = block_hash ^ 0x00FF00FF;
            current_wave[b].e5 = block_hash ^ 0x5A5A5A5A;
            current_wave[b].micro_gain = 8;
        }

        if (!m_k_cache.empty()) {
            int best_score = -999999;
            size_t best_idx = 0;
            for (size_t t = 0; t < m_k_cache.size(); t++) {
                uint32_t m0 = ~(current_wave[0].e0 ^ m_k_cache[t].e0);
                uint32_t m1 = ~(current_wave[0].e1 ^ m_k_cache[t].e1);
                int score = (__builtin_popcount(m0) + __builtin_popcount(m1) - 32);
                if (score > best_score) {
                    best_score = score;
                    best_idx = t;
                }
            }
            uint32_t attn_feedback = m_k_cache[best_idx].e0 ^ rotl32(m_k_cache[best_idx].e1, 5);
            for (size_t b = 0; b < num_blocks; b++) {
                current_wave[b].e0 ^= rotl32(attn_feedback, (unsigned int)(b % 16));
            }
        }

        m_k_cache.push_back(current_wave[0]);
        if (m_k_cache.size() > 4096) {
            m_k_cache.erase(m_k_cache.begin());
        }

        const uint8_t* layer_ptr = m_mmap_weights + m_header.weights_offset;
        size_t raw_layer_bytes = num_blocks * sizeof(TetraBlockDisk);

        for (uint32_t l = 0; l < m_header.num_layers; l++) {
            const TetraBlockDisk* disk_blks = (const TetraBlockDisk*)(layer_ptr + (l * raw_layer_bytes));
            for (size_t b = 0; b < num_blocks; b++) {
                uint32_t wt_e0 = disk_blks[b].v0 ^ disk_blks[b].v1;
                uint32_t wt_e1 = disk_blks[b].v1 ^ disk_blks[b].v2;
                uint32_t wt_e2 = disk_blks[b].v2 ^ disk_blks[b].v0;
                uint32_t wt_e3 = disk_blks[b].v0 ^ disk_blks[b].v3;
                uint32_t wt_e4 = disk_blks[b].v1 ^ disk_blks[b].v3;
                uint32_t wt_e5 = disk_blks[b].v2 ^ disk_blks[b].v3;

                next_wave[b].e0 = tetra_hysteresis_gate(current_wave[b].e0 ^ wt_e0, wt_e1);
                next_wave[b].e1 = tetra_hysteresis_gate(current_wave[b].e1 ^ wt_e1, wt_e2);
                next_wave[b].e2 = tetra_hysteresis_gate(current_wave[b].e2 ^ wt_e2, wt_e3);
                next_wave[b].e3 = tetra_hysteresis_gate(current_wave[b].e3 ^ wt_e3, wt_e4);
                next_wave[b].e4 = tetra_hysteresis_gate(current_wave[b].e4 ^ wt_e4, wt_e5);
                next_wave[b].e5 = tetra_hysteresis_gate(current_wave[b].e5 ^ wt_e5, wt_e0);
            }
            std::memcpy(current_wave.data(), next_wave.data(), num_blocks * sizeof(GpuTetraEdge));
        }

        uint64_t tree_hash = 0;
        for (size_t b = 0; b < num_blocks; b++) {
            uint32_t plane_mix = current_wave[b].e0 ^ rotl32(current_wave[b].e1, 7) ^
                                 rotl32(current_wave[b].e2, 13) ^ rotl32(current_wave[b].e3, 19);
            uint64_t p64 = ((uint64_t)plane_mix << 32) | (uint64_t)current_wave[b].e4;
            tree_hash ^= rotl64(p64, (unsigned int)b);
        }

        uint32_t char_idx = 0;
        for (int lvl = 0; lvl < 9; lvl++) {
            uint32_t det_branch = (uint32_t)((tree_hash >> (lvl * 2)) & 0x03);
            uint32_t branch = tetra_sample_facet_branch(det_branch, tree_hash, lvl, m_temperature, &m_rng_state);
            char_idx = (char_idx * 4) + branch;
        }

        m_current_seq_len++;
        return (char)(DPSTA_CHAR_BASE + (char_idx % DPSTA_CHAR_COUNT));
    }
};

struct TetraEngineHandle {
    TetraEngineImpl* impl;
};

extern "C" {

EXPORT_API TetraEngineHandle* tetra_create_engine(const char* model_path) {
    if (!model_path) return nullptr;
    auto impl = new TetraEngineImpl();
    if (!impl->init(model_path)) {
        delete impl;
        return nullptr;
    }
    auto handle = new TetraEngineHandle();
    handle->impl = impl;
    return handle;
}

EXPORT_API void tetra_set_temperature(TetraEngineHandle* handle, float temp) {
    if (handle && handle->impl) handle->impl->set_temperature(temp);
}

EXPORT_API void tetra_set_seed(TetraEngineHandle* handle, uint32_t seed) {
    if (handle && handle->impl) handle->impl->set_seed(seed);
}

EXPORT_API char tetra_step(TetraEngineHandle* handle, char input_char) {
    if (!handle || !handle->impl) return ' ';
    return handle->impl->step(input_char);
}

EXPORT_API int tetra_generate_text(
    TetraEngineHandle* handle,
    const char* prompt,
    size_t max_tokens,
    char* out_buffer,
    size_t out_buffer_size
) {
    if (!handle || !handle->impl || !prompt || !out_buffer || out_buffer_size == 0) return -1;

    size_t prompt_len = std::strlen(prompt);
    char current_char = ' ';

    for (size_t i = 0; i < prompt_len; ++i) {
        current_char = handle->impl->step(prompt[i]);
    }

    size_t gen_count = 0;
    while (gen_count < max_tokens && (gen_count + 1) < out_buffer_size) {
        current_char = handle->impl->step(current_char);
        out_buffer[gen_count++] = current_char;
    }
    out_buffer[gen_count] = '\0';
    return (int)gen_count;
}

EXPORT_API void tetra_destroy_engine(TetraEngineHandle* handle) {
    if (!handle) return;
    if (handle->impl) delete handle->impl;
    delete handle;
}

} // extern "C"
