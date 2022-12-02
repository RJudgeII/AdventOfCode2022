use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  let sets = read_file();
  let mut total_score1: usize = 0;
  let mut total_score2: usize = 0;
  for set in sets {
    total_score1 += play_set1(set.clone());
    total_score2 += play_set2(set);
  }

  println!("Total score for part 1 is {}", total_score1);
  println!("Total score for part 2 is {}", total_score2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn read_file() -> Vec<Vec<String>> {
  let mut output: Vec<Vec<String>> = Vec::new();

  if let Ok(lines) = read_lines("input_day_02.txt") {
    for line in lines {
      let mut game: Vec<String> = Vec::new();
      if let Ok(input) = line {
        let parts = input.split(" ").collect::<Vec<_>>();
        game.push(parts[0].to_string());
        game.push(parts[1].to_string());
      }
      output.push(game);
    }
  }

  return output;
}

fn play_set1(set: Vec<String>) -> usize {
  if set[0] == "A" {
    if set[1] == "X" {
      return 4;
    } else if set[1] == "Y" {
      return 8;
    } else if set[1] == "Z" {
      return 3;
    }
  } else if set[0] == "B" {
    if set[1] == "X" {
      return 1;
    } else if set[1] == "Y" {
      return 5;
    } else if set[1] == "Z" {
      return 9;
    }
  } else if set[0] == "C" {
    if set[1] == "X" {
      return 7;
    } else if set[1] == "Y" {
      return 2;
    } else if set[1] == "Z" {
      return 6;
    }
  }
  return 0;
}

fn play_set2(set: Vec<String>) -> usize {
  if set[0] == "A" {
    if set[1] == "X" {
      return 3;
    } else if set[1] == "Y" {
      return 4;
    } else if set[1] == "Z" {
      return 8;
    }
  } else if set[0] == "B" {
    if set[1] == "X" {
      return 1;
    } else if set[1] == "Y" {
      return 5;
    } else if set[1] == "Z" {
      return 9;
    }
  } else if set[0] == "C" {
    if set[1] == "X" {
      return 2;
    } else if set[1] == "Y" {
      return 6;
    } else if set[1] == "Z" {
      return 7;
    }
  }
  return 0;
}
