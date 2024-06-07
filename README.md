# bullet-rust-wasm

Experimental project to compile Bullet Physics to WebAssembly for rust wasm projects.

## Overview

**bullet physics is written in C++** and uses many stdlib features in its codebase.

However, **in order to link C++ libraries when building rust** as a browser web-assembly target, **the C++ libraries must be built without using stdlib**.
for more information, see [this stackoverflow question](https://stackoverflow.com/questions/73604042/compiling-rust-that-calls-c-to-wasm)

bullet-rust-wasm exports rust's web-assembly runtime to c++, replacing the c++ stdlib functions used by bullet physics.

Ultimately, this allows you to use bullet physics in a typical rust web-assembly project using rust and wasm-bindgen.

In bullet physics, these elements have mainly been replaced by rust's runtime:

- allocator (malloc, free, new, delete)
- mutex
- math functions (sin, cos, sqrt, etc)

In conclusion, **bullet physics is now available in the rust web-assembly environment** and can be easily threaded using [rayon-rs](https://github.com/rayon-rs/rayon) to get unmatched physics engine performance in the browser.

## Project Status

- [x] Compile bullet physics to web-assembly without stdlib
- [x] Create a simple bullet physics binding in rust
- [ ] Demonstrate bullet physics in a browser environment
- [ ] Publish the project as a crate (currently, no plans to publish the project as a crate)
