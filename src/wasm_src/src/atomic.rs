use parking_lot::lock_api::RawMutex;

#[no_mangle]
pub extern "C" fn bw_get_thread_id() -> usize {
    thread_id::get()
}

#[no_mangle]
pub extern "C" fn bw_mutex_init() -> *mut parking_lot::Mutex<()> {
    let mutex = Box::new(parking_lot::Mutex::new(()));
    let ptr = Box::leak(mutex) as *mut parking_lot::Mutex<()>;
    ptr
}

#[no_mangle]
pub extern "C" fn bw_mutex_destroy(mutex: *mut parking_lot::Mutex<()>) {
    unsafe {
        Box::from_raw(mutex);
    }
}

#[no_mangle]
pub extern "C" fn bw_mutex_lock(mutex: *mut parking_lot::Mutex<()>) {
    unsafe {
        (&mut *mutex).raw()
    }.lock();
}

#[no_mangle]
pub extern "C" fn bw_mutex_unlock(mutex: *mut parking_lot::Mutex<()>) {
    unsafe {
        (&mut *mutex).force_unlock();
    }
}

#[no_mangle]
pub extern "C" fn bw_cond_init() -> *mut parking_lot::Condvar {
    let cond = Box::new(parking_lot::Condvar::new());
    let ptr = Box::leak(cond) as *mut parking_lot::Condvar;
    ptr
}

#[no_mangle]
pub extern "C" fn bw_cond_destroy(cond: *mut parking_lot::Condvar) {
    unsafe {
        Box::from_raw(cond);
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
