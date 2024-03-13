fn main() {
    cc::Build::new()
        .archiver("llvm-ar")
        .cpp_link_stdlib(None)
        .cpp(true)
        .flag("-xc++")
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
