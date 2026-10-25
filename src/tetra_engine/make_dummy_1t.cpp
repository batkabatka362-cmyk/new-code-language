#include "tetra_engine.h"
#include <fstream>
#include <iostream>
#include <vector>
#include <cstring>

int main() {
    std::cout << "[*] 1T Тетраэдр суурь загварын жин үүсгэгч эхэллээ..." << std::endl;
    std::ofstream file("tetra_1t_foundation.bin", std::ios::binary);
    if (!file) {
        std::cerr << "[!] Алдаа: Файл үүсгэж чадсангүй!" << std::endl;
        return 1;
    }

    TetraModelHeader header;
    std::memset(&header, 0, sizeof(header));
    std::strncpy(header.magic, "TETRA1T", 7);
    header.version = 3;
    header.total_params = 1000000000000ULL; // 1 Trillion
    header.d_model = D_MODEL;
    header.num_layers = TOTAL_LAYERS;
    header.vocab_size = 131072;
    header.weights_offset = sizeof(TetraModelHeader);

    file.write((const char*)&header, sizeof(header));

    // 80 давхарга тус бүрт өөр өөрийн давхаргын фазын эргэлттэй жин үүсгэх
    size_t layer_blocks = NUM_BLOCKS_PER_LAYER;
    std::vector<TetraBlockDisk> layer_buf(layer_blocks);

    for (uint32_t l = 0; l < TOTAL_LAYERS; l++) {
        uint32_t layer_phase_shift = (l * 7) % 32;
        uint32_t layer_mask = (l * 0x04081020) ^ 0x13579BDF;

        for (size_t b = 0; b < layer_blocks; b++) {
            uint32_t base_v0 = 0xAAAAAAAA ^ (uint32_t)(b * 0x01010101);
            uint32_t base_v1 = 0x55555555 + (uint32_t)(b * 3);
            uint32_t base_v2 = 0x33333333 ^ (uint32_t)(b << 2);
            uint32_t base_v3 = 0x0F0F0F0F + (uint32_t)(b * 7);

            layer_buf[b].v0 = rotl32(base_v0, layer_phase_shift) ^ layer_mask;
            layer_buf[b].v1 = rotl32(base_v1, (layer_phase_shift + 3) % 32) + layer_mask;
            layer_buf[b].v2 = rotl32(base_v2, (layer_phase_shift + 5) % 32) ^ ~layer_mask;
            layer_buf[b].v3 = rotl32(base_v3, (layer_phase_shift + 9) % 32);
            layer_buf[b].micro_gain = (uint8_t)((b + l) % 16); // gamma in [0..15]
        }

        file.write((const char*)layer_buf.data(), layer_blocks * sizeof(TetraBlockDisk));
    }

    std::cout << "[+] Давхарга бүр өөр фазын эргэлттэй 1.03-бит жингийн файл амжилттай үүсгэгдлээ!" << std::endl;
    std::cout << "    Нийт давхарга: " << TOTAL_LAYERS << ", Блок/давхарга: " << NUM_BLOCKS_PER_LAYER << std::endl;
    return 0;
}
