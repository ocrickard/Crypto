
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

pub mod xor {
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

pub mod hex {
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
  static HEX_CONVERSION_TABLE: &'static [u8; 16] = b"0123456789abcdef";

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

pub mod b64 {
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
  static B64_CONVERSION_TABLE: &'static [u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

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

      match c {
        [x] => {
          output.push(encode_b64_byte(&(x >> 2)));
          output.push(encode_b64_byte(&((x & 0b11) << 4)));
          output.push(61); // =
          output.push(61); // =
        },
        [x, y] => {
          output.push(encode_b64_byte(&(x >> 2)));
          output.push(encode_b64_byte(&(((x & 0b11) << 4) | (y >> 4))));
          output.push(encode_b64_byte(&((y & 0b1111) << 2)));
          output.push(61); // =
        },
        [x, y, z] => {
          output.push(encode_b64_byte(&(x >> 2)));
          output.push(encode_b64_byte(&(((x & 0b11) << 4) | (y >> 4))));
          output.push(encode_b64_byte(&(((y & 0b1111) << 2) | (z >> 6))));
          output.push(encode_b64_byte(&(z & 0b111111)));
        },
        _ => {}
      }
    }
    output
  }

  // Take a Base-64 utf8 encoded buffer and decode it to its binary representation.
  fn decode_b64_buffer(buffer: &[u8]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for (_, c) in buffer.chunks(4).enumerate() {
      // Each b64 value encompasses 6 bits. Thus a group of 4 b64-encoded chars
      // maps to three decoded values.

      match c {
        [x, y, 61, 61] => {
          output.push((decode_b64_byte(&x) << 2) | (decode_b64_byte(&y) >> 4));
        },
        [x, y, z, 61] => {
          output.push((decode_b64_byte(&x) << 2) | (decode_b64_byte(&y) >> 4));
          output.push(((decode_b64_byte(&y) & 0b1111) << 4) | (decode_b64_byte(&z) >> 2));
        }
        _ => {
          output.push((decode_b64_byte(&c[0]) << 2) | (decode_b64_byte(&c[1]) >> 4));
          output.push(((decode_b64_byte(&c[1]) & 0b1111) << 4) | (decode_b64_byte(&c[2]) >> 2));
          output.push(((decode_b64_byte(&c[2]) & 0b11) << 6) | decode_b64_byte(&c[3]));
        }
      }
    }
    output
  }
}

pub mod strings {
  use std::collections::HashMap;

  pub fn normality(frequency_map: &HashMap<u8, usize>) -> f32 {
    let mut sorted_vec: Vec<(&u8, &usize)> = frequency_map.iter().collect();
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

  pub fn frequency(input: &str) -> HashMap<u8, usize> {
    let input = input.to_lowercase();
    let mut map: HashMap<u8, usize> = HashMap::new();
    for byte in input.as_bytes().iter() {
      // skip spaces
      *map.entry(*byte).or_insert(0) += 1;
    }
    map
  }

  pub fn hamming_distance(lhs: &[u8], rhs: &[u8]) -> u32 {
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
    (dist + (lhs.len() as i32 - rhs.len() as i32).abs() * 8) as u32
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
  use strings;
    #[test]
    fn hex_buffer_converts_string() {
      let buf = HexBuffer { raw_bytes: "hey! my name is Oliver, what's yours?".as_bytes().to_vec() };
      assert_eq!(std::str::from_utf8(&buf.encode_to_utf8()).unwrap(), "68657921206d79206e616d65206973204f6c697665722c2077686174277320796f7572733f");
    }

    #[test]
    fn hex_buffer_converts_string_and_back() {
      let string = "hey! my name is Oliver, what's yours?";
      let raw_bytes = string.as_bytes().to_vec();
      let buf = HexBuffer { raw_bytes };
      let encoded = buf.encode_to_utf8();
      let new_buf = HexBuffer::from_encoded_utf8_buffer(&encoded);
      let decoded_bin = new_buf.decode_to_bin();
      let decoded = std::str::from_utf8(&decoded_bin).unwrap();
      assert_eq!(decoded, string);
    }

    #[test]
    fn b64_buffer_converts_string() {
      let buf = B64Buffer { raw_bytes: "hey! my name is Oliver, what's yours?".as_bytes().to_vec() };
      assert_eq!(std::str::from_utf8(&buf.encode_to_utf8()).unwrap(), "aGV5ISBteSBuYW1lIGlzIE9saXZlciwgd2hhdCdzIHlvdXJzPw==");
    }

    #[test]
    fn b64_buffer_converts_string_and_back() {
      let string = "hey! my name is Oliver, what's yours?";
      let raw_bytes = string.as_bytes().to_vec();
      let buf = B64Buffer { raw_bytes };
      let encoded = buf.encode_to_utf8();
      let new_buf = B64Buffer::from_encoded_utf8_buffer(&encoded);
      let decoded_bin = new_buf.decode_to_bin();
      let decoded = std::str::from_utf8(&decoded_bin).unwrap();
      assert_eq!(decoded, string);
    }

    #[test]
    fn hamming_distance_wokka_wokka() {
      assert_eq!(strings::hamming_distance(b"this is a test", b"wokka wokka!!!"), 37);
    }

    #[test]
    fn frequency_count_single_char_works() {
      assert_eq!(*strings::frequency("eeeeeeeeee").get(&101).unwrap(), 10);
    }

    #[test]
    fn frequency_count_multiple_char_works() {
      let freq = strings::frequency("eeeeeeeeeeaaaaa");
      assert_eq!(*freq.get(&101).unwrap(), 10);
      assert_eq!(*freq.get(&97).unwrap(), 5);
    }

    #[test]
    fn single_xor_buffer() {
      let buf = HexBuffer::from_encoded_utf8_buffer(b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
      let xored = buf.xor_value(&88);
      let decoded_bin = xored.decode_to_bin();
      let decoded_string = std::str::from_utf8(&decoded_bin).unwrap();
      assert_eq!(decoded_string, "Cooking MC\'s like a pound of bacon");
    }
}
