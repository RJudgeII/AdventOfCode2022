use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  let packs = read_file();
  let mut total1 = 0;
  for pack in packs.clone() {
    for c in pack[0].clone() {
      if pack[1].contains(&c) {
        total1 += c;
        break;
      }
    }
  }

  let packs_len = packs.len();
  let mut total2 = 0;

  for i in (0..packs_len).step_by(3) {
    println!("Looking at packs {}, {}, and {}", i, i + 1, i + 2);
    let pack1: Vec<usize> = vec![packs[i][0].clone(), packs[i][1].clone()]
      .into_iter()
      .flatten()
      .collect();
    let pack2: Vec<usize> = vec![packs[i + 1][0].clone(), packs[i + 1][1].clone()]
      .into_iter()
      .flatten()
      .collect();
    let pack3: Vec<usize> = vec![packs[i + 2][0].clone(), packs[i + 2][1].clone()]
      .into_iter()
      .flatten()
      .collect();

    for c in pack1 {
      if pack2.contains(&c) && pack3.contains(&c) {
        total2 += c;
        break;
      }
    }
  }

  println!("Total score for part 1 is {}", total1);
  println!("Total score for part 2 is {}", total2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn read_file() -> Vec<Vec<Vec<usize>>> {
  let mut output: Vec<Vec<Vec<usize>>> = Vec::new();

  if let Ok(lines) = read_lines("input_files/input_day_03.txt") {
    for line in lines {
      let mut pack: Vec<Vec<usize>> = Vec::new();
      if let Ok(input) = line {
        let (split1, split2) = input.split_at(input.len() / 2);
        pack.push(to_priorities(split1.to_string()));
        pack.push(to_priorities(split2.to_string()));
      }
      output.push(pack);
    }
  }

  return output;
}

fn to_priorities(input: String) -> Vec<usize> {
  let mut output: Vec<usize> = Vec::new();

  for c in input.chars() {
    if c.is_lowercase() {
      output.push((c as usize) - 96);
    } else {
      output.push((c as usize) - 38);
    }
  }

  return output;
}
