struct Number {
    odd: bool,
    value: i32,
}

impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl std::marker::Copy for Number {}

fn print_number(n: &Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

fn main() {
    let n = Number {
        odd: true,
        value: 51,
    };
    let m = n; // `m` is a copy of `n`
    print_number(&m);
    let o = n; // same. `n` is neither moved nor borrowed.
}
