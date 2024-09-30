//---------------------------------------- BYTES TYPE --------------------------------------------//
//------------------------------------------------------------------------------------------------//
#[cfg(test)]
mod signature {
  use kawala::Signature;

// ----------------------------------- CREATION AND ACCESS --------------------------------------//

  #[test]
  fn sig_creation_and_access() {
    let sig   = Signature::from_bytes(&[0x01,0x02,0x03,0x04]);
    assert_eq!(sig.bytes(), [0x01,0x02,0x03,0x04]);
    assert_eq!(sig.len(), 4);
    assert_eq!(sig.bytes()[0],    1);
  }

// ---------------------------------------- LEN & HEX -------------------------------------------//

  #[test]
  fn hex_conversion() {
    let sig   = Signature::from_bytes(&[0xFF; 4]);
    assert_eq!(sig.hex(), "ffffffff");
  }

  #[test]
  fn len_wrapper() {
    let sig   = Signature::from_bytes(&[1, 2, 3, 4]);
    assert_eq!(sig.len(), 4);
  }

// --------------------------------------- COMPARISONS ------------------------------------------//

  #[test]
  fn sig_equality_match() {
    let a     = Signature::from_bytes(&[0u8;4]);
    let b     = Signature::from_bytes(&[0u8;4]);
    assert_eq!(a, b);
  }

  #[test]
  fn sig_equality_no_match() {
    let a     = Signature::from_bytes(&[0u8;4]);
    let b     = Signature::from_bytes(&[1u8;4]);
    assert_ne!(a, b);
  }

// --------------------------------------- GENERAL USE ------------------------------------------//

 #[test]
  fn sig() {
    let sig = Signature::from_bytes(&[0u8;4]);
    assert_eq!(sig.len(), 4);
    assert_eq!(sig.hex() . chars() . count(), 8);
    //... add a light show
  }
}

