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

#include <smmintrin.h>
#include "LinearMath/btScalar.h"

extern "C" {
    extern void __wasm_call_ctors();
}

extern "C" int bt_get_version() {
    return btGetVersion();
}

extern "C" float bt_sin(float x) {
    return btSin(x);
}
