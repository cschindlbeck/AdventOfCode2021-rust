use std::char;
use std::fs;

fn filter_input_most<'a>(oxygen: &'a mut Vec<&str>) -> Vec<&'a str> {
    let length: usize = oxygen[0].len();
    // println!("len {}", length);
    for j in 0..length {
        let mut count_vec: Vec<u32> = vec![0; length];
        let oxygen2: Vec<&str> = oxygen.clone();
        // println!("{:?}", oxygen2);
        for line in &oxygen2 {
            for (pos, l) in line.chars().enumerate() {
                count_vec[pos] += l.to_digit(10).unwrap();
            }
        }
        let half: f32 = oxygen2.len() as f32 / 2.0;
        // println!("{:?}", half);
        // println!("{:?}", count_vec[j]);
        let most_common: Vec<char> = count_vec
            .iter()
            .map(|&x| {
                if (x as f32) >= half {
                    char::from_digit(1, 10).unwrap()
                } else {
                    char::from_digit(0, 10).unwrap()
                }
            })
            .collect();
        println!("Most common {:?}", most_common[j]);
        for i in (0..oxygen.len()).rev() {
            if most_common[j] != oxygen[i].chars().nth(j).unwrap() {
                oxygen.remove(i);
                if oxygen.len() == 1 {
                    return oxygen.to_vec();
                }
            }
        }
        println!("Oxy  {:?}", oxygen);
    }
    println!("O no{:?}", oxygen);
    panic!("removed to nothingness")
}

fn filter_input_least<'a>(oxygen: &'a mut Vec<&str>) -> Vec<&'a str> {
    let length: usize = oxygen[0].len();
    // println!("len {}", length);
    for j in 0..length {
        let mut count_vec: Vec<u32> = vec![0; length];
        let oxygen2: Vec<&str> = oxygen.clone();
        // println!("{:?}", oxygen2);
        for line in &oxygen2 {
            for (pos, l) in line.chars().enumerate() {
                count_vec[pos] += l.to_digit(10).unwrap();
            }
        }
        let half: f32 = oxygen2.len() as f32 / 2.0;
        // println!("{:?}", half);
        // println!("{:?}", count_vec[j]);
        let most_common: Vec<char> = count_vec
            .iter()
            .map(|&x| {
                if (x as f32) < half {
                    char::from_digit(1, 10).unwrap()
                } else {
                    char::from_digit(0, 10).unwrap()
                }
            })
            .collect();
        println!("Most common {:?}", most_common[j]);
        for i in (0..oxygen.len()).rev() {
            if most_common[j] != oxygen[i].chars().nth(j).unwrap() {
                oxygen.remove(i);
                if oxygen.len() == 1 {
                    return oxygen.to_vec();
                }
            }
        }
        println!("Oxy  {:?}", oxygen);
    }
    println!("O no{:?}", oxygen);
    panic!("removed to nothingness")
}

fn main() {
    let filename: &str = "src/input.txt";
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    // Oxygen rating
    let mut oxygen = lines.to_vec();
    filter_input_most(&mut oxygen);
    println!("Solution {:?}", oxygen);
    let oxygen_rating = isize::from_str_radix(&oxygen.join(""), 2).unwrap();
    println!("Oxygen rating {}", oxygen_rating);

    // CO2 scrubber rating
    let mut scrubber = lines.to_vec();
    filter_input_least(&mut scrubber);
    println!("Solution {:?}", scrubber);
    let scrubber_rating = isize::from_str_radix(&scrubber.join(""), 2).unwrap();
    println!("Scrubber rating {}", scrubber_rating);

    println!("Result {}", oxygen_rating * scrubber_rating);
}
