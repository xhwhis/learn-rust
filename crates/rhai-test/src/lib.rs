use libc::{c_char, c_int};
use rhai::Engine;
use std::ffi::CStr;

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

fn func(a: i64, b: i64) -> i64 {
    unsafe { cb.unwrap()(a as c_int, b as c_int) as i64 }
}

#[no_mangle]
pub extern "C" fn trigger_callback(str: *const c_char) -> c_int {
    let c_str = unsafe {
        assert!(!str.is_null());
        CStr::from_ptr(str)
    };
    let mut engine = Engine::new();
    engine.register_fn("func", func);
    let result = engine.eval::<i64>(c_str.to_str().unwrap()).unwrap();
    result as c_int
}
