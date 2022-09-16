// Libraries
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file() -> Vec<i32>{

    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut depth_measurements:Vec<i32> = vec![0i32];

    for (index,line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let num = line.parse().unwrap();
        depth_measurements.push(num);
    }
    return depth_measurements;
}

pub fn calc_measurements(){
    let mut increments:i32 = 0;
    let mut depth_measurements: Vec<i32> = read_file();
    for i in 0..depth_measurements.len()-1{
        if(depth_measurements[i] > depth_measurements[i+1]){
            increments+=1;
        }
    }
    println!("Total change in measurements: {increments}");
}

// Find a way to return the vector so when it's called in a calculating function, it can be used in there dynamically 