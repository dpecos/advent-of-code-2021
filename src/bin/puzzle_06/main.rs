use utils::files::read_lines;
use utils::strings::*;

fn count_bits(lines: &Vec<String>) -> Vec<i16> {
  // let mut counters = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  let mut counters = vec![];
  for line in lines {
    for (i, c) in line.chars().enumerate() {
      if i >= counters.len() {
        counters.push(0);
      }
      match c {
        '0' => counters[i] -= 1,
        '1' => counters[i] += 1,
        _ => (),
      }
    }
  }

  counters
}

fn convert_counter_to_binary_string(counter: Vec<i16>) -> String {
  println!("{:?}", counter);
  let mut binary_string = String::new();
  for i in counter {
    let current = if i < 0i16 {
      '0'
    } else if i == 0i16 {
      '='
    } else {
      '1'
    };
    binary_string.push(current);
  }
  binary_string
}

fn negate_binary_string(input: &String) -> String {
  let mut output = String::new();
  for c in input.chars() {
    match c {
      '0' => output.push('1'),
      '1' => output.push('0'),
      '=' => output.push('='),
      _ => (),
    }
  }
  output
}

fn filter_lines(path: &str, negate: bool) -> String {
  let mut result = read_lines(path);

  let mut criteria_pos = 0;

  while result.len() > 1 {
    let counters = count_bits(&result);
    let most_common_bits = convert_counter_to_binary_string(counters);

    let criteria_bits = if negate {
      negate_binary_string(&most_common_bits)
    } else {
      most_common_bits
    };

    let c = criteria_bits.chars().nth(criteria_pos).unwrap();

    let mut criteria = c;
    if c == '=' {
      criteria = if negate { '0' } else { '1' };
    }

    let mut filtered = vec![];
    for line in result.iter() {
      if criteria == line.chars().nth(criteria_pos).unwrap() {
        filtered.push(line.clone());
      }
    }
    println!(
      "{:?} {} {} - {:?} {:?}",
      criteria_bits, criteria_pos, criteria, result, filtered
    );
    result = filtered;

    criteria_pos += 1;
  }

  result[0].clone()
}

fn main() {
  // let path = "./src/bin/puzzle_05/test_data.txt";
  let path = "./src/bin/puzzle_05/data.txt";

  let oxygen_generation_rate_bin = filter_lines(&path, false);
  println!(
    "Oxygen generation rate (bin) {:?}",
    oxygen_generation_rate_bin
  );
  let oxygen_generation_rate = convert_binary_string_to_number(oxygen_generation_rate_bin);
  println!("Oxygen generation rate {}", oxygen_generation_rate);

  let co2_scrubber_rate_bin = filter_lines(&path, true);
  println!("CO2 scrubber rate (bin) {:?}", co2_scrubber_rate_bin);
  let co2_scrubber_rate = convert_binary_string_to_number(co2_scrubber_rate_bin);
  println!("CO2 scrubber rate {}", co2_scrubber_rate);

  println!("Result: {}", oxygen_generation_rate * co2_scrubber_rate);
}
