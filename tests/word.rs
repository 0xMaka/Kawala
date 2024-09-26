#[cfg(test)]
mod word { 
  use kawala::{ Word };

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

