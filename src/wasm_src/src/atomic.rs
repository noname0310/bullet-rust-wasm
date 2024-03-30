#[no_mangle]
pub extern "C" fn bw_get_thread_id() -> usize {
    thread_id::get()
}
