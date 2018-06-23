
static HEX_CONVERSION_TABLE: &'static [u8] = &[48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102];

fn decode_hex(value: &u8) -> u8 {
  HEX_CONVERSION_TABLE.iter().position(|&s| s == *value).unwrap() as u8
}

fn encode_hex(value: &u8) -> u8 {
  HEX_CONVERSION_TABLE[*value as usize]
}

fn xor_hex_buffers(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  for (l, r) in lhs.iter().zip(rhs.iter()) {
    output.push(encode_hex(&(decode_hex(l)^decode_hex(r))));
  }

  output
}

fn main() {
    let result = xor_hex_buffers(
      "1c0111001f010100061a024b53535009181c".as_bytes(),
      "686974207468652062756c6c277320657965".as_bytes());

    let output = std::str::from_utf8(&result);

    println!("{:?}", output);
}
