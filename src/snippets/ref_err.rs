fn main() {
    let mut x = 42;
    println!("x = {}", x);
    let x_ref = &x;
    // x = 13;
    println!("x = {}", x);
    println!("x_ref = {}", x_ref);
    // error: cannot assign to `x` because it is borrowed
}
