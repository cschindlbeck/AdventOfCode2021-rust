use std::fs;

struct Board {
    state: Vec<Vec<u16>> 
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
        fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
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