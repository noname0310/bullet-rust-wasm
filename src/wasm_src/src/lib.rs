use wasm_bindgen::prelude::*;
use web_sys::console;

#[link(name = "foolib")]
extern "C" {
    pub fn set_allocator(
        alloc: unsafe extern "C" fn(usize) -> *mut u8,
        dealloc: unsafe extern "C" fn(*mut u8, usize)
    );
    
    pub fn test_function(i: i32) -> i32;

    pub fn allocation_test(size: i32) -> i32;
}

unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

unsafe extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    let _buf = Vec::from_raw_parts(ptr, 0, size);
}

#[wasm_bindgen(js_name = init)]
pub fn init() {
    console_error_panic_hook::set_once();
    unsafe {
        set_allocator(alloc, dealloc);
    }
}

#[wasm_bindgen(js_name = testFunction)]
pub fn test_function_js(i : i32) -> i32{
    let res = unsafe{ test_function(i) };
    res
}

#[wasm_bindgen(js_name = allocationTest)]
pub fn allocation_test_js(size : i32) -> i32{
    let res = unsafe{ allocation_test(size) };
    res
}
