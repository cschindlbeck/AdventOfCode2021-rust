use std::fs::File;
use std::io::{Read, Error as IoError};
use std::path::Path;

pub fn read_a_file() -> Vec<u8> {
    let mut file = File::open("input.txt");

    //let mut data = Vec::new();
    //file.read_to_end(&mut data);
    //println!("{}", data);

    let mut filename: &str = "bla";
    let mut f = File::open(&Path::new(filename));
    let mut v: Vec<u8> = Vec::new();
    let file_content = f.read_to_end(&mut v);
    println!("{:?}", file_content);

    //return file_content;
    return v;
}