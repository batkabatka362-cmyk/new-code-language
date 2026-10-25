#include "tetra_engine.h"
#include <iostream>
#include <fstream>
#include <cstring>
#include <chrono>
#include <vector>

#ifdef _WIN32
#include <windows.h>
#else
#include <fcntl.h>
#include <unistd.h>
#include <sys/mman.h>
#include <sys/stat.h>
#endif

class Tetra1TInferenceEngine {
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
    TetraHarmonicRingBuffer m_ring_buffer;
    size_t m_current_seq_len = 0;
    float m_temperature = 0.0f;
    uint32_t m_rng_state = 0x12345678;

public:
    Tetra1TInferenceEngine() {
        tetra_ring_init(&m_ring_buffer);
    }

    void set_temperature(float temp) { m_temperature = temp; }
    void set_seed(uint32_t seed) { m_rng_state = seed ? seed : 0x12345678; }

    ~Tetra1TInferenceEngine() {
        cleanup();
    }

    bool init(const std::string& model_path) {
        std::cout << "[*] 1T Загварын хоёртын файлыг дискнээс нээж байна: " << model_path << std::endl;

#ifdef _WIN32
        m_file_handle = CreateFileA(model_path.c_str(), GENERIC_READ, FILE_SHARE_READ, NULL,
                                    OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
        if (m_file_handle == INVALID_HANDLE_VALUE) {
            std::cerr << "[!] Алдаа: Загварын файл нээгдсэнгүй: " << model_path << std::endl;
            return false;
        }

        LARGE_INTEGER size_li;
        GetFileSizeEx(m_file_handle, &size_li);
        m_file_size = (size_t)size_li.QuadPart;

        m_map_handle = CreateFileMappingA(m_file_handle, NULL, PAGE_READONLY, 0, 0, NULL);
        if (!m_map_handle) {
            std::cerr << "[!] Алдаа: CreateFileMapping амжилтгүй боллоо!" << std::endl;
            CloseHandle(m_file_handle);
            return false;
        }

        m_mmap_weights = (uint8_t*)MapViewOfFile(m_map_handle, FILE_MAP_READ, 0, 0, 0);
        if (!m_mmap_weights) {
            std::cerr << "[!] Алдаа: MapViewOfFile амжилтгүй боллоо!" << std::endl;
            CloseHandle(m_map_handle);
            CloseHandle(m_file_handle);
            return false;
        }
#else
        m_file_fd = open(model_path.c_str(), O_RDONLY);
        if (m_file_fd < 0) {
            std::cerr << "[!] Алдаа: Загварын файл нээгдсэнгүй: " << model_path << std::endl;
            return false;
        }
        struct stat sb;
        fstat(m_file_fd, &sb);
        m_file_size = sb.st_size;

        m_mmap_weights = (uint8_t*)mmap(NULL, m_file_size, PROT_READ, MAP_SHARED, m_file_fd, 0);
        if (m_mmap_weights == MAP_FAILED) {
            std::cerr << "[!] Алдаа: mmap амжилтгүй боллоо!" << std::endl;
            close(m_file_fd);
            return false;
        }
#endif

        std::memcpy(&m_header, m_mmap_weights, sizeof(TetraModelHeader));
        if (std::strncmp(m_header.magic, "TETRA1T", 7) != 0) {
            std::cerr << "[!] Алдаа: Хүчингүй Тетраэдр 1T загварын файл!" << std::endl;
            return false;
        }

        std::cout << "[+] Загвар амжилттай холбогдлоо. Нийт параметр: " << m_header.total_params
                  << " (" << (m_file_size / (1024.0 * 1024.0)) << " MB уншлаа)" << std::endl;
        std::cout << "    Давхарга: " << m_header.num_layers << ", Далд хэмжээс: " << m_header.d_model << std::endl;
        return true;
    }

    size_t get_num_blocks() const {
        return (m_header.d_model >= TETRA_BLOCK_WEIGHTS) ? (m_header.d_model / TETRA_BLOCK_WEIGHTS) : 1;
    }

    // Текстийг DPSTA долгион + Ротари Байрлалын Фаз (RoPE Phase Shift) болгох
    void encode_dpsta_wave(char c, size_t pos, GpuTetraEdge* out_edge_block, size_t num_blocks) {
        uint8_t uc = (uint8_t)c;
        uint32_t k = (uc >= DPSTA_CHAR_BASE && uc < DPSTA_CHAR_BASE + DPSTA_CHAR_COUNT) ? 
                     (uc - DPSTA_CHAR_BASE) : 0;
        uint32_t char_phase = k * DPSTA_HARMONIC_MULT;
        // Алтан харьцааны ротари байрлалын вектор (Rotary Positional Phase)
        uint32_t pos_phase = rotl32((uint32_t)pos * 0x9E3779B9, (unsigned int)(pos % 32));
        uint32_t total_phase = char_phase ^ pos_phase;

        for (size_t b = 0; b < num_blocks; b++) {
            uint32_t block_hash = rotl32(total_phase, (unsigned int)(b * 3));
            out_edge_block[b].e0 = block_hash ^ 0xAAAAAAAA;
            out_edge_block[b].e1 = block_hash ^ 0x55555555;
            out_edge_block[b].e2 = block_hash ^ 0x33333333;
            out_edge_block[b].e3 = block_hash ^ 0x0F0F0F0F;
            out_edge_block[b].e4 = block_hash ^ 0x00FF00FF;
            out_edge_block[b].e5 = block_hash ^ 0x5A5A5A5A;
            out_edge_block[b].micro_gain = 8;
        }
    }

    void apply_resonance_attention(GpuTetraEdge* wave, size_t num_blocks) {
        if (m_k_cache.empty()) return;

        int best_score = -999999;
        size_t best_idx = 0;

        for (size_t t = 0; t < m_k_cache.size(); t++) {
            uint32_t m0 = ~(wave[0].e0 ^ m_k_cache[t].e0);
            uint32_t m1 = ~(wave[0].e1 ^ m_k_cache[t].e1);
            int score = (__builtin_popcount(m0) + __builtin_popcount(m1) - 32);
            if (score > best_score) {
                best_score = score;
                best_idx = t;
            }
        }

        uint32_t attn_feedback = m_k_cache[best_idx].e0 ^ rotl32(m_k_cache[best_idx].e1, 5);
        for (size_t b = 0; b < num_blocks; b++) {
            wave[b].e0 ^= rotl32(attn_feedback, (unsigned int)(b % 16));
        }
    }

    char step_forward(char input_char) {
        size_t num_blocks = get_num_blocks();
        std::vector<GpuTetraEdge> current_wave(num_blocks);
        std::vector<GpuTetraEdge> next_wave(num_blocks);
        encode_dpsta_wave(input_char, m_current_seq_len, current_wave.data(), num_blocks);

        apply_resonance_attention(current_wave.data(), num_blocks);
        current_wave[0].e0 = tetra_ring_resonate(&m_ring_buffer, current_wave[0].e0);

        uint64_t wave_slice = ((uint64_t)current_wave[0].e0 << 32) | (uint64_t)current_wave[0].e1;
        tetra_ring_push(&m_ring_buffer, wave_slice);

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
                uint8_t wt_gain = disk_blks[b].micro_gain;

                next_wave[b].e0 = tetra_hysteresis_gate(current_wave[b].e0 ^ wt_e0, wt_e1);
                next_wave[b].e1 = tetra_hysteresis_gate(current_wave[b].e1 ^ wt_e1, wt_e2);
                next_wave[b].e2 = tetra_hysteresis_gate(current_wave[b].e2 ^ wt_e2, wt_e3);
                next_wave[b].e3 = tetra_hysteresis_gate(current_wave[b].e3 ^ wt_e3, wt_e4);
                next_wave[b].e4 = tetra_hysteresis_gate(current_wave[b].e4 ^ wt_e4, wt_e5);
                next_wave[b].e5 = tetra_hysteresis_gate(current_wave[b].e5 ^ wt_e5, wt_e0);
                next_wave[b].micro_gain = (current_wave[b].micro_gain + wt_gain) >> 1;
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

        char predicted_char = (char)(DPSTA_CHAR_BASE + (char_idx % DPSTA_CHAR_COUNT));
        m_current_seq_len++;
        return predicted_char;
    }

    void generate(const std::string& prompt, size_t max_new_tokens, float temperature = 0.0f) {
        m_temperature = temperature;
        std::cout << "\n[Промпт урсгаж байна]: " << prompt << std::flush;
        char last_char = ' ';
        for (char c : prompt) {
            last_char = step_forward(c);
        }

        std::cout << "\n[1.03-Бит Тетраэдр Үүсгэж эхэллээ (T=" << m_temperature << ")]: " << std::flush;
        auto t0 = std::chrono::high_resolution_clock::now();

        for (size_t i = 0; i < max_new_tokens; i++) {
            last_char = step_forward(last_char);
            std::cout << last_char << std::flush;
        }

        auto t1 = std::chrono::high_resolution_clock::now();
        double dur_sec = std::chrono::duration<double>(t1 - t0).count();
        std::cout << "\n\n[*] Хурд: " << (max_new_tokens / dur_sec) << " токен/сек ("
                  << (dur_sec / max_new_tokens * 1000.0) << " ms/токен)" << std::endl;
    }

private:
    void cleanup() {
#ifdef _WIN32
        if (m_mmap_weights) UnmapViewOfFile(m_mmap_weights);
        if (m_map_handle) CloseHandle(m_map_handle);
        if (m_file_handle != INVALID_HANDLE_VALUE) CloseHandle(m_file_handle);
#else
        if (m_mmap_weights && m_mmap_weights != MAP_FAILED) {
            munmap(m_mmap_weights, m_file_size);
        }
        if (m_file_fd >= 0) close(m_file_fd);
#endif
    }
};

int main(int argc, char** argv) {
    std::string model_file = "tetra_1t_foundation.bin";
    std::string prompt = "Sain baina uu, Mongol uls!";
    size_t tokens_to_generate = 40;
    float temperature = 0.7f;

    if (argc >= 2) model_file = argv[1];
    if (argc >= 3) prompt = argv[2];
    if (argc >= 4) tokens_to_generate = (size_t)std::stoul(argv[3]);
    if (argc >= 5) temperature = std::stof(argv[4]);

    Tetra1TInferenceEngine engine;
    if (!engine.init(model_file)) {
        return 1;
    }

    engine.generate(prompt, tokens_to_generate, temperature);
    return 0;
}
