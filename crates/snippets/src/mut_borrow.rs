fn main() {
    let mut x = 42;
    let x_ref1 = &mut x;
    // let x_ref2 = &mut x;
    println!("x_ref1 = {}", x_ref1);
    // error: cannot borrow `x` as mutable because it is also borrowed as immutable
    // println!("x_ref2 = {}", x_ref2);
}
