trait Signed {
    fn is_strictly_negative(self) -> bool;
}

impl Signed for i32 {
    fn is_strictly_negative(self) -> bool {
        self < 0
    }
}

fn main() {
    println!("{}", (-1).is_strictly_negative());
}
