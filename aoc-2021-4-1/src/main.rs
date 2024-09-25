use std::fs;
<<<<<<< HEAD

struct Board {
    state: Vec<Vec<u16>>,
}

impl Board {
    // fill board with values
    fn read_to_board(&mut self, num: u16) {
        self.state[0][0] = num;
    }

    // check if row is equal
    fn is_row_equal(&mut self, num: u16) {
        self.state[0][0] = num;
    }

    // check if column is equal
    fn is_col_equal(&mut self, num: u16) {
        self.state[0][0] = num;
    }
}

fn main() {
    let filename: &str = "src/test.txt";
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n\n").collect();

    // println!("Result {:?}", lines[0]);
    // println!("Result {:?}", lines[1]);
    // println!("Result {:?}", lines[2]);
    // println!("Result {:?}", lines[3]);
    // println!("Result {:?}", lines[4]);

    let sublines: Vec<&str> = lines[1].split("\n").collect();
    println!("Result {:?}", sublines);

    // let mut board1 = Board {state: vec![vec![0u16; 5]; 5]};
    // println!("{:?}", board1.state);
    // board1.read_to_board(5u16);
    // println!("{:?}", board1.state);
    // // board1.state[0][0] = 1;
    // //

    // draw numbers
    // for l in lines[0].split(","){
    //     println!("{:?}", l.parse::<u16>().unwrap());
    // }
}
=======
use std::char;

fn filter_input_most<'a>(oxygen: &'a mut Vec<&str>) -> Vec<&'a str>{
    let length: usize = oxygen[0].len();
    // println!("len {}", length);
    for j in 0..length{
        let mut count_vec: Vec<u32> = vec![0; length];
        let oxygen2: Vec<&str> = oxygen.clone();
        // println!("{:?}", oxygen2);
        for line in &oxygen2{
            for (pos, l) in line.chars().enumerate(){
                count_vec[pos] += l.to_digit(10).unwrap();
            }
        }
        let half: f32 = oxygen2.len() as f32 / 2.0;
        // println!("{:?}", half);
        // println!("{:?}", count_vec[j]);
        let most_common: Vec<char> = count_vec.iter()
                                            .map(|&x| { if (x as f32) >= half {char::from_digit(1,10).unwrap()} else {char::from_digit(0,10).unwrap()} })
                                            .collect();
        println!("Most common {:?}", most_common[j]);
        for i in (0..oxygen.len()).rev(){
            if most_common[j] != oxygen[i].chars().nth(j).unwrap(){
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

fn filter_input_least<'a>(oxygen: &'a mut Vec<&str>) -> Vec<&'a str>{
    let length: usize = oxygen[0].len();
    // println!("len {}", length);
    for j in 0..length{
        let mut count_vec: Vec<u32> = vec![0; length];
        let oxygen2: Vec<&str> = oxygen.clone();
        // println!("{:?}", oxygen2);
        for line in &oxygen2{
            for (pos, l) in line.chars().enumerate(){
                count_vec[pos] += l.to_digit(10).unwrap();
            }
        }
        let half: f32 = oxygen2.len() as f32 / 2.0;
        // println!("{:?}", half);
        // println!("{:?}", count_vec[j]);
        let most_common: Vec<char> = count_vec.iter()
                                            .map(|&x| { if (x as f32) < half {char::from_digit(1,10).unwrap()} else {char::from_digit(0,10).unwrap()} })
                                            .collect();
        println!("Most common {:?}", most_common[j]);
        for i in (0..oxygen.len()).rev(){
            if most_common[j] != oxygen[i].chars().nth(j).unwrap(){
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
        fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
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

    println!("Result {}", oxygen_rating*scrubber_rating);
    

}
>>>>>>> 013fb03 (Local changes)
