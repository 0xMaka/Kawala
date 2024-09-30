//--------                       -------- KWL32::CON TEST --------                       --------//
//-----------------------------------------------------------------------------------------------//

#[cfg(test)]
mod kwl32_util {  
  use kawala::kwl32::util;

//--------                        --------     PAD32     --------                        --------//
  
  // empty 32
  #[test]
  fn pad32r_empty() {
    assert_eq!(util::pad32r(&[]), [0u8;32]);
  }

  #[test]
  fn pad32l_empty() {
    assert_eq!(util::pad32l(&[]), [0u8;32]);
  }
  
  // exact 32
  #[test]
  fn pad32r_exact_32_bytes() {
    let input    = [0u8;32];
    assert_eq!(util::pad32r(&input), input);
  }
  
  #[test]
  fn pad32l_exact_32_bytes() {
    let input    = [0u8; 32];
    assert_eq!(util::pad32l(&input), input);
  }
  
  // less than 32
  #[test]
  fn pad32r_less_than_32_bytes() {
    let input    = [1u8;16];
    let expected = [[1u8;16], [0u8;16]] .concat();
    assert_eq!(util::pad32r(&input), expected . as_slice());
  }

  #[test]
  fn pad32l_less_than_32_bytes() {
    let input    = [1u8; 16];
    let expected = [[0u8;16], [1u8;16]] .concat();
    assert_eq!(util::pad32l(&input), expected . as_slice());
  }
   
  
  // more than 32 (will trunk):
  #[test]
  fn pad32r_more_than_32_bytes() {
    let input    = [0u8;64];
    let expected = [0u8;32];
    assert_eq!(util::pad32r(&input), expected);
  }
  
  #[test]
  fn pad32l_more_than_32_bytes() {
    let input    = [0u8;64];
    let expected = [0u8;32];
    assert_eq!(util::pad32l(&input), expected);
  }
    
//--------                        --------    ROLL32     --------                        --------//

  // no shift:
  #[test]
  fn roll32r_no_shift() {
    let input = [0u8;32];
    assert_eq!(util::roll32r(&input, 0), input);
  }
  
  #[test]
  fn roll32l_no_shift() {
    let input = [0u8;32];
    assert_eq!(util::roll32l(&input, 0), input);
  }
    
  // single shift
  #[test]
  fn roll32r_single_byte_shift() {
    let input    = util::pad32l(&[1u8]); 
    let expected = [1u8]
    . into_iter()
    . chain([0u8;31])
    . collect::<Vec<u8>>();
    assert_eq!(util::roll32r(&input, 1), expected . as_slice());
  }
  
  #[test]
  fn roll32l_single_byte_shift() {
    let input    = util::pad32r(&[1u8]); 
    let expected = [0u8;31]
    . into_iter()
    . chain([1u8])
    . collect::<Vec<u8>>();
    assert_eq!(util::roll32l(&input, 1), expected . as_slice());
  } 
  
  // multi shift
  #[test]
  fn roll32r_multiple_byte_shift() {
    let input    = util::pad32r(&[1u8;16]); 
    let expected = [0u8; 16]
    . into_iter()
    . chain([1u8; 16])
    . collect::<Vec<u8>>();
    assert_eq!(util::roll32r(&input, 16), expected . as_slice());
  }
  
  #[test]
  fn roll32l_multiple_byte_shift() {
    let input    = util::pad32l(&[1u8;16]); 
    let expected = [1u8; 16]
    . into_iter()
    . chain([0u8; 16])
    . collect::<Vec<u8>>();
    assert_eq!(util::roll32l(&input, 16), expected . as_slice());
  }
   
  
  // shift more than 32:
  #[test]
  fn roll32r_shift_exceeding_32() {
    let input    = [1u8; 32];
    let expected = input;
    assert_eq!(util::roll32r(&input, 64), expected);
  }
  
  #[test]
  fn roll32l_shift_exceeding_32() {
    let input    = [1u8; 32];
    let expected = input;
    assert_eq!(util::roll32l(&input, 64), expected);
  }

//--------                        --------   CHUNK 32    --------                        --------//

