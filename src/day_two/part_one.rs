use std::collections::HashMap;

pub fn advent_of_code_day_two_part_one(input: &str) -> i32 {
  let seperated_values: Vec<&str> = input.split(",").collect();
  
  let invalidIds: Vec<&str> = [].to_vec();

  for value in seperated_values {
    let ids: Vec<&str> = value.split("-").collect();
    // println!("ids {:?}", ids);
    let mut map = HashMap::new();
    for id in ids {
      map.insert(id, id.parse::<i32>().unwrap());
    }
    println!("id map {:?}", map);
    match map.get("11") {
      Some(value) => println!("found value {:?}", value),
      None => println!("No Value")
    }

  }

  return 0;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_values() {
      let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
      let output = advent_of_code_day_two_part_one(input);
      println!("OUTPUT: {}", output)
      // assert_eq!(output, 6);
  }
}