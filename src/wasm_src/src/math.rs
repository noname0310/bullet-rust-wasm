#[no_mangle]
unsafe extern "C" fn bw_sqrtf(x: f32) -> f32 {
    x.sqrt()
}

#[no_mangle]
unsafe extern "C" fn bw_fabsf(x: f32) -> f32 {
    x.abs()
}

#[no_mangle]
unsafe extern "C" fn bw_cosf(x: f32) -> f32 {
    x.cos()
}

#[no_mangle]
unsafe extern "C" fn bw_sinf(x: f32) -> f32 {
    x.sin()
}

#[no_mangle]
unsafe extern "C" fn bw_tanf(x: f32) -> f32 {
    x.tan()
}

#[no_mangle]
unsafe extern "C" fn bw_acosf(x: f32) -> f32 {
    x.acos()
}

#[no_mangle]
unsafe extern "C" fn bw_asinf(x: f32) -> f32 {
    x.asin()
}

#[no_mangle]
unsafe extern "C" fn bw_atanf(x: f32) -> f32 {
    x.atan()
}

#[no_mangle]
unsafe extern "C" fn bw_atan2f(y: f32, x: f32) -> f32 {
    y.atan2(x)
}

#[no_mangle]
unsafe extern "C" fn bw_expf(x: f32) -> f32 {
    x.exp()
}

#[no_mangle]
unsafe extern "C" fn bw_logf(x: f32) -> f32 {
    x.ln()
}

#[no_mangle]
unsafe extern "C" fn bw_powf(x: f32, y: f32) -> f32 {
    x.powf(y)
}

#[no_mangle]
unsafe extern "C" fn bw_fmodf(x: f32, y: f32) -> f32 {
    x % y
}
