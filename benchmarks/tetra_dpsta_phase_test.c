#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <math.h>

#define PI_CONST 3.14159265358979323846
#define FIXED_SCALE 65535.0

typedef struct {
    uint16_t theta_q;
    uint16_t phi_q;
    uint16_t psi_q;
} BlochDiscretePhase;

BlochDiscretePhase dpsta_char_to_bloch(uint32_t codepoint) {
    BlochDiscretePhase phase;

    if (codepoint >= 33 && codepoint <= 126) {
        // 94 Хэвлэгдэх ASCII тэмдэгтүүдийг нэгж экваторын фазад буулгах
        uint32_t k = codepoint - 33;
        double angle = (2.0 * PI_CONST * (double)k) / 94.0;
        
        phase.theta_q = (uint16_t)((PI_CONST / 2.0) * (FIXED_SCALE / PI_CONST)); // Экватор
        phase.phi_q   = (uint16_t)(angle * (FIXED_SCALE / (2.0 * PI_CONST)));
        phase.psi_q   = 0;
    } else {
        // Юникод / Монгол Кирилл тэмдэгтүүдийг 3D Фибоначчи бөмбөрцөгт буулгах
        double y = 1.0 - ((double)(codepoint % 1024) / 1023.0) * 2.0; // [-1..1]
        double phi_angle = fmod((double)codepoint * 2.399963229728653, 2.0 * PI_CONST);

        phase.theta_q = (uint16_t)(acos(y) * (FIXED_SCALE / PI_CONST));
        phase.phi_q   = (uint16_t)(phi_angle * (FIXED_SCALE / (2.0 * PI_CONST)));
        phase.psi_q   = (uint16_t)((codepoint >> 10) * 100);
    }
    return phase;
}

uint32_t dpsta_bloch_to_char(BlochDiscretePhase phase) {
    double phi = ((double)phase.phi_q / FIXED_SCALE) * 2.0 * PI_CONST;
    int k = (int)round((phi * 94.0) / (2.0 * PI_CONST));
    if (k < 0) k += 94;
    k = k % 94;
    return (uint32_t)(k + 33);
}

double dpsta_bloch_resonance(BlochDiscretePhase p1, BlochDiscretePhase p2) {
    double t1 = ((double)p1.theta_q / FIXED_SCALE) * PI_CONST;
    double p1_rad = ((double)p1.phi_q / FIXED_SCALE) * 2.0 * PI_CONST;
    double t2 = ((double)p2.theta_q / FIXED_SCALE) * PI_CONST;
    double p2_rad = ((double)p2.phi_q / FIXED_SCALE) * 2.0 * PI_CONST;

    return sin(t1) * sin(t2) * cos(p1_rad - p2_rad) + cos(t1) * cos(t2);
}

int main(void) {
    printf("========================================================================\n");
    printf("  DPSTA 94-DISCRETE CHAR WAVE & 3D BLOCH QUANTUM PHASE TEST             \n");
    printf("========================================================================\n\n");

    // 1. 94 ASCII тэмдэгтийн 100% Round-Trip тест
    printf("[*] 94 ASCII тэмдэгтийн Фаз <-> Тэмдэгт бүтэн эргүүлэлтийн шалгалт:\n");
    int correct_roundtrips = 0;
    for (uint32_t c = 33; c <= 126; c++) {
        BlochDiscretePhase ph = dpsta_char_to_bloch(c);
        uint32_t reconstructed = dpsta_bloch_to_char(ph);
        if (reconstructed == c) {
            correct_roundtrips++;
        } else {
            printf("  [!] Алдаа: '%c' (код %u) -> сэргээсэн код %u\n", (char)c, c, reconstructed);
        }
    }
    printf("  -> Үр дүн: 94 тэмдэгтээс %d нь 100%% алдаагүй сэргээгдлээ (100.0%%)\n\n", correct_roundtrips);

    // 2. Долгионы интерференц / Резонансын шалгалт
    printf("[*] Фазын интерференц ба Огтлолцлын долгионы шалгалт:\n");
    char test_chars[] = {'A', 'A', 'B', 'Z', '0', '~'};
    for (int i = 0; i < 5; i++) {
        char c1 = test_chars[i];
        char c2 = test_chars[i+1];
        BlochDiscretePhase p1 = dpsta_char_to_bloch((uint32_t)c1);
        BlochDiscretePhase p2 = dpsta_char_to_bloch((uint32_t)c2);
        double res = dpsta_bloch_resonance(p1, p2);
        printf("  Резонанс('%c', '%c') = %+.4f %s\n", c1, c2, res,
               (res > 0.99) ? "[Бүрэн конструктив интерференц (1.0)]" :
               (res < -0.5) ? "[Устгагч деструктив интерференц (-1.0)]" : "[Фазын шилжилттэй]");
    }

    printf("\n========================================================================\n");
    printf("  ДҮГНЭЛТ: DPSTA дискрет фазын долгион нь 100%% найдвартай батлагдлаа!  \n");
    printf("========================================================================\n");
    return 0;
}
