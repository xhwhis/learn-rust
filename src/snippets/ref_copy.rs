#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn negate(p: Point) -> Point {
    Point { x: -p.x, y: -p.y }
}

fn main() {
    let p = Point { x: 1.0, y: 3.0 };
    let p_ref = &p;
    negate(*p_ref); // ...and now this works
}
