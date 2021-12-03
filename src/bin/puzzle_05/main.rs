use utils::files::read_lines;

fn convert_counter_to_binary_string(counter: Vec<i8>) -> String {
  let mut binary_string = String::new();
  for i in counter {
    let current = if i <= 0i8 { 0 } else { 1 };
    binary_string.push_str(current.to_string().as_str());
  }
  binary_string
}

fn negate_binary_string(input: &String) -> String {
  let mut output = String::new();
  for c in input.chars() {
    let current = if c == '0' { '1' } else { '0' };
    output.push(current);
  }
  output
}

fn convert_binary_string_to_number(input: String) -> i32 {
  let mut output = 0i32;
  for (i, c) in input.chars().rev().enumerate() {
    let current = if c == '1' { 1 } else { 0 };
    output += current * 2i32.pow(i as u32);
  }
  output
}

fn main() {
  // let path = "./src/bin/puzzle_05/test_data.txt";
  let path = "./src/bin/puzzle_05/data.txt";

  let mut counters = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

  if let Ok(lines) = read_lines(path) {
    for line_result in lines {
      if let Ok(line) = line_result {
        for (i, c) in line.chars().enumerate() {
          match c {
            '0' => counters[i] -= 1,
            '1' => counters[i] += 1,
            _ => (),
          }
        }
      }
    }
  }

  println!("Counters: {:?}", counters);

  let gamma_bin_str = convert_counter_to_binary_string(counters);
  let epsilon_bin_str = negate_binary_string(&gamma_bin_str);

  println!("Gamma (bin): {}", gamma_bin_str);
  println!("Epsilon (bin): {}", epsilon_bin_str);

  let gamma = convert_binary_string_to_number(gamma_bin_str);
  let epsilon = convert_binary_string_to_number(epsilon_bin_str);

  println!("Gamma (dec): {}", gamma);
  println!("Epsilon (dec): {}", epsilon);

  println!("Power: {}", gamma * epsilon);
}
