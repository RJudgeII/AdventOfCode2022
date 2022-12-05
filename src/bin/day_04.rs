use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  let elf_list = read_file();
  let mut total1 = 0;
  let mut total2 = 0;

  for pair in elf_list.clone() {
    if (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1])
      || (pair[1][0] >= pair[0][0] && pair[1][1] <= pair[0][1])
    {
      total1 += 1;
      total2 += 1;
      continue;
    }

    if (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][0])
      || (pair[1][0] <= pair[0][0] && pair[1][1] >= pair[0][0])
      || (pair[0][0] <= pair[1][1] && pair[0][1] >= pair[1][1])
      || (pair[1][0] <= pair[0][1] && pair[1][1] >= pair[0][1])
    {
      total2 += 1;
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

  if let Ok(lines) = read_lines("input_day_04.txt") {
    for line in lines {
      let mut elves: Vec<Vec<usize>> = Vec::new();
      let mut elf1: Vec<usize> = Vec::new();
      let mut elf2: Vec<usize> = Vec::new();
      if let Ok(input) = line {
        let splices = input.split(',');
        let elfs: Vec<&str> = splices.collect();

        let elf1_range_split = elfs[0].split('-');
        let elf2_range_split = elfs[1].split('-');

        let elf1_range: Vec<&str> = elf1_range_split.collect();
        let elf2_range: Vec<&str> = elf2_range_split.collect();

        elf1.push(elf1_range[0].parse::<usize>().unwrap());
        elf1.push(elf1_range[1].parse::<usize>().unwrap());
        elf2.push(elf2_range[0].parse::<usize>().unwrap());
        elf2.push(elf2_range[1].parse::<usize>().unwrap());

        elves.push(elf1);
        elves.push(elf2);
      }
      output.push(elves);
    }
  }

  return output;
}
