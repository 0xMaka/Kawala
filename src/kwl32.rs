//----------------------------------------------------------------------------//
/*                                                     MIT License 2024 Maka  */
// -----------------------------------------------------------------   o8o   -
/*                                                                   o8||0o   |
 @title  : kwl32::util                                            C(•Ω•)D     |
 @notice : functions for working with 32 byte words                (_  כ      |
 @author : Maka                                                       || kwla |
*/                                                                   /*\----*/
/* ----------------------------------------------------------------------------
           Try to do at least one thing well, then leave it at that.
-----------------------------------------------------------------------------*/

pub mod util {
use std::cmp::{ min, max };

  // output the result of right padding input with zeros
  pub fn pad32r(bytes: &[u8]) -> [u8;32] {
    let ost = min(32, bytes.len() as i32) as usize;
    let mut padded = [0u8;32]; padded[..ost] . copy_from_slice(&bytes[..ost]);
    padded
  }
  // output the result of left padding input with zeros
  pub fn pad32l(bytes: &[u8])  -> [u8;32] {
    let ost = min(32, bytes.len() as i32) as usize;
    let pad = max(0, 32 as i32 - bytes.len() as i32) as usize;
    let mut padded = [0u8;32]; padded[pad..] . copy_from_slice(&bytes[..ost]);
    padded
  }

//-----------------------------------------------------------------------------

  // output the result of performing a given right shift on the input
  pub fn roll32r(bytes: &[u8;32], shift: usize) -> [u8;32] {
    let shift = shift % 32; /* bounds check */ if shift == 0 { return *bytes; }
    let mut buf = [0u8;32];
    buf[shift..] . copy_from_slice(&bytes[..32 - shift]);
    buf[..shift] . copy_from_slice(&bytes[32 - shift..]); buf
  }
  // output the result of performing a given left shift on the input
  pub fn roll32l(bytes: &[u8;32], shift: usize) -> [u8;32] {
    let shift = shift % 32; /* bounds check */ if shift == 0 { return *bytes; }
    let mut buf = [0u8;32];
    buf[32 - shift..] . copy_from_slice(&bytes[..shift]);
    buf[..32 - shift] . copy_from_slice(&bytes[shift..]); buf
  }

//-----------------------------------------------------------------------------
  
  // takes an arbitrary lengthed slice, returns a 32 byte slice
  pub fn chunk32(bytes: &[u8]) -> [u8;32] { 
    if bytes.len() < 32 { pad32r(bytes) } 
    else { let mut buf = [0u8;32]; buf . copy_from_slice(&bytes[..32]); buf }
  }
  // takes an arbitrary lengthed slice, returns an array of 32 byte slice(s)
  pub fn chunks32(bytes: &[u8]) -> Vec<[u8;32]> {
    (0..bytes.len()) . step_by(32) . map(|x| { chunk32(&bytes[x..]) })
    . collect::<Vec<[u8;32]>>()
  }

//-----------------------------------------------------------------------------

  // h/o abstraction to remove repetition
  fn _fab(f: &dyn Fn(u8,u8) -> u8, a: &[u8;32], b: &[u8;32]) -> [u8;32] {
    let mut buf = [0u8;32]; (0..32) . for_each(|i|buf[i] = f(a[i], b[i]));
    buf
  }
  // output the result of performing xor on a pair of 32 byte words
  pub fn xor32(a: &[u8;32], b: &[u8;32]) -> [u8;32] { _fab(&xoru8, a, b) }
  // output the result of performing and on a pair of 32 byte words
  pub fn and32(a: &[u8;32], b: &[u8;32]) -> [u8;32] { _fab(&andu8, a, b) }
  // output the result of performing or on a pair of 32 byte words
  pub fn or32 (a: &[u8;32], b: &[u8;32]) -> [u8;32] { _fab(&oru8 , a, b) }
  // output the result of performing not on the input
  pub fn not32(a: &[u8;32]             ) -> [u8;32] {
    let mut buf = [0u8;32]; (0..32) . for_each(|i|buf[i] = notu8(a[i])); buf
  }
  // passable comparitors
  fn xoru8(a: u8, b: u8) -> u8 { a ^ b } fn andu8(a: u8, b: u8) -> u8 { a & b }
  fn notu8(a: u8)        -> u8 { ! a   } fn oru8 (a: u8, b: u8) -> u8 { a | b }
}

/*
   End of util.
  //////////////////////////////////////////////////////////////////////// */
/* --------------------------------------------------------------------------*/
