use parking_lot::lock_api::RawMutex;

#[no_mangle]
pub extern "C" fn bw_get_thread_id() -> usize {
    thread_id::get()
}

#[no_mangle]
pub extern "C" fn bw_mutex_init() -> u8 {
    let mutex = parking_lot::Mutex::new(());
    unsafe {
        std::mem::transmute(mutex)
    }
}

#[no_mangle]
pub extern "C" fn bw_mutex_lock(mutex: *mut parking_lot::Mutex<()>) {
    unsafe {
        (&mut *mutex).raw()
    }.lock();
    web_sys::console::log_1(&"bw_mutex_lock".into());
}

#[no_mangle]
pub extern "C" fn bw_mutex_unlock(mutex: *mut parking_lot::Mutex<()>) {
    unsafe {
        (&mut *mutex).force_unlock();
    }
    web_sys::console::log_1(&"bw_mutex_unlock".into());
}

#[no_mangle]
pub extern "C" fn bw_cond_init() -> u32 {
    let cond = parking_lot::Condvar::new();
    unsafe {
        std::mem::transmute(cond)
    }
}

#[no_mangle]
pub extern "C" fn bw_cond_wait(cond: *mut parking_lot::Condvar, mutex: *mut parking_lot::Mutex<()>) -> u8 {
    let mut guard = unsafe { &*mutex }.lock();

    unsafe { &*cond }.wait(&mut guard);
    0
}

#[no_mangle]
pub extern "C" fn bw_cond_broadcast(cond: *mut parking_lot::Condvar) -> u8 {
    unsafe { &*cond }.notify_all();
    0
}
