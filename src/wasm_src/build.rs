fn main() {
    cc::Build::new()
        .warnings(false)
        .archiver("llvm-ar")
        .cpp_link_stdlib(None)
        .cpp(true)
        .flag("-xc++")
        .flag("-matomics")
        .flag("-mbulk-memory")
        .flag("-msimd128")
        .flag("-Wno-c++11-narrowing")
        .flag("-Wno-deprecated-declarations")
        .flag("-Wno-#pragma-messages")
        .includes([
            "cpp_wasm_std"
        ])
        .files([
            "cpp_src/lib.h",
            "cpp_src/LinearMath/btScalar.h"
        ])
        .compile("bullet");

    println!("cargo:rerun-if-changed=cpp_wasm_std");
    println!("cargo:rerun-if-changed=cpp_src/lib.h");
    println!("cargo:rerun-if-changed=cpp_src/LinearMath/btScalar.h");
}
