fn main() {
    cc::Build::new()
        .archiver("llvm-ar")
        .cpp_link_stdlib(None)
        .cpp(true)
        .files([
            "cpp_src/LinearMath/btScalar.h"
        ])
        .compile("foolib");
}
