
use std::collections::HashMap;

fn normality(frequency_map: &HashMap<u8, u8>) -> f32 {
  let mut sorted_vec: Vec<(&u8, &u8)> = frequency_map.iter().collect();
  sorted_vec.sort_by(|lhs, rhs| rhs.1.cmp(lhs.1));
  let sorted_vec = sorted_vec.iter().map(|(val, _)| val);

  let expected = "etaoinshrdlcumwfgypbvkjxqz".as_bytes();

  let mut sum_similar: f32 = 0.0;

  for (i, actual) in sorted_vec.enumerate() {
    if let Some(exp_position) = expected.iter().position(|&s| s == **actual) {
      sum_similar += 1.0 - ((exp_position as f32 - (i as f32)).abs() / expected.len() as f32)
    }
  }
  sum_similar / (expected.len() as f32)
}

fn frequency(input: &str) -> HashMap<u8, u8> {
  let input = input.to_lowercase();
  let mut map: HashMap<u8, u8> = HashMap::new();
  for byte in input.as_bytes().iter() {
    if *byte != 32 { 
      // skip spaces
      *map.entry(*byte).or_insert(0) += 1;
    }
  }
  map
}

fn hamming_distance(lhs: &[u8], rhs: &[u8]) -> u32 {
  let mut dist = 0;
  for (l, r) in lhs.iter().zip(rhs.iter()) {
    // By taking the xor of l and r, we get the bits set to 1 that were different
    // between l and r's binary representation.
    let different_bits = l ^ r;
    for shift in 0..8 {
      // Bit shift this comparison bit through all the fields that could have
      // values so we can find out what bits are set in different_bits.
      let comparison = 0b1 << shift;
      if (different_bits & comparison) != 0 {
        dist += 1
      }
    }
  }
  // Not mentioned in the problem spec, but we consider differences of length
  // to be different for every bit, not just 1's. This is an impl detail.
  (dist + ((lhs.len() - rhs.len()) as i32).abs() * 8) as u32
}

// ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/
static B64_CONVERSION_TABLE: &'static [u8] = &[65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 43, 47];

fn decode_b64_byte(byte: &u8) -> u8 {
  match B64_CONVERSION_TABLE.iter().position(|&s| s == *byte) {
    Some(output) => output as u8,
    None => 0
  }
}

fn decode_b64_buffer(buffer: &[u8]) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
  for (i, c) in buffer.chunks(4).enumerate() {
    // Each b64 value encompasses 6 bits. Thus a group of 4 b64-encoded chars
    // maps to three decoded values.

    // For the first one, we want the all of the first b64 byte, plus 2 bits of the second.
    output.push((decode_b64_byte(&c[0]) << 2) | (decode_b64_byte(&c[1]) >> 4));
    // We next take the last 4 bits of the second b64 value, and then add the first 4 bits of the third.
    output.push(((decode_b64_byte(&c[1]) & 0b1111) << 4) | (decode_b64_byte(&c[2]) >> 2));
    // Finally, we need the last 2 bits of the third b64 value, and all of the fourth.
    output.push(((decode_b64_byte(&c[2]) & 0b11) << 6) | decode_b64_byte(&c[3]));
  }
  output
}

fn xor_buffer(buffer: &[u8], rhs: &u8) -> Vec<u8> {
  buffer.iter().map(|c| c ^ rhs).collect()
}

fn guessed_key_length(buffer: &[u8]) -> Vec<usize> {
  if buffer.len() < 80 {
    panic!("buffer not long enough to guess.");
  }
  let mut normalized_values: Vec<(usize, f32)> = Vec::new();
  for keysize in 1..40 {
    let first = &buffer[..keysize];
    let second = &buffer[keysize..keysize*2];
    let distance = hamming_distance(first, second);
    let normalized = (distance as f32) / (keysize as f32);

    let distance = hamming_distance(first, second);
    let normalized = (distance as f32) / (keysize as f32);
    normalized_values.push((keysize, normalized));
  }
  normalized_values.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
  normalized_values[..5].iter().map(|v| v.0 ).collect()
}

fn main() {
  let input = "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".to_string();
  let stripped = input.replace("\n", "");
  let decoded = decode_b64_buffer(&stripped.as_bytes());
  let guessed = guessed_key_length(&decoded);

  for guess in guessed {
    let chunkified = decoded.chunks(guess);

    // The transpose step maps 
    let mut transposed: Vec<Vec<u8>> = Vec::new();
    for _ in 0..guess {
      transposed.push(Vec::new());
    }

    for chunk in chunkified {
      for (j, val) in chunk.iter().enumerate() {
        transposed[j].push(*val);
      }
    }

    let mut keys: Vec<u8> = Vec::new();

    for transposed_bin in transposed {
      let mut max: f32 = 0.0;
      let mut index = 0;
      let mut utf8_max = "".to_string();

      // We only need to go up to 256, since each hex char is at most 8 bits, aka 2^8 options.
      for i in 1..256 {
        // First, we need to xor the buffer with i
        let xored = xor_buffer(&transposed_bin, &(i as u8));
        // At this point we have a buffer of xored bytes. Let's decode it into utf8 string for analysis/printing.
        let utf8 = match std::str::from_utf8(&xored) {
          Ok(string) => string,
          Err(_) => ""
        };
        let freq = frequency(&utf8);
        let norm = normality(&freq);
        
        if norm > max {
          max = norm;
          index = i;
          utf8_max = utf8.to_string();
        }
      }

      println!("Best guess is xor key {}: {}", index, utf8_max);
      keys.push(index);
    }
  }

}
