#[no_mangle]
pub extern "C" fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}

#[no_mangle]
pub extern "C" fn fabsf(x: f32) -> f32 {
    x.abs()
}

#[no_mangle]
pub extern "C" fn cosf(x: f32) -> f32 {
    x.cos()
}

#[no_mangle]
pub extern "C" fn sinf(x: f32) -> f32 {
    x.sin()
}

#[no_mangle]
pub extern "C" fn tanf(x: f32) -> f32 {
    x.tan()
}

#[no_mangle]
pub extern "C" fn acosf(x: f32) -> f32 {
    x.acos()
}

#[no_mangle]
pub extern "C" fn asinf(x: f32) -> f32 {
    x.asin()
}

#[no_mangle]
pub extern "C" fn atanf(x: f32) -> f32 {
    x.atan()
}

#[no_mangle]
pub extern "C" fn atan2f(y: f32, x: f32) -> f32 {
    y.atan2(x)
}

#[no_mangle]
pub extern "C" fn expf(x: f32) -> f32 {
    x.exp()
}

#[no_mangle]
pub extern "C" fn logf(x: f32) -> f32 {
    x.ln()
}

#[no_mangle]
pub extern "C" fn powf(x: f32, y: f32) -> f32 {
    x.powf(y)
}

#[no_mangle]
pub extern "C" fn fmodf(x: f32, y: f32) -> f32 {
    x % y
}
