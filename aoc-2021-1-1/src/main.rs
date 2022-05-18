use std::{fs, str::SplitWhitespace};

fn main() {
    let filename: &str = "src/input.txt";
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let splitwhitespace: SplitWhitespace = contents.split_whitespace();

    let mut vec: Vec<i64> = Vec::<i64>::new();
    for st in splitwhitespace {
        vec.push(st.parse::<i64>().unwrap());
    }

    let mut prev_meas: i64 = std::i64::MAX;
    let mut counter = 0;
    for v in vec {
        if v > prev_meas {
            counter = counter + 1;
        }
        prev_meas = v;
    }

    println!("Result: {}", counter);
}
