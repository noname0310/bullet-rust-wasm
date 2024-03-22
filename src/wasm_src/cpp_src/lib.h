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

#include <smmintrin.h>
#include "LinearMath/btMinMax.h"
#include "LinearMath/btScalar.h"
#include "LinearMath/btVector3.h"

extern "C" {
    extern void __wasm_call_ctors();
}

extern "C" int bt_get_version() {
    return btGetVersion();
}

extern "C" float bt_sin(float x) {
    const int* boxed = new int(3);
    delete boxed;

    return btSin(x);
}
