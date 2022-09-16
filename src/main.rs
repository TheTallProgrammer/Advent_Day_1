mod functions;

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/data.txt").unwrap();
    let reader = BufReader::new(file);

    for (index,line) in reader.lines().enumerate() {
        let line = line.unwrap();
        print!("{}. {}  ",index + 1, line);
    }
}
