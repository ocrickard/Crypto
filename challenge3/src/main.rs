
use std::collections::HashMap;

fn normality(frequency_map: &HashMap<u8, u8>) -> f32 {
  let mut sorted_vec: Vec<(&u8, &u8)> = frequency_map.iter().collect();
  sorted_vec.sort_by(|lhs, rhs| rhs.1.cmp(lhs.1));
  let sorted_vec: Vec<u8> = sorted_vec.iter().map(|(val, _)| **val).collect();

  let expected = b"et aoinshrdlcumwfgypbvkjxqz";

  let mut sum_similar: f32 = 0.0;

  for (i, actual) in sorted_vec.iter().enumerate() {
    if let Some(exp_position) = expected.iter().position(|&s| s == *actual) {
      sum_similar += 1.0 - ((exp_position as f32 - (i as f32)).abs() / expected.len() as f32)
    }
  }
  sum_similar / (expected.len() as f32)
}

fn frequency(input: &str) -> HashMap<u8, u8> {
  let input = input.to_lowercase();
  let mut map: HashMap<u8, u8> = HashMap::new();
  for byte in input.as_bytes().iter() {
    // skip spaces
    *map.entry(*byte).or_insert(0) += 1;
  }
  map
}

// 0123456789abcdef
static HEX_CONVERSION_TABLE: &'static [u8] = 
  &[48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102];

fn decode_hex_char(value: &u8) -> u8 {
  match HEX_CONVERSION_TABLE.iter().position(|&s| s == *value) {
    Some(output) => output as u8,
    None => 0
  }
}

fn decode_hex_buffer(buffer: &[u8]) -> Vec<u8> {
  buffer.chunks(2).map(|c| {
    (decode_hex_char(&c[0]) << 4) | decode_hex_char(&c[1])
  }).collect()
}

fn xor_buffer(buffer: &[u8], rhs: &u8) -> Vec<u8> {
  buffer.iter().map(|c| c ^ rhs).collect()
}

fn main() {
  // This buffer was originally utf8 bytes. These bytes were broken up into 4-bit chunks and
  // converted to hex. The hex code was then xored with one hex char. We have to figure out
  // which char was xored with this hex buffer.
  let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

  let decoded_bin = decode_hex_buffer(input.as_bytes());

  let mut max: f32 = 0.0;
  let mut index = 0;
  let mut utf8_max = "".to_string();

  // We only need to go up to 256, since each hex char is at most 8 bits, aka 2^8 options.
  for i in 1..256 {
    // First, we need to xor the buffer with i
    let xored = xor_buffer(&decoded_bin, &(i as u8));
    // At this point we have a buffer of xored bytes. Let's decode it into utf8 string for analysis/printing.
    let utf8 = match std::str::from_utf8(&xored) {
      Ok(string) => string,
      Err(_) => ""
    };
    let freq = frequency(&utf8);
    let norm = normality(&freq);
    
    if norm > max {
      println!("{:?} {} freq: {:?}", utf8, max, freq);
      max = norm;
      index = i;
      utf8_max = utf8.to_string();
    }
  }

  println!("Best guess is xor key {}: {}", index, utf8_max);
}
