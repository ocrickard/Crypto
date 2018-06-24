
// The basic trait for conversion to/from string and binary.
pub trait Encodable {
  fn decode_to_bin(&self) -> Vec<u8>;
  fn encode_to_utf8(&self) -> Vec<u8>;
}

pub trait StringConstructible {
  fn from_encoded_utf8_buffer(buffer: &[u8]) -> Self;
}

pub trait Xorable {
  fn xor_value(&self, value: &u8) -> Self;
  fn xor_sequential(&self, sequence: &[u8]) -> Self;
}

mod xor {
  pub fn xor_buffer(buffer: &[u8], rhs: &u8) -> Vec<u8> {
    buffer.iter().map(|c| c ^ rhs).collect()
  }

  pub fn xor_sequential(buffer: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(key.len() > 0);
    let mut output: Vec<u8> = Vec::new();

    for i in 0..buffer.len() {
      let b = buffer[i];
      let k = key[i % key.len()];
      output.push(b ^ k);
    }

    output
  }
}

mod hex {
  use Encodable;
  use StringConstructible;
  use Xorable;
  use xor;

  #[derive(Debug)]
  pub struct HexBuffer {
    // Internal representation is a decoded binary buffer.
    pub raw_bytes: Vec<u8>
  }

  impl Encodable for HexBuffer {
    fn decode_to_bin(&self) -> Vec<u8> {
      self.raw_bytes.to_vec()
    }
    fn encode_to_utf8(&self) -> Vec<u8> {
      encode_hex_buffer(&self.raw_bytes)
    }
  }

  impl StringConstructible for HexBuffer {
    fn from_encoded_utf8_buffer(buffer: &[u8]) -> Self {
      HexBuffer { raw_bytes: decode_hex_buffer(buffer) }
    }
  }

  impl Xorable for HexBuffer {
    fn xor_value(&self, value: &u8) -> Self {
      let xored = xor::xor_buffer(&self.raw_bytes, value);
      HexBuffer { raw_bytes: xored }
    }

    fn xor_sequential(&self, sequence: &[u8]) -> Self {
      let xored = xor::xor_sequential(&self.raw_bytes, sequence);
      HexBuffer { raw_bytes: xored }
    }
  }

  // 0123456789abcdef
  static HEX_CONVERSION_TABLE: &'static [u8] = &[48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102];

  // Take a hex-encoded utf8 byte and convert to a binary byte.
  fn decode_hex_byte(byte: &u8) -> u8 {
    match HEX_CONVERSION_TABLE.iter().position(|&s| s == *byte) {
      Some(output) => output as u8,
      // TODO: consider panicking or returning an error instead of silently failing.
      None => 0
    }
  }

  // Take a binary byte and convert to a hex-encoded utf8 byte.
  fn encode_hex_byte(byte: &u8) -> u8 {
    HEX_CONVERSION_TABLE[*byte as usize]
  }

  // Take a hex-encoded utf8 buffer and convert to a binary buffer.
  fn decode_hex_buffer(buffer: &[u8]) -> Vec<u8> {
    // Each hex byte holds 4 bits of information for the binary buffer.
    // Thus we just have to combine each pair of 2 values in the original
    // buffer, decode their values, and we've got a binary buffer.
    buffer.chunks(2).map(|c| {
      (decode_hex_byte(&c[0]) << 4) | decode_hex_byte(&c[1])
    }).collect()
  }

  // Take a binary buffer and encode to utf8 hex bytes.
  fn encode_hex_buffer(buffer: &[u8]) -> Vec<u8> {
    // Each binary byte contains 8 bits of information. We need to map
    // this to twice that number of hex bytes, which each contain 4 bits.
    let mut output: Vec<u8> = Vec::new();

    for c in buffer {
      output.push(encode_hex_byte(&(c >> 4)));
      output.push(encode_hex_byte(&(c & 0b1111)));
    }

    output
  }
}

mod b64 {
  use Encodable;
  use StringConstructible;
  use Xorable;
  use xor;

  #[derive(Debug)]
  pub struct B64Buffer {
    // Internal representation is a decoded binary buffer.
    pub raw_bytes: Vec<u8>
  }

  impl Encodable for B64Buffer {
    fn decode_to_bin(&self) -> Vec<u8> {
      self.raw_bytes.to_vec()
    }
    fn encode_to_utf8(&self) -> Vec<u8> {
      encode_b64_buffer(&self.raw_bytes)
    }
  }

  impl StringConstructible for B64Buffer {
    fn from_encoded_utf8_buffer(buffer: &[u8]) -> Self {
      B64Buffer { raw_bytes: decode_b64_buffer(buffer) }
    }
  }

  impl Xorable for B64Buffer {
    fn xor_value(&self, value: &u8) -> Self {
      let xored = xor::xor_buffer(&self.raw_bytes, value);
      B64Buffer { raw_bytes: xored }
    }

    fn xor_sequential(&self, sequence: &[u8]) -> Self {
      let xored = xor::xor_sequential(&self.raw_bytes, sequence);
      B64Buffer { raw_bytes: xored }
    }
  }

  // ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/
  static B64_CONVERSION_TABLE: &'static [u8] = &[65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 43, 47];
  
  // Take a utf8 base64 representation and convert to its binary value.
  fn decode_b64_byte(byte: &u8) -> u8 {
    match B64_CONVERSION_TABLE.iter().position(|&s| s == *byte) {
      Some(output) => output as u8,
      // TODO: consider panicking or returning an error instead of silently failing.
      None => 0
    }
  }

  // Take a binary value and encode it to a base-64 mapped utf8 byte.
  fn encode_b64_byte(byte: &u8) -> u8 {
    B64_CONVERSION_TABLE[*byte as usize]
  }

  // Take a binary buffer and translate it to its Base-64 encoded binary representation.
  fn encode_b64_buffer(buffer: &[u8]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    for (_, c) in buffer.chunks(3).enumerate() {
      // Each binary byte is formed of 8 bits, which needs to be translated into
      // groupings of 6 bits which can be represented in Base64. Thus, we chunk
      // our input bytes into groups of 3 8-bit bytes. From these 24 bits we can
      // create 4 6-bit Base-64 values.

      // The first byte is the first 6 bits from c0.
      output.push(encode_b64_byte(&(c[0] >> 2)));
      // The second byte is the last 2 bits from c0, plus the first 4 bits of c1.
      output.push(encode_b64_byte(&(((c[0] & 0b11) << 4) | (c[1] >> 4))));
      // The third byte is the last 4 bits of c1, plus the first 2 bits of c2.
      output.push(encode_b64_byte(&(((c[1] & 0b1111) << 2) | (c[2] >> 6))));
      // The last byte is the last 6 bits of c2, and that's it!
      output.push(encode_b64_byte(&(c[2] & 0b111111)));
    }
    output
  }

  // Take a Base-64 utf8 encoded buffer and decode it to its binary representation.
  fn decode_b64_buffer(buffer: &[u8]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for (_, c) in buffer.chunks(4).enumerate() {
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
}

mod strings {
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
}

#[cfg(test)]
mod tests {
  use b64::B64Buffer;
  use hex::HexBuffer;
  use Encodable;
  use StringConstructible;
  use Xorable;
  use std;
    #[test]
    fn hex_buffer_converts_string() {
      let buf = HexBuffer { raw_bytes: "hey! my name is Oliver, what's yours?".as_bytes().to_vec() };
      println!("{:?}", std::str::from_utf8(&buf.encode_to_utf8()));
    }
}
