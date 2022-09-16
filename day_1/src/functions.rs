// Libraries
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(){

    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut depth_measurements = vec![0i32];

    for (index,line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let num = line.parse().unwrap();
        depth_measurements.push(num);
    }

    println!("{}", depth_measurements[2]);
}

// Find a way to return the vector so when it's called in a calculating function, it can be used in there dynamically 