use std::ops::{Add, Mul};
fn magic<T>(first: T, second: T) -> (T, T)
    where T: Mul<Output = T> + Add<Output = T> + Copy
{
    (first + second, first * second)
}

fn main() {
    let a = 4;
    let b = 3;
    println!("{} +/* {}: {:?}", a, b, magic(a, b));
}
