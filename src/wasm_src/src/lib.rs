mod math;
mod allocator;

use wasm_bindgen::prelude::*;
use web_sys::console;

#[link(name = "bullet")]
extern "C" {
    fn __wasm_call_ctors();

    fn bt_get_version() -> i32;

    fn bt_sin(x: f32) -> f32;

    fn bt_alloc_int() -> *mut i32;

    fn bt_free_int(ptr: *mut i32);

    fn bt_nonallocnew_test() -> *mut i32;

    fn bt_transform_test();

    fn bt_vector3_test() -> i32;

    fn bt_create_dbvtbroadphase() -> *mut i32;

    fn bt_delete_dbvtbroadphase(ptr: *mut i32);
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

        let boxed_int = bt_alloc_int();
        console::log_1(&format!("boxed_int: {:?}", *boxed_int).into());
        bt_free_int(boxed_int);

        let boxed_int = bt_nonallocnew_test();
        console::log_1(&format!("boxed_int: {:?}", *boxed_int).into());
        bt_free_int(boxed_int);

        bt_transform_test();

        let vector3_test = bt_vector3_test();
        console::log_1(&format!("vector3_test: {}", vector3_test).into());

        let dbvtbroadphase = bt_create_dbvtbroadphase();
        bt_delete_dbvtbroadphase(dbvtbroadphase);
    }
}
