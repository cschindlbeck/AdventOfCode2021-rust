use std::fs;

fn main() {
    let mut depth: i64 = 0;
    let mut position: i64 = 0;

    let filename: &str = "src/input.txt";
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.lines();
    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();
        let value: i64 = split[1].parse::<i64>().unwrap();
        match split[0].as_ref() {
            "forward" => position = position + value,
            "down" => depth = depth + value,
            "up" => depth = depth - value,
            _ => println!("else"),
        }
    }

    println!("Position {}", position);
    println!("Depth {}", depth);

    println!("Result {}", depth * position);
}
