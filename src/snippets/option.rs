enum MyOption<T> {
    None,
    Some(T),
}

impl<T> MyOption<T> {
    fn unwrap(self) -> T {
        // enums variants can be used in patterns:
        match self {
            Self::Some(t) => t,
            Self::None => panic!(".unwrap() called on a None option"),
        }
    }
}

use self::MyOption::{None, Some};

fn main() {
    let o1: MyOption<i32> = Some(128);
    o1.unwrap(); // this is fine

    let o2: MyOption<i32> = None;
    o2.unwrap(); // this panics!
}
