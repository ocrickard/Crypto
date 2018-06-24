
fn sequential_xor(buffer: &[u8], key: &[u8]) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  for i in 0..buffer.len() {
    let b = buffer[i];
    let k = key[i % key.len()];
    output.push(b ^ k);
  }

  output
}

static HEX_CONVERSION_TABLE: &'static [u8] = 
  &[48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102];

fn encode_hex_char(c: &u8) -> u8 {
  HEX_CONVERSION_TABLE[*c as usize]
}

fn encode_hex_buffer(buffer: &[u8]) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  for c in buffer {
    // split it into 2 pieces:
    let c1 = c >> 4;
    let c2 = c & 0b1111;
    output.push(encode_hex_char(&c1));
    output.push(encode_hex_char(&c2));
  }

  output
}

fn main() {

  let args: Vec<String> = std::env::args().collect();

  let input = &args[1];
  let key = &args[2];

  let xored = sequential_xor(&input.as_bytes(), &key.as_bytes());
  let xored_hex = encode_hex_buffer(&xored);
  let xored_string = std::str::from_utf8(&xored_hex).unwrap();

  print!("{}", xored_string);
}
