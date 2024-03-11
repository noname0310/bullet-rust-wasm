#pragma once

// empty implementation for supressing intellisense errors
#ifdef __INTELLISENSE__
float sqrtf(float x) { return 0; }
    
float fabsf(float x) { return 0; }

float cosf(float x) { return 0; }
float sinf(float x) { return 0; }
float tanf(float x) { return 0; }

float acosf(float x) { return 0; }
float asinf(float x) { return 0; }
float atanf(float x) { return 0; }
float atan2f(float y, float x) { return 0; }

float expf(float x) { return 0; }
float logf(float x) { return 0; }
float powf(float x, float y) { return 0; }
float fmodf(float x, float y) { return 0; }
#else
extern "C" {
    float sqrtf(float x);

    float fabsf(float x);

    float cosf(float x);
    float sinf(float x);
    float tanf(float x);

    float acosf(float x);
    float asinf(float x);
    float atanf(float x);
    float atan2f(float y, float x);

    float expf(float x);
    float logf(float x);
    float powf(float x, float y);
    float fmodf(float x, float y);
}
#endif
