struct NumRef<'a> {
    x: &'a i32,
}

fn as_num_ref<'a>(x: &'a i32) -> NumRef<'a> {
    NumRef { x: &x }
}

fn main() {
    let x: i32 = 99;
    let x_ref = as_num_ref(&x);
    println!("x = {}", x);
    println!("x_ref = {}", x_ref.x);
    // `x_ref` cannot outlive `x`, etc.
}
