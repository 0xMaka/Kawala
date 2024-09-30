//--------------------------------------- CALLDATA TYPE -----------------------------------------//
// ----------------------------------------------------------------------------------------------//
#[cfg(test)]
mod calldata { 
  use kawala::{ Calldata };
// ----------------------------------- CREATION AND ACCESS --------------------------------------//

  #[test]
  fn call_creation_and_access_from_bytes() {
    let call = Calldata::from_bytes(&[0x01,0x02,0x03,0x04]);
    assert_eq!(call.bytes(), [0x01,0x02,0x03,0x04]);
    assert_eq!(call.len(),     4);
    assert_eq!(call.bytes()[0], 1); 
  }

  #[test]
  fn call_creation_and_access_from_hex() {
    let call  = Calldata::from_hex("01020304");
    assert_eq!(call.bytes(), [0x01,0x02,0x03,0x04]);
    assert_eq!(call.len(),     4);
    assert_eq!(call.bytes()[0], 1);
  }

// ---------------------------------------- LEN & HEX -------------------------------------------//

  #[test]
  fn hex_conversion() {
    let call = Calldata::from_bytes(&[0xFF; 32]);
    assert_eq!(call.hex(), "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
  }

  #[test]
  fn len_wrapper() {
    let call = Calldata::from_bytes(&[1, 2, 3, 4]);
    assert_eq!(call.len(), 4);
  }

// --------------------------------------- COMPARISONS ------------------------------------------//

  #[test]
  fn call_equality_match() {
    let a     = Calldata::from_bytes(&[0u8;4]);
    let b     = Calldata::from_bytes(&[0u8;4]);
    assert_eq!(a, b);
  }

  #[test]
  fn call_equality_no_match() {
    let a     = Calldata::from_bytes(&[0u8;4]);
    let b     = Calldata::from_bytes(&[1u8;4]);
    assert_ne!(a, b);
  }

// ---------------------------------------   GENERAL   ------------------------------------------//

  #[test]
  fn call() -> () { 
    let call   = Calldata::from_bytes(&[0u8;32]);
    assert_eq!(call.len(), 32);
    let call   = Calldata::from_hex(&"0".repeat(64));
    assert_eq!(call.len(), 32);
  }

  #[test]
  fn more_than_32() {
    let call = Calldata::from_bytes(&[0u8;64]);
    assert_eq!(call.len(), 64);
  }

}

