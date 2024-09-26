/* ----------------------------------------------------------------------------
   @title  : bai::con - base16 (as in) ascii (hex) integer (bytes) converter
   @author : Maka

   @notice : embedded style solution.. works on upper and lower nibbles
             using a minimal lookup table.
   errors  : put garbage in get garbage out.. if you pass a value that
             can't be parsed you can receive malformed data, else error
             handling adheres to the unwrap_or_default philosophy.
// --------------------------------------------------------------------------*/

pub mod con {

  pub fn bytes_to_hex(bytes : &[u8]) -> String {
    bytes . iter() . map(|b| {
        let high_nibble = (b >> 4) & 0x0F;
        let low_nibble  = b & 0x0F;
        let left_side   = HEX_TABLE
          . get(high_nibble as usize)
          . unwrap_or(&(0x00,'0')).1;
        let right_side  = HEX_TABLE
          . get(low_nibble  as usize)
          . unwrap_or(&(0x00,'0')).1;
        format!("{}{}", left_side, right_side)
      })
      . collect::<String>()
  }

  pub fn hex_to_bytes(hex : &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    if hex.len() % 2 != 0 { return bytes; }
    (0..hex.len() / 2) . for_each(|i| {
      let high_nibble = hex
        . chars()
        . nth(i * 2)
        . unwrap_or_default();
      let low_nibble  = hex
        . chars()
        . nth(i * 2 + 1)
        . unwrap_or_default();
      let byte = u8::from_str_radix(&format!(
        "{}{}", high_nibble, low_nibble
      ), 16)
        . unwrap_or(0x00);
      bytes.push(byte);
    });
    bytes
  }

  const HEX_TABLE: [(u8, char); 16] = [
    (0b0000, '0'), (0b0001, '1'), (0b0010, '2'), (0b0011, '3'),
    (0b0100, '4'), (0b0101, '5'), (0b0110, '6'), (0b0111, '7'),
    (0b1000, '8'), (0b1001, '9'), (0b1010, 'a'), (0b1011, 'b'),
    (0b1100, 'c'), (0b1101, 'd'), (0b1110, 'e'), (0b1111, 'f')
  ];
}
/*
   End of con.
  //////////////////////////////////////////////////////////////////////// */
/* --------------------------------------------------------------------------
                                                      MIT License 2024 Maka  */
