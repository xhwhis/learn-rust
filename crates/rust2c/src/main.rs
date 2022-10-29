#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let mut ab = AB { a: 1, b: 2 };
    println!("a = {}, b = {}", ab.a, ab.b);
    unsafe {
        assign(&mut ab, 3, 4);
    }
    println!("a = {}, b = {}", ab.a, ab.b);
}
