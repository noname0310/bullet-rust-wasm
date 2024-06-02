mod math;
mod allocator;
mod error_handler;
mod atomic;

// use atomic::{bw_mutex_init, bw_mutex_lock, bw_mutex_unlock, bw_get_thread_id, UnsafeManualMutex};
use wasm_bindgen::prelude::*;
use web_sys::console;
// use rayon::prelude::*;

pub use wasm_bindgen_rayon::init_thread_pool;

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

    fn bt_create_dbvtbroadphase() -> *mut core::ffi::c_void;

    fn bt_delete_dbvtbroadphase(ptr: *mut core::ffi::c_void);

    fn bt_create_rigidbody() -> *mut core::ffi::c_void;

    fn bt_delete_rigidbody(ptr: *mut core::ffi::c_void);

    fn bt_link_test();
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

        let rigidbody = bt_create_rigidbody();
        bt_delete_rigidbody(rigidbody);

        for _ in 0..10 {
            rayon::spawn(move || {
                bt_link_test();
            });
        }
    }

    // wasmsync test

    // let mut mutex = wasm_sync::Mutex::new(0);
    // for _ in 0..10 {
    //     let mutex_ptr = &mut mutex as *mut wasm_sync::Mutex<i32> as usize;
    //     rayon::spawn(move || {
    //         let mutex = unsafe { &mut *(mutex_ptr as *mut wasm_sync::Mutex<i32>) };
    //         let mut data = mutex.lock().unwrap();
    //         *data += 1;
    //         console::log_1(&format!("counter: {}", *data).into());
    //     });
    // }

    // mutex test

    // let unsafe_manual_mutex = bw_mutex_init();
    // let counter = Box::new(0);
    // let counter = Box::leak(counter) as *mut i32;

    // for _ in 0..1000 {
    //     let mutex = unsafe_manual_mutex as usize;
    //     let counter = counter as usize;

    //     rayon::spawn(move || {
    //         let mutex = mutex as *mut UnsafeManualMutex;
    //         let counter = counter as *mut i32;

    //         let lock_result = bw_mutex_lock(mutex);
    //         if lock_result != 0 {
    //             console::log_1(&"lock failed".into());
    //         }
    //         unsafe {
    //             *counter += 1;
    //             console::log_1(&format!("counter: {} thread: {}", *counter, bw_get_thread_id()).into());
    //         }
    //         let unlock_result = bw_mutex_unlock(mutex);
    //         if unlock_result != 0 {
    //             console::log_1(&"unlock failed".into());
    //         }
    //     });
    // } 
}
