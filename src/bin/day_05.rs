use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  let (crates, instructions) = read_file();

  let mut manifest1: Vec<VecDeque<String>> = create_manifest(crates);
  let mut manifest2: Vec<VecDeque<String>> = manifest1.clone();
  let instruction_list: Vec<Vec<usize>> = create_instruction_list(instructions);

  for instruction in instruction_list.clone() {
    for _i in 0..instruction[0] {
      let moved = manifest1[instruction[1] - 1].pop_front().unwrap();
      manifest1[instruction[2] - 1].push_front(moved);
    }
  }

  for instruction in instruction_list {
    let mut crane: VecDeque<String> = VecDeque::new();
    for _i in 0..instruction[0] {
      crane.push_back(manifest2[instruction[1] - 1].pop_front().unwrap());
    }
    for _i in 0..crane.len() {
      manifest2[instruction[2] - 1].push_front(crane.pop_back().unwrap());
    }
  }

  let mut output1: String = "".to_string();
  for mut stack in manifest1 {
    output1 += &stack.pop_front().unwrap();
  }

  let mut output2: String = "".to_string();
  for mut stack in manifest2 {
    output2 += &stack.pop_front().unwrap();
  }

  println!("Output for part 1 is {}", output1);
  println!("Output for part 2 is {}", output2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn read_file() -> (Vec<String>, Vec<String>) {
  let mut line_number = 1;

  let mut crates: Vec<String> = Vec::new();
  let mut instructions: Vec<String> = Vec::new();

  if let Ok(lines) = read_lines("input_files/input_day_05.txt") {
    for line in lines {
      if line_number <= 8 {
        if let Ok(input) = line {
          crates.push(input);
        }
      } else if line_number >= 11 {
        if let Ok(input) = line {
          instructions.push(input)
        }
      }
      line_number += 1;
    }
  }

  return (crates, instructions);
}

fn create_manifest(crates: Vec<String>) -> Vec<VecDeque<String>> {
  let mut output: Vec<VecDeque<String>> = vec![VecDeque::new(); 9];

  for line in crates {
    if !line.chars().nth(1).unwrap().is_whitespace() {
      output[0].push_back(line.chars().nth(1).unwrap().to_string());
    }
    if !line.chars().nth(5).unwrap().is_whitespace() {
      output[1].push_back(line.chars().nth(5).unwrap().to_string());
    }
    if !line.chars().nth(9).unwrap().is_whitespace() {
      output[2].push_back(line.chars().nth(9).unwrap().to_string());
    }
    if !line.chars().nth(13).unwrap().is_whitespace() {
      output[3].push_back(line.chars().nth(13).unwrap().to_string());
    }
    if !line.chars().nth(17).unwrap().is_whitespace() {
      output[4].push_back(line.chars().nth(17).unwrap().to_string());
    }
    if !line.chars().nth(21).unwrap().is_whitespace() {
      output[5].push_back(line.chars().nth(21).unwrap().to_string());
    }
    if !line.chars().nth(25).unwrap().is_whitespace() {
      output[6].push_back(line.chars().nth(25).unwrap().to_string());
    }
    if !line.chars().nth(29).unwrap().is_whitespace() {
      output[7].push_back(line.chars().nth(29).unwrap().to_string());
    }
    if !line.chars().nth(33).unwrap().is_whitespace() {
      output[8].push_back(line.chars().nth(33).unwrap().to_string());
    }
  }

  return output;
}

fn create_instruction_list(instructions: Vec<String>) -> Vec<Vec<usize>> {
  let mut output: Vec<Vec<usize>> = Vec::new();

  for line in instructions {
    let mut line_out: Vec<usize> = Vec::new();

    let splice = line.split(' ');
    let splits: Vec<&str> = splice.collect();

    line_out.push(splits[1].parse::<usize>().unwrap());
    line_out.push(splits[3].parse::<usize>().unwrap());
    line_out.push(splits[5].parse::<usize>().unwrap());

    output.push(line_out);
  }

  return output;
}
