use std::fmt::Display;
use std::fmt;

struct Swagger<T: Display> {
    value: T,
}

impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#swag {} #yolo", self.value)
    }
}

fn main() {
    let v = Swagger { value: "Its Me!" };
    println!("{}", v);
}
