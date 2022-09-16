// Libraries
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file() -> Vec<i32>{
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut depth_measurements:Vec<i32> = vec![0i32]; // Vector to hold read file data
    for (_index,line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let num = line.parse().unwrap();
        depth_measurements.push(num);
    }
    return depth_measurements;
} // End of read_file (returns a vector)

pub fn count_measurements(){
    let mut increments:i32 = 0;
    let depth_measurements: Vec<i32> = read_file(); // Call function read_file and input that vector to the new vector
    for i in 0..depth_measurements.len()-1{ // Goes out of bounds without the -1
        if depth_measurements[i+1] > depth_measurements[i] {increments+=1;} // Increment if the value is greater
    }
    println!("Total change in measurements: {increments}");
} // End of count_measurements