#include "LinearMath/btScalar.h"

extern "C" {
    extern void __wasm_call_ctors();
}

extern "C" int bt_get_version() {
    return btGetVersion();
}
