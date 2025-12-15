use std::env;
use std::fs;

fn main() {
    let input = "input.txt";
    let contents = fs::read_to_string(input).expect("Should have been able to read the file");
    let split_string = contents.split("\n");
    let collection: Vec<&str> = split_string.collect();

    let output = advent_of_code_day_one(collection, 0);
    println!("there were {} 0 counts:", output)
}

fn advent_of_code_day_one(collection: Vec<&str>, initial_value: i32) -> i32 {
    let mut zero_count = 0;
    let mut start_number = initial_value;

    for a in collection {
        let rotation = a.chars().next().unwrap().to_string();
        let rotation_value: String = a.chars().skip(1).collect();
        let rotation_number = rotation_value.parse::<i32>().unwrap();

        let left = "L";
        let right = "R";
        
        if rotation == left {
            // Count how many times we cross 0 going left
            let crosses = ((start_number - rotation_number).abs() / 100) as i32;
            if crosses > 0 && start_number > 0 {
                zero_count += crosses;
            }
            
            start_number = ((start_number - rotation_number) % 100 + 100) % 100;
            
            if start_number == 0 {
                zero_count += 1;
            }
        } else if rotation == right {
            // Count how many times we cross 0 going right
            let crosses = (rotation_number / 100) as i32;
            if crosses > 0 && start_number == 0 {
                zero_count += crosses;
            }
            
            start_number = (start_number + rotation_number) % 100;
            
            if start_number == 0 {
                zero_count += 1;
            }
        }
    }

    return zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_values() {
        let input = vec!["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"];
        let output = advent_of_code_day_one(input, 50);
        assert_eq!(output, 3);
    }

    #[test]
    fn test_reddit_case() {
        let input = vec!["L50", "R101"];
        let output = advent_of_code_day_one(input, 50);
        assert_eq!(output, 2);
    }
}