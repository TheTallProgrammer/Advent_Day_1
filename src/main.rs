mod functions;

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut depth_measurements = vec![0i32];

    for (index,line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let num = line.parse().unwrap();
        depth_measurements.push(num);
    }

    println!("{}", depth_measurements[2]);


}

