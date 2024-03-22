// for use simd instructions
#define _WIN32
#define _MSC_VER 1401
#define __i386__
#define __SSE__
#define __SSE2__
#define __SSE3__
#define __SSSE3__
#define __SSE4_1__

#define BT_USE_SSE
#define BT_USE_SSE_IN_API
#define BT_NO_SIMD_OPERATOR_OVERLOADS
#define BT_USE_SIMD_VECTOR3

// for manual global memory initialization

extern "C" {
    extern void __wasm_call_ctors();
}

//

// override allocation functions

#include <stdlib.h>

extern "C" {
    void* bw_malloc(size_t size);
    void bw_free(void* ptr);
}

inline void* malloc(size_t size) noexcept {
    return bw_malloc(size);
}

inline void free(void* ptr) noexcept {
    bw_free(ptr);
}

void* operator new(size_t size) {
    return bw_malloc(size);
}

void* operator new(size_t size, void* ptr) noexcept {
    return ptr;
}

void* operator new[](size_t size) {
    return bw_malloc(size);
}

void* operator new[](size_t size, void* ptr) noexcept {
    return ptr;
}

void operator delete(void* ptr) noexcept {
    bw_free(ptr);
}

void operator delete(void* ptr, size_t size) noexcept {
    bw_free(ptr);
}

void operator delete[](void* ptr) noexcept {
    bw_free(ptr);
}

void operator delete[](void* ptr, size_t size) noexcept {
    bw_free(ptr);
}

//

// for use SIMD instructions

#include <smmintrin.h>

//

#include "LinearMath/btMinMax.h"
#include "LinearMath/btScalar.h"
#include "LinearMath/btVector3.h"

// test extern functions

extern "C" int bt_get_version() {
    return btGetVersion();
}

extern "C" float bt_sin(float x) {
    return btSin(x);
}

extern "C" int* bt_alloc_int() {
    int* boxed = new int(3);
    return boxed;
}

extern "C" int* bt_nonallocnew_test() {
    void* ptr = malloc(sizeof(int));
    int* boxed = new(ptr) int(3);
    return boxed;
}

extern "C" void bt_free_int(int* ptr) {
    delete ptr;
}

//
