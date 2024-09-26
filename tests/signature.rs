#[cfg(test)]
mod tests {
  use kawala::Signature;

  #[test]
  fn signature() -> () { 
    /* initialize a kawala::Signature */
    let sig = Signature::from_hex("0x791ac94700d00d1e");

    // will truncate
    assert_eq!(sig.hex() . chars() . count(), 8);
    assert_eq!(sig.bytes() . len(), 4);
  }

  #[test]
  fn less_than_4() {
    let sig = Signature::from_hex("0x791ac9");

    // if it's less than 4 it can't be a signature
    // unwrap or default
    assert_eq!(sig.hex(), "00000000");
    assert_eq!(sig.bytes(), b"\x00\x00\x00\x00")
  }
}

