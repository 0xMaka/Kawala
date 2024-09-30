//---------------------------------------- BYTES TYPE --------------------------------------------//
//------------------------------------------------------------------------------------------------//
#[cfg(test)]
mod bytes {
  use kawala::Bytes;
// ----------------------------------------------------------------------------------------------//

// ----------------------------------- CREATION AND ACCESS --------------------------------------//

  #[test]
  fn bytes4_creation_and_access() {
    let bytes = Bytes::Bytes4([0x01,0x02,0x03,0x04]);
    assert_eq!(bytes.bytes(), [0x01,0x02,0x03,0x04]);
    assert_eq!(bytes.len(), 4);
    assert_eq!(bytes[0],    1);
  }

  #[test]
  fn bytes32_creation_and_access() {
    let bytes = Bytes::Bytes32([0; 32]);
    assert_eq!(bytes.bytes().len(), 32);
    assert_eq!(bytes[0],             0);
  }

  #[test]
  fn array_creation_and_access() {
    let bytes = Bytes::Array(vec![0x01,0x02,0x03]);
    assert_eq!(bytes.bytes(), [1, 2, 3]);
    assert_eq!(bytes.len(), 3);
    assert_eq!(bytes[1], 2);
  }

  #[test]
  fn array_empty() {
    let bytes = Bytes::Array(vec![]);
    assert_eq!(bytes.bytes().len(), 0);
  }

// ---------------------------------------- LEN & HEX -------------------------------------------//

  #[test]
  fn hex_conversion() {
    let bytes = Bytes::Bytes32([0xFF; 32]);
    assert_eq!(bytes.hex(), "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
  }

  #[test]
  fn len_wrapper() {
    let bytes1 = Bytes::Bytes4([1, 2, 3, 4]);
    let bytes2 = Bytes::Bytes32([0; 32]);
    let bytes3 = Bytes::Array(vec![1, 2, 3, 4, 5]);

    assert_eq!(bytes1.len(), 4);
    assert_eq!(bytes2.len(), 32);
    assert_eq!(bytes3.len(), 5);
  }

// --------------------------------------- COMPARISONS ------------------------------------------//

  #[test]
  fn bytes4_equality_match() {
    let a     = Bytes::Bytes4([0u8;4]);
    let b     = Bytes::Bytes4([0u8;4]);
    assert_eq!(a, b);
  }

  #[test]
  fn bytes4_equality_no_match() {
    let a     = Bytes::Bytes4([0u8;4]);
    let b     = Bytes::Bytes4([1u8;4]);
    assert_ne!(a, b);
  }

  #[test]
  fn bytes32_equality_match() {
    let a     = Bytes::Bytes32([0u8;32]);
    let b     = Bytes::Bytes32([0u8;32]);
    assert_eq!(a, b);
  }

  #[test]
  fn bytes32_equality_no_match() {
    let a     = Bytes::Bytes32([0u8;32]);
    let b     = Bytes::Bytes32([1u8;32]);
    assert_ne!(a, b);
  }

  #[test]
  fn array_equality_match() {
    let a     = Bytes::Array(vec![0u8;4]);
    let b     = Bytes::Array(vec![0u8;4]);
    assert_eq!(a, b);
  }

  #[test]
  fn array_equality_no_match() {
    let a     = Bytes::Array(vec![0u8;4]);
    let b     = Bytes::Array(vec![1u8;4]);
    assert_ne!(a, b);
  }


// --------------------------------------- GENERAL USE ------------------------------------------//

  #[test]
  fn bytes32() {
    let bytes = Bytes::Bytes32([0u8;32]);
    assert_eq!(bytes.len(), 32);
    assert_eq!(bytes.hex() . chars() . count(), 64);
    assert_eq!(bytes.bytes() . len(), 32);
  }
 
  #[test]
  fn byte_array() {
    /* initialize a byte array */
    let bytes = Bytes::Array(vec![0u8;64]);
    assert_eq!(bytes.len(), 64);
    assert_ne!(bytes.hex() . chars() . count(), 64);
  }

  #[test]
  fn small_array() {
    /* initialize a byte array */
    let bytes = Bytes::Array(vec![0u8;2]);
    assert_eq!(bytes.len(), 2);
  }

 #[test]
  fn bytes4() {
    /* initialize a bytes32 */
    let bytes = Bytes::Bytes4([0u8;4]);
    assert_eq!(bytes.len(), 4);
    assert_eq!(bytes.hex() . chars() . count(), 8);
  }
}

