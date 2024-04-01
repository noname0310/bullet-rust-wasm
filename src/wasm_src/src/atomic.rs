#[no_mangle]
pub extern "C" fn bw_get_thread_id() -> usize {
    thread_id::get()
}

#[no_mangle]
pub extern "C" fn bw_mutex_init() -> *mut wasm_sync::Mutex<()> {
    let mutex = Box::new(wasm_sync::Mutex::new(()));
    let ptr = Box::into_raw(mutex);
    std::mem::forget(mutex);
    ptr
}

#[no_mangle]
pub extern "C" fn bw_mutex_destroy(mutex: *mut wasm_sync::Mutex<()>) {
    unsafe {
        Box::from_raw(mutex);
    }
}

#[no_mangle]
pub extern "C" fn bw_mutex_lock(mutex: *mut wasm_sync::Mutex<()>) {
    unsafe { *mutex }.lock().unwrap();
}

#[no_mangle]
pub extern "C" fn bw_mutex_unlock(mutex: *mut wasm_sync::Mutex<()>) {
    unsafe { *mutex }.unlock().unwrap();
}

#[no_mangle]
pub extern "C" fn bw_cond_init() -> *mut wasm_sync::Condvar {
    let cond = Box::new(wasm_sync::Condvar::new());
    let ptr = Box::into_raw(cond);
    std::mem::forget(cond);
    ptr
}

#[no_mangle]
pub extern "C" fn bw_cond_destroy(cond: *mut wasm_sync::Condvar) {
    unsafe {
        Box::from_raw(cond);
    }
}

#[no_mangle]
pub extern "C" fn bw_cond_wait(cond: *mut wasm_sync::Condvar, mutex: *mut wasm_sync::Mutex<()>) {
    unsafe {
        let mut guard = mutex.lock().unwrap();
        cond.wait(&mut guard).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn bw_cond_broadcast(cond: *mut wasm_sync::Condvar) {
    unsafe { *cond }.notify_all();
}
