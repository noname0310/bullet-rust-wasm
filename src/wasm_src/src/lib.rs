mod bind;
mod cpp_runtime;

use bind::physics_world::PhysicsWorld;
use glam::Vec3;
use rayon::iter::IntoParallelIterator;
use wasm_bindgen::prelude::*;
use rayon::prelude::*;

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
    web_sys::console::log_1(&"start physics engine threading test".into());
    (0..1000).into_par_iter().for_each(|_| {
        let world = PhysicsWorld::new();
        world.set_gravity(Vec3::new(0.0, -9.8 * 10.0, 0.0));

        for _ in 0..1000 {
            world.step_simulation(1.0 / 60.0, 120, 1.0 / 120.0);
        }
    });
    web_sys::console::log_1(&"end physics engine threading test".into());
}
