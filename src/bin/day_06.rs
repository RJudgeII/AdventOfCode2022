use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  let input: String = read_file();

  let output1 = get_difference_length(input.clone(), 4);
  let output2 = get_difference_length(input, 14);

  println!("First packet is at character {}", output1);
  println!("First message is at character {}", output2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn read_file() -> String {
  let mut output: String = String::new();

  if let Ok(lines) = read_lines("input_files/input_day_06.txt") {
    for line in lines {
      if let Ok(input) = line {
        output += &input;
      }
    }
  }
  return output;
}

fn get_difference_length(input: String, length: usize) -> usize {
  let mut output = 1;
  let len = input.len();

  for i in (length - 1)..len {
    let mut flag = false;

    for j in (0..length - 1).rev() {
      for k in (j + 1..length).rev() {
        if input.chars().nth(i - j).unwrap().to_string()
          == input.chars().nth(i - k).unwrap().to_string()
        {
          flag = true;
        }
      }
    }

    if !flag {
      output += i;
      break;
    }
  }

  return output;
}
