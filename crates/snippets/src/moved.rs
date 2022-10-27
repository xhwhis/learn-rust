struct Number {
    odd: bool,
    value: i32,
}

fn print_number(n: &Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

fn main() {
    let n = Number {
        odd: true,
        value: 51,
    };
    print_number(&n); // `n` is moved
    print_number(&n); // error: use of moved value: `n`
}
