use wasm_bindgen::prelude::*;
use web_sys::console;

#[link(name = "bullet")]
extern "C" {
    // pub fn test_function(i: i32) -> i32;
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
    console_error_panic_hook::set_once();
}
