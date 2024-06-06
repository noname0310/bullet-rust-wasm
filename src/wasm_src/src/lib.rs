mod bind;
mod cpp_runtime;

use bind::physics_world::PhysicsWorld;
use wasm_bindgen::prelude::*;
// use rayon::prelude::*;

pub use wasm_bindgen_rayon::init_thread_pool;

#[link(name = "bullet")]
extern "C" {
    fn __wasm_call_ctors();
}

#[wasm_bindgen(js_name = init)]
pub fn init() {
    unsafe {
        __wasm_call_ctors();
    }

    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name = test)]
pub fn test() {
    for _ in 0..10000000 {
        rayon::spawn(move || {
            let world = PhysicsWorld::new();
        });
    } 
}
