use libc::c_int;

#[allow(non_camel_case_types)]
type c_callback = unsafe extern "C" fn(c_int, c_int) -> c_int;

#[allow(non_upper_case_globals)]
static mut cb: Option<c_callback> = None;

#[no_mangle]
pub extern "C" fn register_callback(callback: Option<c_callback>) {
    unsafe {
        cb = callback;
    }
}

#[no_mangle]
pub extern "C" fn trigger_callback() -> c_int {
    unsafe {
        return cb.unwrap()(1, 2);
    }
}
