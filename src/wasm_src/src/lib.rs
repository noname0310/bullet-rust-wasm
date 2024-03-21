mod math;

use wasm_bindgen::prelude::*;
use web_sys::console;

#[link(name = "bullet")]
extern "C" {
    pub fn __wasm_call_ctors();

    pub fn bt_get_version() -> i32;

    pub fn bt_sin(x: f32) -> f32;
}

// #[no_mangle]
// unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
//     let mut buf = Vec::with_capacity(size);
//     let ptr = buf.as_mut_ptr();
//     std::mem::forget(buf);
//     ptr
// }

// #[no_mangle]
// unsafe extern "C" fn dealloc(ptr: *mut u8, size: usize) {
//     let _buf = Vec::from_raw_parts(ptr, 0, size);
// }

#[wasm_bindgen(js_name = init)]
pub fn init() {
    unsafe {
        __wasm_call_ctors();
    }

    console_error_panic_hook::set_once();

    unsafe {
        let version = bt_get_version();
        console::log_1(&format!("Bullet version: {}", version).into());

        let sin = bt_sin(1.0);
        console::log_1(&format!("sin(1.0): {}", sin).into());
    }
}