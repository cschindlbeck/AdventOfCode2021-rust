use std::fs;

fn main() {
    let filename: &str = "src/input.txt";
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let length: usize = lines[0].len();
    let mut count_vec: Vec<u32> = vec![0; length];

    for line in &lines {
        for (pos, l) in line.chars().enumerate() {
            count_vec[pos] += l.to_digit(10).unwrap();
        }
    }

    let half: f32 = lines.len() as f32 / 2.0;
    let gamma_str: String = count_vec
        .iter()
        .map(|&x| {
            if (x as f32) > half {
                1.to_string()
            } else {
                0.to_string()
            }
        })
        .collect::<String>();
    let gamma = isize::from_str_radix(&gamma_str, 2).unwrap();

    let epsilon_str: String = count_vec
        .iter()
        .map(|&x| {
            if (x as f32) < half {
                1.to_string()
            } else {
                0.to_string()
            }
        })
        .collect::<String>();
    let epsilon = isize::from_str_radix(&epsilon_str, 2).unwrap();

    println!("{:?}", gamma);
    println!("{:?}", epsilon);

    println!("Result {}", gamma * epsilon);
}
