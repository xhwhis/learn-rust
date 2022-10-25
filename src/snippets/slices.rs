fn tail(s: &[u8]) -> &[u8] {
    &s[1..]
}

fn main() {
    let x = &[1, 2, 3, 4, 5];
    let y = tail(x);
    println!("y = {:?}", y);
}
