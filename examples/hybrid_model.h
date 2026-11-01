#ifndef CRON_HYBRID_MODEL_H
#define CRON_HYBRID_MODEL_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

#if defined(_WIN32) || defined(__CYGWIN__)
  #ifdef CRON_BUILD_DLL
    #define CRON_API __declspec(dllexport)
  #else
    #define CRON_API __declspec(dllimport)
  #endif
#else
  #define CRON_API __attribute__((visibility("default")))
#endif

CRON_API void F(const float* W, const float* x, float* out);
CRON_API void N(const float* x, const float* γ, float* out);
CRON_API void P(const float* A, const float* B, float* out);

#ifdef __cplusplus
}
#endif

#endif // CRON_HYBRID_MODEL_H
