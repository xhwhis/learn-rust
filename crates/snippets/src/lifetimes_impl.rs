struct NumRef<'a> {
    x: &'a i32,
}

impl<'a> NumRef<'a> {
    fn as_i32_ref(&'a self) -> &'a i32 {
        self.x
    }
}

fn main() {
    let x: i32 = 99;
    let x_num_ref = NumRef { x: &x };
    let x_i32_ref = x_num_ref.as_i32_ref();
    // neither ref can outlive `x`
    println!("x_i32_ref = {}", x_i32_ref);
}
