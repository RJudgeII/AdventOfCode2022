use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
struct Elf {
  list_of_calories: Vec<usize>,
  total_calories: usize,
}

fn main() {
  let mut list_of_elves: Vec<Elf> = Vec::new();
  let mut current_elf = Elf {
    list_of_calories: Vec::new(),
    total_calories: 0,
  };

  if let Ok(lines) = read_lines("input_files/input_day_01.txt") {
    for line in lines {
      if let Ok(input) = line {
        let test = input.parse::<usize>();

        match test {
          Ok(ok) => {
            current_elf.list_of_calories.push(ok);
            current_elf.total_calories += ok;
          }
          Err(_e) => {
            list_of_elves.push(current_elf.clone());
            current_elf = Elf {
              list_of_calories: Vec::new(),
              total_calories: 0,
            }
          }
        }
      }
    }

    let mut max_calories: usize = 0;
    let mut top_three: [usize; 3] = [0; 3];

    for elf in list_of_elves.clone() {
      if elf.total_calories > top_three[0] {
        top_three[0] = elf.total_calories;
      }
      top_three.sort();

      if elf.total_calories > max_calories {
        max_calories = elf.total_calories;
      }
    }

    println!("The elf with the most Calories has {} total.", max_calories);
    println!(
      "The top three elves combined have a total of {} Calories.",
      top_three[0] + top_three[1] + top_three[2]
    );
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
