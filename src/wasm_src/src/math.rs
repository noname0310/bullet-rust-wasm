#[no_mangle]
unsafe extern "C" fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}

#[no_mangle]
unsafe extern "C" fn fabsf(x: f32) -> f32 {
    x.abs()
}

#[no_mangle]
unsafe extern "C" fn cosf(x: f32) -> f32 {
    x.cos()
}

#[no_mangle]
unsafe extern "C" fn sinf(x: f32) -> f32 {
    x.sin()
}

#[no_mangle]
unsafe extern "C" fn tanf(x: f32) -> f32 {
    x.tan()
}

#[no_mangle]
unsafe extern "C" fn acosf(x: f32) -> f32 {
    x.acos()
}

#[no_mangle]
unsafe extern "C" fn asinf(x: f32) -> f32 {
    x.asin()
}

#[no_mangle]
unsafe extern "C" fn atanf(x: f32) -> f32 {
    x.atan()
}

#[no_mangle]
unsafe extern "C" fn atan2f(y: f32, x: f32) -> f32 {
    y.atan2(x)
}

#[no_mangle]
unsafe extern "C" fn expf(x: f32) -> f32 {
    x.exp()
}

#[no_mangle]
unsafe extern "C" fn logf(x: f32) -> f32 {
    x.ln()
}

#[no_mangle]
unsafe extern "C" fn powf(x: f32, y: f32) -> f32 {
    x.powf(y)
}

#[no_mangle]
unsafe extern "C" fn fmodf(x: f32, y: f32) -> f32 {
    x % y
}
