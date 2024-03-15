#pragma once

extern "C" {
    float bw_sqrtf(float x);

    float bw_fabsf(float x);

    float bw_cosf(float x);
    float bw_sinf(float x);
    float bw_tanf(float x);

    float bw_acosf(float x);
    float bw_asinf(float x);
    float bw_atanf(float x);
    float bw_atan2f(float y, float x);

    float bw_expf(float x);
    float bw_logf(float x);
    float bw_powf(float x, float y);
    float bw_fmodf(float x, float y);
}

inline float sqrtf(float x) {
    return bw_sqrtf(x);
}

inline float fabsf(float x) {
    return bw_fabsf(x);
}

inline float cosf(float x) {
    return bw_cosf(x);
}

inline float sinf(float x) {
    return bw_sinf(x);
}

inline float tanf(float x) {
    return bw_tanf(x);
}

inline float acosf(float x) {
    return bw_acosf(x);
}

inline float asinf(float x) {
    return bw_asinf(x);
}

inline float atanf(float x) {
    return bw_atanf(x);
}

inline float atan2f(float y, float x) {
    return bw_atan2f(y, x);
}

inline float expf(float x) {
    return bw_expf(x);
}

inline float logf(float x) {
    return bw_logf(x);
}

inline float powf(float x, float y) {
    return bw_powf(x, y);
}

inline float fmodf(float x, float y) {
    return bw_fmodf(x, y);
}
