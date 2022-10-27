fn foobar<F>(mut f: F)
where
    F: FnMut(i32) -> i32,
{
    let tmp = f(2);
    println!("{}", f(tmp));
}

fn main() {
    let mut acc = 2;
    foobar(|x| {
        acc += 1;
        x * acc
    });
    println!("acc = {}", acc);
}