  // less input
  #[test]
  fn chunk32_less_than_32_bytes() {
    let input    = [0u8; 16];
    let expected = util::pad32r(&input);
    assert_eq!(util::chunk32(&input), expected);
  }

  // exact input
  #[test]
  fn chunk32_exact_32_bytes() {
    let input    = [0u8; 32];
    assert_eq!(util::chunk32(&input), input);
  }
  
  // more input
  #[test]
  fn chunk32_more_than_32_bytes() {
    let input    = [0u8; 64];
    let expected = [0u8; 32];
    assert_eq!(util::chunk32(&input), expected);
  }
  
  //--------                        --------   CHUNKS 32    --------                        --------//
  
  // empty input
  #[test]
  fn chunks32_empty() { assert_eq!(util::chunks32(&[]), Vec::<[u8;32]>::new()) }
  
  // exact input
  #[test]
  fn chunks32_exact_32_bytes() {
    let input    = [0u8; 32];
    assert_eq!(util::chunks32(&input), vec![input]);
  }
  
  // regular multi input
  #[test]
  fn chunks32_multiple_32_bytes() {
    let input    = [0u8; 96];
    assert_eq!(util::chunks32(&input), vec![
      [0u8;32],
      [0u8;32],
      [0u8;32]
    ]);
  }
  
  // irregular multi (will trunk)
  #[test]
  fn chunks32_not_multiple_32_bytes() {
    let input    = [0u8; 65];
    assert_eq!(util::chunks32(&input), vec![
      [0u8;32],
      [0u8;32],
      [0u8;32]
    ]);
  }

//--------                        -------     RESERVED    -------                        --------//

// -- should not compile regardless of flag ------------------------------------------------------|

/*
  #[test]
  fn xor32_less_than_32_bytes() { let _ = util::xor32(&[0u8;16],&[0u8;32]); }

  #[test]
  fn xor32_more_than_32_bytes() { let _ = util::xor32(&[0u8;64],&[0u8;32]); }

  #[test]
  fn and32_less_than_32_bytes() { let _ = util::and32(&[0u8;16],&[0u8;32]); }

  #[test]
  fn and32_more_than_32_bytes() { let _ = util::and32(&[0u8;64],&[0u8;32]); }

  #[test]
  fn or32_less_than_32_bytes() { let _ = util::or32(&[0u8;16],&[0u8;32]); }

  #[test]
  fn or32_more_than_32_bytes() { let _ = util::or32(&[0u8;64],&[0u8;32]); }

  #[test]
  fn not32_less_than_32_bytes() { let _ = util::not32(&[0u8;16]); }

  #[test]
  fn not32_more_than_32_bytes() { let _ = util::not32(&[0u8;64]); }
*/

// -- --------------------------------------- ----------------------------------------------------|

//--------                        --------  General Ops  --------                        --------//
 
  #[test]
  fn xor32() {
    let a = [0xFFu8; 32];
    let b = [0x00u8; 32];

    assert_eq!(util::xor32(&a, &b), a);
  }
  
  #[test]
  fn and32() {
    let a = [0xFFu8; 32];
    let b = [0x00u8; 32];
    assert_eq!(util::and32(&a, &b), [0u8; 32]);
  }

  #[test]
  fn or32() {
    let a = [0xFFu8; 32];
    let b = [0x00u8; 32];
    assert_eq!(util::or32(&a, &b), a);
  }
  
  #[test]
  fn not32() {
    let a = [0xFFu8; 32];
    assert_eq!(util::not32(&a), [0u8;32]);
  }

//--------                        --------  EDGE CASES   --------                        --------//
  
  #[test]
  fn xor32_same_input() {
    let a = [0xFFu8; 32];
    assert_eq!(util::xor32(&a, &a), [0u8; 32]);
  }

  #[test]
  fn and32_same_input() {
    let a = [0xFFu8; 32];
    assert_eq!(util::and32(&a, &a), a);
  }

  #[test]
  fn or32_same_input() {
    let a = [0xFFu8; 32];
    assert_eq!(util::or32(&a, &a), a);
  }

}

//-----------------------------------------------------------------------------------------------//
