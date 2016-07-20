fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        println!("Error Params - Count: {}", args.len());
    } else {
        let first = match args[1].parse::<i32>() {
            Ok(n) => n,
            Err(n) => 0,
        };
        let second = match args[3].parse::<i32>() {
            Ok(n) => n,
            Err(n) => 0,
        };
        let result = match args[2].as_ref() {
            "+" => first + second,
            "-" => first - second,
            _ => 0,
        };
        println!("Result: {} {} {} = {}", first, args[2], second, result);
    }
}
