pub fn convert_binary_string_to_number(input: String) -> i32 {
  let mut output = 0i32;
  for (i, c) in input.chars().rev().enumerate() {
    let current = if c == '1' { 1 } else { 0 };
    output += current * 2i32.pow(i as u32);
  }
  output
}
