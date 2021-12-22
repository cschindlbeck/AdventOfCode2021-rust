use std::{fs, str::SplitWhitespace};

fn main() {

    let filename: &str = "src/input.txt";
    let contents: String =
        fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    let splitwhitespace: SplitWhitespace = contents.split_whitespace();
    
    let mut vec: Vec<i64> = Vec::<i64>::new();
    for st in splitwhitespace {
        vec.push(st.parse::<i64>().unwrap());
    }

    let mut prev_meas: i64 = std::i64::MAX;
    let mut counter = 0;
    let ind: usize = vec.len();
    for indx in 0..ind-2{
        let sliding_window: i64 = vec[indx] + vec[indx+1] + vec[indx+2];
        if  sliding_window>prev_meas{
            counter = counter + 1;
        }
        prev_meas = sliding_window;

        // println!("With text{}  {}  {}", indx, vec[indx], sliding_window);
    }
        
    println!("Result: {}", counter);


}