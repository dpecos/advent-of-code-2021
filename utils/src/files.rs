use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Vec<String>
where
  P: AsRef<Path>,
{
  let file = File::open(filename).expect("Unable to open file");
  let reader = io::BufReader::new(file);
  let lines = reader.lines().map(|l| l.unwrap());
  lines.collect()
}
