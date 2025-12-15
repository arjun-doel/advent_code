use std::env;
use std::fs;

fn main() {
    let input = "input.txt";
    let contents = fs::read_to_string(input).expect("Should have been able to read the file");
    let split_string = contents.split("\n");
    let collection: Vec<&str> = split_string.collect();
    
    
    let mut zero_count = 0;
    let mut start_number = 50;

    for a in collection {
        let rotation = a.chars().next().unwrap().to_string();
        let rotation_value: String = a.chars().skip(1).collect();
        let rotation_number = rotation_value.parse::<i32>().unwrap();

        let left = "L";
        let right = "R";
        
        if rotation == left {
            start_number =- rotation_number
        } else if rotation == right {
            start_number += rotation_number
        }

        println!("start number {}", start_number);
        if start_number == 0 {
            zero_count += 1
        }
    }

    println!("zero count {}", zero_count)
}
