fn for_each_planet<F>(f: F)
where
    F: Fn(&'static str),
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}

fn main() {
    let greeting = String::from("Good to see you");
    for_each_planet(|planet| println!("{}, {}", greeting, planet));
    // our closure borrows `greeting`, so it cannot outlive it
}
