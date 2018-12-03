use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Result};

fn main() {
    let args: Vec<String> = env::args().collect();
    let _program_name = &args[0];
    let filename = &args[1];

    let mut file = File::open(filename).expect("file not found");
    let mut file_contents = String::new();
    
    let mut num_v = Vec::new();

    file.read_to_string(&mut file_contents)
        .expect("something is wrong with that file");
    
    let file_len = &mut file_contents.len();
    
    for line in BufReader::new(file).lines()   {
        num_v.push(line);
        println!("{}", line?);
    }
}

