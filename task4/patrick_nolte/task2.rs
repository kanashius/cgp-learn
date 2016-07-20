struct Swagger<T: std::fmt::Display> {
    value: T,
}

impl<T: std::fmt::Display> std::fmt::Display for Swagger<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#swag {} #yolo", self.value)
    }
}

fn main() {
    let v = Swagger { value: "Its Me!" };
    println!("{}", v);
}
