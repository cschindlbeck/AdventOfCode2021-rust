use std::{fs, str::SplitWhitespace};

fn main() {

    let filename: &str = "src/input.txt";
    let contents: String =
        fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    let splitwhitespace: SplitWhitespace = contents.split_whitespace();
    
    let mut vec: Vec<i64> = Vec::<i64>::new();
    for v in splitwhitespace {
        vec.push(v.parse::<i64>().unwrap());
    }

    println!("With text:\n{}", vec[0]);
}
