pub fn advent_of_code_day_one_part_two(collection: Vec<&str>, initial_value: i32) -> i32 {
  let mut zero_count = 0;
  let mut position = initial_value;

  for instruction in collection {
      let direction = instruction.chars().next().unwrap();
      let amount: i32 = instruction[1..].parse().unwrap();
      
      let new_position = if direction == 'L' {
          position - amount
      } else {
          position + amount
      };

      // Count how many times we point at 0 during this rotation
      // For RIGHT: count multiples of 100 in range (position, position+amount]
      // For LEFT: count multiples of 100 in range [position-amount, position)
      
      if direction == 'R' {
          // Going right: floor((position + amount) / 100)
          zero_count += (position + amount) / 100;
      } else {
          // Going left: count how many times we hit 0
          if position == 0 {
              // Starting at 0 going left: only count if we wrap all the way around
              zero_count += amount / 100;
          } else if amount >= position {
              // We'll cross 0: count = floor((amount - position) / 100) + 1
              zero_count += ((amount - position) / 100) + 1;
          }
          // else: amount < position, we don't reach 0
      }

      // Wrap position to stay in [0, 100) range
      position = new_position.rem_euclid(100);
  }

  zero_count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_values() {
      let input = vec!["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"];
      let output = advent_of_code_day_one_part_two(input, 50);
      assert_eq!(output, 6);
  }

  // #[test]
  // fn test_reddit_case() {
  //     let input = vec!["L50", "R101"];
  //     let output = advent_of_code_day_one_part_two(input, 50);
  //     assert_eq!(output, 2);
  // }
}
