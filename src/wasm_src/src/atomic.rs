#[no_mangle]
pub extern "C" fn bw_get_thread_id() -> usize {
    thread_id::get()
}

struct ManualMutex {
    mutex: wasm_sync::Mutex<()>,
    guard: Option<&'static mut std::sync::MutexGuard<'static, ()>>,
}

impl ManualMutex {
    fn new() -> Self {
        let mutex = wasm_sync::Mutex::new(());
        let guard = None;

        Self { mutex, guard }
    }

    fn lock(&mut self) -> u8 {
        match self.mutex.lock() {
            Ok(guard) => {
                self.guard = Some(Box::leak(Box::new(guard)));
                0
            }
            Err(_) => 1,
        }
    }

    fn unlock(&mut self) -> u8 {
        match self.guard.take() {
            Some(guard) => {
                drop(guard);
                0
            }
            None => 1,
        }
    }
}

#[no_mangle]
pub extern "C" fn bw_mutex_init() -> *mut ManualMutex {
    let mutex = Box::new(ManualMutex::new());
    let ptr = Box::leak(mutex) as *mut ManualMutex;
    ptr
}

#[no_mangle]
pub extern "C" fn bw_mutex_destroy(mutex: *mut ManualMutex) {
    unsafe {
        Box::from_raw(mutex);
    }
}

#[no_mangle]
pub extern "C" fn bw_mutex_lock(mutex: *mut ManualMutex) -> u8 {
    unsafe { &mut *mutex }.lock()
}

#[no_mangle]
pub extern "C" fn bw_mutex_unlock(mutex: *mut ManualMutex) -> u8 {
    unsafe { &mut *mutex }.unlock()
}

#[no_mangle]
pub extern "C" fn bw_cond_init() -> *mut wasm_sync::Condvar {
    let cond = Box::new(wasm_sync::Condvar::new());
    let ptr = Box::leak(cond) as *mut wasm_sync::Condvar;
    ptr
}

#[no_mangle]
pub extern "C" fn bw_cond_destroy(cond: *mut wasm_sync::Condvar) {
    unsafe {
        Box::from_raw(cond);
    }
}

#[no_mangle]
pub extern "C" fn bw_cond_wait(cond: *mut wasm_sync::Condvar, mutex: *mut ManualMutex) -> u8 {
    let guard = match unsafe { &*mutex }.mutex.lock() {
        Ok(guard) => guard,
        Err(_) => return 1,
    };

    match unsafe { &*cond }.wait(guard) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

#[no_mangle]
pub extern "C" fn bw_cond_broadcast(cond: *mut wasm_sync::Condvar) -> u8 {
    unsafe { &*cond }.notify_all();
    0
}
