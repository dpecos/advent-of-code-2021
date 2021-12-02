use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  // let path = "./src/bin/puzzle_04/test-data.txt";
  let path = "./src/bin/puzzle_04/data.txt";

  let mut position = (0, 0, 0);

  if let Ok(lines) = read_lines(path) {
    for line_result in lines {
      if let Ok(line) = line_result {
        let mut line_split = line.split(' ');
        let movement = (line_split.next().unwrap(), line_split.next().unwrap());

        match movement {
          ("forward", x) => {
            let x = x.parse::<i32>().unwrap();
            position.0 += x;
            position.1 += x * position.2;
          }
          ("up", y) => {
            let y = y.parse::<i32>().unwrap();
            position.2 -= y;
          }
          ("down", y) => {
            let y = y.parse::<i32>().unwrap();
            position.2 += y;
          }
          _ => {
            println!("Unknown movement: {:?}", movement);
          }
        }
      }
    }
  }

  println!("Final position: {:?}", position);
  println!("Solution: {}", position.0 * position.1);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
