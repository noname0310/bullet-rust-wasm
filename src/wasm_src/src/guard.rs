use web_sys::console;

#[no_mangle]
extern "C" fn __cxa_guard_acquire(_guard: *mut u8) -> i32 {
    console::log_1(&format!("__cxa_guard_acquire{}", _guard as usize).into());
    1
}

#[no_mangle]
extern "C" fn __cxa_guard_release(_guard: *mut u8) {
    console::log_1(&format!("__cxa_guard_release{}", _guard as usize).into());
}

#[no_mangle]
extern "C" fn __cxa_guard_abort(_guard: *mut u8) {
    console::log_1(&format!("__cxa_guard_abort{}", _guard as usize).into());
}
