struct Pair<T1, T2> {
    a: T1,
    b: T2,
}

fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    use std::any::type_name;
    println!("{}", type_name::<i32>()); // prints "i32"
    println!("{}", type_name::<(f64, char)>()); // prints "(f64, char)"
    let p1 = Pair { a: 3, b: 9 };
    let p2 = Pair { a: true, b: 9 };
    print_type_name(&p1); // prints "Pair<i32>"
    print_type_name(&p2); // prints "Pair<bool>"
    let mut v1 = Vec::new();
    v1.push(1);
    let mut v2 = Vec::new();
    v2.push(false);
    print_type_name(&v1); // prints "Vec<i32>"
    print_type_name(&v2); // prints "Vec<bool>"
}
