#[cfg(test)]
mod tests {
  use kawala::Bytes;

  #[test]
  fn bytes32() -> () {
    /* initialize a bytes32 */
    let bytes = Bytes::Bytes32([0u8;32]);
    /* Run some sanity checks */
    assert_eq!(bytes.len(), 32);
    assert_eq!(bytes.hex() . chars() . count(), 64);
    assert_eq!(bytes.bytes() . len(), 32);
  }
 
  #[test]
  fn byte_array() -> () {
    /* initialize a byte array */
    let bytes = Bytes::Array([0u8;64].to_vec());
    assert_eq!(bytes.len(), 64);
    assert_ne!(bytes.hex() . chars() . count(), 64);
  }

  #[test]
  fn small_array() -> () {
    /* initialize a byte array */
    let bytes = Bytes::Array([0u8;2].to_vec());
    assert_eq!(bytes.len(), 2);
  }

 #[test]
  fn bytes4() -> () {
    /* initialize a bytes32 */
    let bytes = Bytes::Bytes4([0u8;4]);
    assert_eq!(bytes.len(), 4);
    assert_eq!(bytes.hex() . chars() . count(), 8);
  }

/* 
  //won't accept over
  #[test]
  fn bytes_overflow() -> () {    
    /* initialize a bytes4 with 8 bytes */
    let bytes = Bytes::Bytes4([0u8;8]);
  }
*/
}

