//-----------------------------------------------------------------------------------------------//

//--------                        -------- BAI::CON TEST --------                        --------//
//-----------------------------------------------------------------------------
#[cfg(test)]
mod bai_con {  
  use kawala::bai::con
//-----------------------------------------------------------------------------------------------//

//--------                        -------- BYTES TO HEX  --------                        --------//

  /* BYTES TO HEX */
  // empty input
  #[test]
  fn bytes_to_hex_empty() { 
    assert_eq!(con::bytes_to_hex(&[]), ""); 
  }

  // single byte input
  #[test] 
  fn bytes_to_hex_single_byte() { 
    assert_eq!(con::bytes_to_hex(&[0xFF]), "ff"); 
  }

  // multi byte input
  #[test] 
  fn bytes_to_hex_multiple_bytes() {     
    assert_eq!(con::bytes_to_hex(&[0xAB, 0xCD]), "abcd"); 
  }

  // error handling - unwrap or default
  #[allow(overflowing_literals)]
  #[test]
  fn bytes_to_hex_invalid_input() {
    con::bytes_to_hex(&[0x100]); // we will unwrap or default
  }

//-----------------------------------------------------------------------------------------------//

//--------                        -------- HEX TO BYTES --------                        --------//

  // empty input
  #[test]
    fn hex_to_bytes_empty() {
    assert_eq!(con::hex_to_bytes(""), vec![]);
  }

  // single byte input
   #[test]
    fn hex_to_bytes_single_pair() {
    assert_eq!(con::hex_to_bytes("ff"), vec![0xFF]);
  }

  // multi byte input
  #[test] fn hex_to_bytes_multiple_bytes() {     
    assert_eq!(con::bytes_to_hex(&[0xBA, 0x11]), "ba11"); 
  }

  // invalid input
  #[test]
  fn hex_to_bytes_invalid_length() {
    assert_eq!(con::hex_to_bytes("abc"), vec![]);
  }

  // error handling - unwrap or default
  #[test]
  fn test_hex_to_bytes_invalid_char() {
    con::hex_to_bytes("g0");
  }

}

//-----------------------------------------------------------------------------
