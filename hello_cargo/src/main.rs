//mod read_input;
use std::fs;

fn main() {
    println!("Hello, world!");
    //read_input::read_input();
    //read_input::read_a_file();

    let filename: &str = "src/input.txt";

    // --snip--
    println!("In file {}", filename);

    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let bla = contents.split_whitespace();
    let mut vec = Vec::<i64>::new();
    
    for b in bla {
        vec.push(b.parse::<i64>().unwrap());
    }

    println!("With text:\n{}", vec[0]);
}
