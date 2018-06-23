
fn convert_hex_to_b64(input: &[u8]) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  // 1 hex byte = 4 bits
  // 1 b64 byte = 6 bits

  // Lowest common multiple of 4 and 6 is 12, so 3 hex bytes turn into 2 b64 bytes

  //    .   .   . hex
  // 010101010101
  //      .     . b64

  // So we chunk the input bytes into groups of 3

  let hex_conversion_table = "0123456789abcdef".as_bytes();

  for (_, hex) in input.chunks(3).enumerate() {
    // The hex grouping contains 3 bytes, we're going to convert these 3 bytes
    // into 2 b64 bytes, and append them to the output vector.

    let converted: Vec<u8> = hex.iter().map(|&h| { 
      hex_conversion_table.iter().position(|&s| s == h).unwrap() as u8
    }).collect();

    let b1 = (converted[0] << 2) | ((converted[1] & 12) >> 2);
    let b2 = ((converted[1] & 3) << 4) | converted[2];

    output.push(b1);
    output.push(b2);
  }

  let b64_conversion_table = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    .as_bytes();

  let b64_converted: Vec<u8> = output.iter().map(|&b| {
    b64_conversion_table[b as usize]
  }).collect();

  b64_converted
}

fn main() {
    let string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let base64_converted_bytes = convert_hex_to_b64(string.as_bytes());

    let output = std::str::from_utf8(&base64_converted_bytes);

    println!("{:?}", output);
}
