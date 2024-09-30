//---------------------------------------- WORD TYPE --------------------------------------------//
// ----------------------------------------------------------------------------------------------//
#[cfg(test)]
mod word { 
  use kawala::{ Word };
// ----------------------------------- CREATION AND ACCESS --------------------------------------//

  #[test]
  fn word_creation_and_access_from_bytes() {
    let word = Word::from_bytes(&[0x01,0x02,0x03,0x04]);
    assert_eq!(word.bytes(), [0x01,0x02,0x03,0x04]);
    assert_eq!(word.len(),     4);
    assert_eq!(word.data()[0], 1); 
  }

  #[test]
  fn word_creation_and_access_from_hex() {
    let word  = Word::from_hex("01020304");
    assert_eq!(word.bytes(), [0x01,0x02,0x03,0x04]);
    assert_eq!(word.len(),     4);
    assert_eq!(word.data()[0], 1);
  }

// ---------------------------------------- LEN & HEX -------------------------------------------//

  #[test]
  fn hex_conversion() {
    let word = Word::from_bytes(&[0xFF; 32]);
    assert_eq!(word.hex(), "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
  }

  #[test]
  fn len_wrapper() {
    let word = Word::from_bytes(&[1, 2, 3, 4]);
    assert_eq!(word.len(), 4);
  }

// --------------------------------------- COMPARISONS ------------------------------------------//

  #[test]
  fn word_equality_match() {
    let a     = Word::from_bytes(&[0u8;4]);
    let b     = Word::from_bytes(&[0u8;4]);
    assert_eq!(a, b);
  }

  #[test]
  fn word_equality_no_match() {
    let a     = Word::from_bytes(&[0u8;4]);
    let b     = Word::from_bytes(&[1u8;4]);
    assert_ne!(a, b);
  }

// ---------------------------------------   GENERAL   ------------------------------------------//

  #[test]
  fn word() -> () { 
    /* initialize a kawala::Word */
    let word   = Word::from_bytes(&[0u8;32]);
    assert_eq!(word.len(), 32);
    let word   = Word::from_hex(&"0".repeat(64));
    assert_eq!(word.len(), 32);
  }

  #[test]
  fn more_than_32() {
    let word = Word::from_bytes(&[0u8;64]);
    assert_eq!(word.len(), 32);
  }
 
  #[test]
  fn general_methods() {
    let word  = Word::from_bytes(&[0u8;64]);
    let bytes = word.bytes();
    let hex   = word.hex();
    let len   = word.len();
    assert_eq!(hex,      "0".repeat(64));
    assert_eq!(bytes,    [0u8;32]);
    assert_eq!(len,       32);
  }
}

