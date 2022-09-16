// Libraries
use std::env;
use std::fs;

pub fn read_file(){

    let contents = fs::read_to_string("src/input.txt")
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}