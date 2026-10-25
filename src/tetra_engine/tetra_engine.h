#ifndef TETRA_ENGINE_H
#define TETRA_ENGINE_H

#ifdef __cplusplus
#include <cmath>
#include <cstddef>
#include <cstdint>
#else
#include <math.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#ifndef alignas
#define alignas(x) _Alignas(x)
#endif
#endif

#define TETRA_BLOCK_WEIGHTS 128
#define D_MODEL 8192
#define NUM_BLOCKS_PER_LAYER (D_MODEL / TETRA_BLOCK_WEIGHTS) // 64 блок
#define TOTAL_LAYERS 80

// DPSTA 95 Discrete Characters: Space (32) to Tilde (126)
#define DPSTA_CHAR_BASE 32
#define DPSTA_CHAR_COUNT 95
#define DPSTA_HARMONIC_MULT 0x02C9FB4D

// 1.03125-бит Тетраэдр Блок (132-бит физик дүрслэл: 32 тетраэдр + 4-бит
// микро-масштаб)
#pragma pack(push, 1)
struct TetraBlockDisk {
  uint32_t v0;
  uint32_t v1;
  uint32_t v2;
  uint32_t v3;
  uint8_t micro_gain; // Бага 4 бит: gamma in [0..15]
};
#pragma pack(pop)

// 32-байт санах ойд зэрэгцүүлсэн Ирмэгийн Бүрэн Төлөв
#if defined(__GNUC__) || defined(__clang__)
struct __attribute__((aligned(32))) GpuTetraEdge {
#else
struct alignas(32) GpuTetraEdge {
#endif
  uint32_t e0, e1, e2, e3, e4, e5;
  uint8_t micro_gain;
  uint8_t pad[7];
};

// 1T Загварын Хоёртын Файлын Толгой Бүтэц
struct TetraModelHeader {
  char magic[8];           // "TETRA1T\0"
  uint32_t version;        // 3
  uint64_t total_params;   // 1,000,000,000,000
  uint32_t d_model;        // 8192
  uint32_t num_layers;     // 80
  uint32_t vocab_size;     // 131072 (Tetra-Tree 9 түвшин)
  uint64_t weights_offset; // Файл доторх жингийн эхлэх байт
};

// Хурдан бит эргүүлэлт (RotL32 & RotL64)
static inline uint32_t rotl32(uint32_t val, unsigned int shift) {
  shift &= 31;
  return (val << shift) | (val >> (32 - shift));
}

static inline uint64_t rotl64(uint64_t val, unsigned int shift) {
  shift &= 63;
  return (val << shift) | (val >> (64 - shift));
}

// 4.3 Бүлэг: Hysteresis Gate (Резонансын босго шүүлтүүр)
// Хэт жижиг хэлбэлзлийг шүүж, үндсэн фазыг туйлшруулна
static inline uint32_t tetra_hysteresis_gate(uint32_t wave,
                                             uint32_t ref_plane) {
  uint32_t xor_diff = wave ^ ref_plane;
  // Тэгш хэмийн тэнцвэржүүлэгч маск
  uint32_t polarized = wave ^ (rotl32(xor_diff, 5) & 0x55555555);
  return polarized;
}

// 32-битийн хурдан PRNG (xorshift32)
static inline uint32_t tetra_xorshift32(uint32_t *state) {
  uint32_t x = *state;
  x ^= x << 13;
  x ^= x >> 17;
  x ^= x << 5;
  *state = x;
  return x;
}

// 4-төлөвт Тетраэдр Модны Фаз / Температур Тооцоолол (Boltzmann Sampling)
// branch in [0..3]
static inline uint32_t tetra_sample_facet_branch(uint32_t det_branch,
                                                 uint64_t tree_hash, int lvl,
                                                 float temperature,
                                                 uint32_t *rng_state) {
  if (temperature <= 0.01f || !rng_state) {
    return det_branch & 0x03; // Детерминистик Argmax
  }

  // Тетраэдрын тэгш хэмт 4 талст дээрх энергийн түвшин (Logits)
  // Детерминистик талст нь хамгийн өндөр оноотой (+2.5), бусад 3 хөрш нь бага
  // оноотой (-0.5)
  float logits[4];
  float max_l = -1e9f;
  for (int i = 0; i < 4; i++) {
    float base_score = (i == (int)(det_branch & 0x03)) ? 2.5f : -0.5f;
    int harmonic_bias = (int)((tree_hash >> (lvl * 4 + i)) & 1);
    logits[i] = (base_score + (harmonic_bias ? 0.3f : -0.3f)) / temperature;
    if (logits[i] > max_l)
      max_l = logits[i];
  }

  float sum_exp = 0.0f;
  float probs[4];
  for (int i = 0; i < 4; i++) {
#ifdef __cplusplus
    probs[i] = std::exp(logits[i] - max_l);
#else
    probs[i] = expf(logits[i] - max_l);
#endif
    sum_exp += probs[i];
  }

  uint32_t rand_val = tetra_xorshift32(rng_state);
  float r = (float)(rand_val & 0xFFFFFF) / (float)0x1000000;
  float cum = 0.0f;
  for (int i = 0; i < 4; i++) {
    cum += probs[i] / sum_exp;
    if (r <= cum || i == 3) {
      return (uint32_t)i;
    }
  }
  return det_branch & 0x03;
}

// 4.4 Бүлэг: Harmonic Ring Buffer (Урт хугацааны семантик фазын цагираг санах ой - SSM)
// 256-битийн эргэлдэх фазын аккумлятор (4 x uint64_t = 256 бит, O(1) санах ой)
#define TETRA_RING_CAPACITY 4

struct TetraHarmonicRingBuffer {
  uint64_t phase_slots[TETRA_RING_CAPACITY];
  uint32_t head;
  uint32_t count;
};

static inline void tetra_ring_init(struct TetraHarmonicRingBuffer *rb) {
  for (int i = 0; i < TETRA_RING_CAPACITY; i++) {
    rb->phase_slots[i] = 0ULL;
  }
  rb->head = 0;
  rb->count = 0;
}

static inline void tetra_ring_push(struct TetraHarmonicRingBuffer *rb,
                                   uint64_t wave_slice) {
  // Алтан огтлол ба гармоник мушгиралтаар өмнөх фазыг шилжүүлэн хадгална
  uint64_t twisted = rotl64(wave_slice ^ 0x9E3779B97F4A7C15ULL, 7);
  rb->phase_slots[rb->head] = twisted;
  rb->head = (rb->head + 1) % TETRA_RING_CAPACITY;
  if (rb->count < TETRA_RING_CAPACITY) {
    rb->count++;
  }
}

// Контекстийн резонанс: Одоогийн долгионыг цагираг санах ойн фазуудтай харьцуулж
// урт хугацааны семантик хазайлтыг тооцоолно
static inline uint32_t tetra_ring_resonate(const struct TetraHarmonicRingBuffer *rb,
                                           uint32_t current_wave) {
  if (!rb || rb->count == 0) return current_wave;
  uint64_t acc = 0;
  for (uint32_t i = 0; i < rb->count; i++) {
    acc ^= rotl64(rb->phase_slots[i], (unsigned int)((i * 13) & 63));
  }
  uint32_t ring_harmonics = (uint32_t)(acc ^ (acc >> 32));
  return tetra_hysteresis_gate(current_wave, ring_harmonics);
}

#endif // TETRA_ENGINE_H

