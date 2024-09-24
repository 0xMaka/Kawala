/* ----------------------------------------------------------------------------

 @title  : Kawala - Just a Kawl Data Companion (Crafting to a high Kawalaty)
 @author : Maka 
                                                           MIT License 2024  */
// --------------------------------------------------------------------------*/
/* Bytes type                                                           ʕ·͡ᴥ·ʔ
-----------------------------------------------------------------------------*/
  ///////////////////////////////////////////////////////////////////////////  

#[derive(Debug)]
pub enum Bytes {
 Bytes4 ([u8;SIG_LEN]),
 Bytes32([u8;WORD_LEN]),
 Array  (Vec<u8>)
}

trait BytesTrait {
  fn bytes(&self) -> &[u8];
  fn hex(&self)   -> String;
  fn len(&self)   -> usize;
}

impl BytesTrait for Bytes {
  fn bytes(&self) -> &[u8] {
    match self {
      Bytes::Bytes4(bytes)  => bytes,
      Bytes::Bytes32(bytes) => bytes,
      Bytes::Array(bytes)   => bytes.as_slice()
    }
  }
  
  fn hex(&self) -> String { 
    bytes_to_hex(self.bytes()) // will unwrap_or_default
  }
  
  fn len(&self) -> usize  { self.bytes().len() }
}

impl<I: std::slice::SliceIndex<[u8]>> std::ops::Index<I> for Bytes {
  type Output = I::Output;
  fn index(&self, i: I) -> &Self::Output {
    match self {
      Bytes::Bytes4(bytes)  => &bytes[i],
      Bytes::Bytes32(bytes) => &bytes[i],
      Bytes::Array(bytes)   => &bytes.as_slice()[i]
    }
  }
}

/* ----------------------------------------------------------------------------
 Calldata structure
-----------------------------------------------------------------------------*/
  ///////////////////////////////////////////////////////////////////////////  

#[derive(Debug)]
pub struct Calldata {
  data : Bytes
}

trait DataTrait {
  fn new(data : Bytes)          ->  Self;
  fn bytes(&self)               ->  &[u8];
  fn hex(&self)                 ->  String;
  fn from_bytes(bytes : &[u8])  ->  Self;
  fn from_hex(string : &str)    ->  Self;
}

impl DataTrait for Calldata {
  fn new(bytes : Bytes) -> Self {
      Calldata { data : bytes }
  }
  
  fn bytes(&self) -> &[u8] {
    self.data.bytes()    
  }
  
  fn hex(&self) -> String { 
    self.data.hex()
  }
  
  fn from_bytes(bytes : &[u8]) -> Self {
    Self::new(Bytes::Array(bytes.to_vec()))
  }

  fn from_hex(string : &str) -> Self {
    Self::new(Bytes::Array(marshal_pre(string)))
  }
}

impl Calldata {
  fn prefixed(&self) -> String { 
    "0x".to_owned() + &self.data.hex()
  }
}

/* ----------------------------------------------------------------------------
 Signature structure
-----------------------------------------------------------------------------*/
  ///////////////////////////////////////////////////////////////////////////  

#[derive(Debug)]
pub struct Signature {
  data : Bytes
}

impl DataTrait for Signature {

  fn new(bytes : Bytes) -> Self {
      Signature { data : bytes }
  }
  
  fn bytes(&self) -> &[u8] {
    self.data.bytes()
  }
 
  fn hex(&self)  -> String {
    self.data.hex()
  }
  
  fn from_bytes(array : &[u8]) -> Self {
    Self::new(Bytes::Bytes4(
      array[..std::cmp::min(SIG_LEN, array.len())]
      . try_into()
      . unwrap_or(EMPTY_SIG)
    ))
  }

  fn from_hex(string : &str) -> Self {
    let bytes = marshal_pre(string);
    Self::new(Bytes::Bytes4({
      bytes[..std::cmp::min(SIG_LEN, bytes.len())]
      . try_into()
      . unwrap_or(EMPTY_SIG)
    }))
  }
}

/*----------------------------------------------------------------------------
 Word structure
-----------------------------------------------------------------------------*/
  ///////////////////////////////////////////////////////////////////////////  

#[derive(Debug)]
pub struct Word {
  data : Bytes
}

impl DataTrait for Word {

  fn new(bytes : Bytes) -> Self {
    Word { data : bytes }
  }
  
  fn bytes(&self) -> &[u8] {
    self.data.bytes()    
  }
  
  fn hex(&self)  -> String { 
    self.data.hex()
  }
  
  fn from_bytes(array : &[u8]) -> Self {
    let slice = &array[..std::cmp::min(WORD_LEN, array.len())];
    match array.len() == WORD_LEN {
      true => Self::new(Bytes::Bytes32(
        slice 
      . try_into()
      . unwrap_or(EMPTY_BYTES32)
      )),
      false => Self::new(Bytes::Array(
        slice 
      . try_into()
      . unwrap_or(Vec::from(EMPTY_U8_SLICE))
      ))
    }
  }
  
  fn from_hex(string : &str) -> Self {
    Self::from_bytes(marshal_pre(string).as_slice())
  }
}

impl Word {
  /* 
  So many of the functions want to work on a common 32 byte word, and in the 
  vast majority of cases that is fine. It's unlikely we would go through the 
  overhead of a View struct to work on less, but there are cases where the end 
  of a stream or array spans less than 32, and we still want to support that, 
  as well as have an ability to craft from a very primitive state.. 
  add command byte > right pad > xor amount and address to create stream, etc.
  */
  fn as_bytes32(&self) -> [u8;32] {
    match &self.data {
      Bytes::Bytes32(x) => *x,
      Bytes::Array(x)   => rpad32(&x . as_slice()),
      Bytes::Bytes4(x)  => rpad32(x)
 
    }
  }
}
/* ----------------------------------------------------------------------------
 View structure
-----------------------------------------------------------------------------*/
  ///////////////////////////////////////////////////////////////////////////  

#[derive(Debug)]
pub struct View {
  sig  : Option<Signature>, 
  page : Vec<Word>
}

pub enum WithSig { True, False }

impl View {

  fn new(call : Calldata, with_sig : WithSig) -> Self {
 
    let min  = |x,y| std::cmp::min(x,y); 
    let mins = |   | min(SIG_LEN, call.data.len());  // handle missuse of sig
 
     // chunk byte array
    let chunks = | bytes: &[u8], step: usize | -> Vec<Word> {
      (0..bytes.len()) . step_by(step) . map(|x| {
        Some(Word::from_bytes(&bytes[x..min(x + step, bytes.len())])) 
        . unwrap_or(Word::from_bytes(&EMPTY_BYTES32))
      }) 
      . collect::<Vec<Word>>()
    };    
    
    // no point forcing a shift trick in here just match
    match with_sig {
      WithSig::True  => View { 
        sig  : Some(Signature::from_bytes(&call.data.bytes()[..mins()])), 
        page : Some(chunks(&call.data.bytes()[mins()..], WORD_LEN))
             . unwrap_or(chunks(&EMPTY_BYTES32, WORD_LEN))
      },
      WithSig::False => View { 
        sig  : None,
        page : Some(chunks(&call.data.bytes(), WORD_LEN)) 
             . unwrap_or(chunks(&EMPTY_BYTES32, WORD_LEN))
      }
    }
  }    
  // returns the 4 byte function signature as a hex str
  fn sig(&self) -> String {
    if let Some(x) = &self.sig { 
      bytes_to_hex(&x.data.bytes()) 
    } else { String::from("") }  
  }
  // returns the `arguments` portion of the calldata
  fn data(&self) -> String {
    if &self.word_count() <= &ONE_WORD { 
      self.word(0) 
    } else { self.page().join("") }
  }
  // returns the complete calldata, minus `0x` prefix
  fn calldata(&self) -> String { 
    self.sig() + &self.data() 
  }
  // returns the complete calldata with `0x` prefix
  fn prefixed(&self) -> String { 
    "0x".to_owned() + &self.sig() + &self.data()
  }
  // returns all 32 byte words as hex str's, in an array 
  fn page(&self) -> Vec<String> {
    self.words(ZERO_INDEX, self.word_count() -ZERO_OFFSET)
  }
  // returns a single word, will truncate overflow to max len
  fn word(&self, index : usize) -> String { 
    self.__word(index).hex() 
  }
  // returns a range of words, will trunc to max
  fn words(&self, start : usize, end : usize) -> Vec<String> { 
    self.__words(start, end)
    . iter()
    . map(|x|x.hex())
    . collect::<Vec<String>>()
  }
  // returns the number of word segments in array
  fn word_count(&self) -> usize { self.page.len() } 
  // quick prints a summary
  fn summary(&self) -> () {
    println!(
      "Signature: {}\nData:\n{}\nView:\n{:?}\nCount: {}",
      self.sig(), self.data(), self.page(), self.word_count()
    );
  }
  
/* ----------------------------------------------------------------------------
View cont..      destructive functions that mutate state
-----------------------------------------------------------------------------*/
  ///////////////////////////////////////////////////////////////////////////  
  
  // will trunk to 4 bytes, but return [0u8;4] if under, since wouldn't be a sig
  fn replace_sig(&mut self, bytes : &[u8]) -> () {
    self.sig = Some (Signature::from_bytes(bytes))
  }
  // private replaces a word, will replace last if pass out of bounds
  fn _replace_word(&mut self, index : usize, bytes : &[u8]) -> () {
    let slice_cap = std::cmp::min(WORD_LEN, bytes.len());
    let capped_id = std::cmp::min(index, &self.word_count() -ZERO_OFFSET);
    let slice     = &bytes[..slice_cap];
    self.page[capped_id] = Word::from_bytes(slice)
  }
  // replace word using a byte array as source
  fn replace_word_from_bytes(&mut self, index : usize, bytes : &[u8]) -> () {
    self._replace_word(index, bytes)
  }
  // replace word, first converting from a hex str as source
  fn replace_word_from_hex(&mut self, index : usize, string : &str) -> () { 
    self._replace_word(index, hex_to_bytes(string).as_slice())
  }
  
  fn left_pad_word(&mut self, index : usize) -> () {
    let word = self.__word(index);
    self.replace_word_from_bytes(index, &lpad32(word.data.bytes()))
  }
  
  fn right_pad_word(&mut self, index : usize) -> (){
    let word = self.__word(index);
    self.replace_word_from_bytes(index, &rpad32(word.data.bytes()))
  }
  // merge no overlap
  fn xor_a_into_b(&mut self, array_a : &[u8], index_b: usize) -> () {
    let _a = Word::from_bytes(array_a); let _b = self.__word(index_b);
    self.replace_word_from_bytes(index_b, &self.__xor_words((&_a, _b)).bytes())
  }
  // remove difference
  fn and_a_into_b(&mut self, array_a : &[u8], index_b: usize) -> () {
    let _a = Word::from_bytes(array_a); let _b = self.__word(index_b);
    self.replace_word_from_bytes(index_b, &self.__and_words((&_a, _b)).bytes())
  }
  // merge with overlap  (god tier?)
  fn or_a_into_b(&mut self, array_a : &[u8], index_b: usize) -> () {
    let _a = Word::from_bytes(array_a); let _b = self.__word(index_b);
    self.replace_word_from_bytes(index_b, &self.__or_words((&_a, _b)).bytes())
  }
  // flip 'em, the bird.
  fn not_a_into_a(&mut self, index_a: usize) -> () {
    let _a = self.__word(index_a);
    self.replace_word_from_bytes(index_a, &self.__not_word(&_a).bytes())
  }
  /* 
    ---------------------------------------------------------------------------
    Issue passing self.method functions as a params when mutable:
    Where mutable, it feels I need to better understand the borrower checker
    or drop the refs for the abstraction to result in less code when calling,
    than just naivly copy pasting the same thing a few times
    see _fops(). Come back to this later. [@Reader.. If you know, school me.]
    ---------------------------------------------------------------------------
    fn _fop<F>(
      &mut self, f : F, array : &[u8], index : usize
    ) -> () where F: FnOnce((Word, Word)) -> Word {
    let _a = Word::from_bytes(array); let _b = self.__word(index);
    self.replace_word_from_bytes(index, &f((&_a, _b)).bytes())
    }  
    // what iteration of this are we on, is it even relevent now?
    ---------------------------------------------------------------------------
  */
  
/* ----------------------------------------------------------------------------
View cont..   exposed functions that take or return Kawala types
-----------------------------------------------------------------------------*/
   ///////////////////////////////////////////////////////////////////////////  

  // append a word to the end 
  fn __append(&mut self, word : Word) -> () {
    self.page.push(word)
  }
  // pop a word from the end
  fn __pop(&mut self) -> Word {
    self.page.pop() . unwrap_or(Word::from_bytes(&EMPTY_BYTES32)) 
  }  
  // returns a ref to all 32 byte Words 
  fn __page(&self) -> &[Word] {
    &self.__words(ZERO_INDEX, self.word_count() -ZERO_OFFSET)
  }
  // returns a ref to a single Word, will truncate out of bounds to max len
  fn __word(&self, index : usize) -> &Word { 
    &self.page[std::cmp::min(index, &self.word_count() -ZERO_OFFSET)] 
  }
  // returns a ref to a range of Words, will trunc to max
  fn __words(&self, start : usize, end : usize) -> &[Word] { 
    &self.page[
      std::cmp::min(start, end)..
      std::cmp::min(end, &self.page.len() -ZERO_OFFSET)
    ]
  }
  /* 
    unneeded now? -------------------------------------------------------------
  // shouldn't be relied on, will pad to right as is the default
  fn __normalize_words(&self, a : &Word, b : &Word) -> (Word, Word) {
    let f = |x| Word::from_bytes(x); (f(&a.as_bytes32()), f(&b.as_bytes32()))
  }
  -----------------------------------------------------------------------------
  // legacy implementation - lifting moved to dedicated handler
  fn __normalize_words(&self, a : &Word, b : &Word) -> (Word, Word) {
    let f = |x| Word::from_bytes(x);
    if a.data.len() == WORD_LEN && WORD_LEN == b.data.len() { 
      return (f(a.data.bytes()), f(b.data.bytes())); } else {
      (f(&rpad32(a.data.bytes())), f(&rpad32(b.data.bytes())))
      }
  } 
  -----------------------------------------------------------------------------
  */
    
  // xor two words, these and the rest are useful for building byte streams
  fn __xor_words(&self, words : (&Word, &Word)) -> Word {
    self._fops(&xor32, (words.0, words.1))
  }
  // return result of a logical and on two 32 byte words
  fn __and_words(&self, words : (&Word, &Word)) -> Word {
    self._fops(&and32, (words.0, words.1))
  }
  // return the result of flipping the bits in a single 32 byte word
  fn __not_word(&self, word : &Word) -> Word {
    Word::from_bytes(&not32(&rpad32(&word.data.bytes())))
  }
  // return result of a logical or on two 32 byte words
  fn __or_words(&self, words : (&Word, &Word)) -> Word {
    self._fops(&or32, (words.0, words.1))
  }
  // higher order abstraction to save some repetition 
  fn _fops(
    &self, 
    f     : &dyn Fn(&[u8;WORD_LEN], &[u8;WORD_LEN]) -> [u8;WORD_LEN], 
    words : (&Word, &Word)
  ) -> Word {
    Word::from_bytes(&f(&words.0.as_bytes32(), &words.1.as_bytes32()))
  } 
}

/* ----------------------------------------------------------------------------
Appendix
-----------------------------------------------------------------------------*/
  ///////////////////////////////////////////////////////////////////////////  
  
// constants
const  EMPTY_U8_SLICE  :    [u8;0]        =   [0;0]; 
const  SIG_LEN         :    usize         =   4;
const  EMPTY_SIG       :    [u8;4]        =   [0;4]; 
const  WORD_LEN        :    usize         =   32;
const  EMPTY_BYTES32   :    [u8;32]       =   [0;32];

const  ZERO_INDEX      :    usize         =   0;
const  ZERO_OFFSET     :    usize         =   1;
const  ONE_WORD        :    usize         =   1;
// marshall through prefixed hex strings    
fn marshal_pre(fixed: &str) -> Vec<u8> {
  let ost = &fixed[..2] == "0x"; hex_to_bytes(&fixed[shift(ost)..])
}        
// just removes the need for an if to check hex, or multiplication of the bool.
fn shift(b : bool) -> usize { ((b as u8) << (b as u8) << 0) as usize }
// mod imports
use  con::{ bytes_to_hex, hex_to_bytes };
use util::{       lpad32, rpad32       };
use util::{ xor32, and32, not32, or32  };

/*
   End of core.
  //////////////////////////////////////////////////////////////////////// */  
/* ----------------------------------------------------------------------------
                                                     MIT License 2024 Maka  */ 
// ----------------------------------------------------------------------------

// -----------------------------------------------------------------    o8o   -
/*                                                                    o8||0o             
 @title  : Kwl32::util                                             C(•Ω•)D  
 @notice : some tools for Kawala                                     (_  כ 
 @author : Maka                                                        || kwla
*/                                                                    /*\----*/
/* ----------------------------------------------------------------------------
           Try to do at least one thing well, then leave it at that.
-----------------------------------------------------------------------------*/
pub mod util {
  pub fn rpad32(bytes: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32]; padded[..bytes.len()] . copy_from_slice(bytes);
    padded
  }

  pub fn lpad32(bytes: &[u8])  -> [u8; 32] {
    let padding    = std::cmp::max(0, 32 - bytes.len());
    let mut padded = [0u8; 32]; padded[padding..]     . copy_from_slice(bytes);
    padded
  }
//-----------------------------------------------------------------------------

 /* map can be better than simd for small arrays and we are working on 32 bytes
   so profile these! */
  #[cfg(feature = "simd")]
  use std::simd::{ u8x16, SimdFloat };

  fn _fab(f: &dyn Fn(u8,u8) -> u8, a: &[u8;32], b: &[u8;32]) -> [u8;32] {
    #[cfg(feature = "simd")]
    {
      let a_simd = u8x16::from_slice(a);
      let b_simd = u8x16::from_slice(b);
      let result_simd = f(a_simd, b_simd);
      return result_simd.to_array;
    }
    // Fallback
    let mut buf = [0u8;32]; (0..32) . for_each(|i|buf[i] = f(a[i], b[i]));
    buf    
  }

  pub fn xor32(a: &[u8;32], b: &[u8;32]) -> [u8;32] { _fab(&xoru8, a, b) }

  pub fn and32(a: &[u8;32], b: &[u8;32]) -> [u8;32] { _fab(&andu8, a, b) }  

  pub fn not32(a: &[u8;32]          ) -> [u8;32] { 
    let mut buf = [0u8;32]; (0..32) . for_each(|i|buf[i] = notu8(a[i])); buf
  }  
 
  pub fn or32 (a: &[u8;32], b: &[u8;32]) -> [u8;32] { _fab(&oru8 , a, b) }
  
  fn xoru8(a: u8, b: u8) -> u8 { a ^ b } fn andu8(a: u8, b: u8) -> u8 { a & b }
  fn notu8(a: u8)        -> u8 { ! a   } fn oru8 (a: u8, b: u8) -> u8 { a | b }
}
/*
   End of util.
  //////////////////////////////////////////////////////////////////////// */
  
/* ----------------------------------------------------------------------------
   @title  : Bai::con - Basic (as in) ascii (hex) integer (bytes) converter
   @author : Maka

   @notice : embedded style solution.. works on upper and lower nibbles 
           using a minimal lookup table.
   errors  : put garbage in get garbage out.. if you pass a value that 
           can't be parsed you can receive malformed data, else error 
           handling adheres to the unwrap_or_default philosophy.
// --------------------------------------------------------------------------*/

pub mod con {

  pub fn bytes_to_hex(bytes : &[u8]) -> String {
    bytes . iter() . map(|b| {
        let high_nibble = (b >> 4) & 0x0F;
        let low_nibble  = b & 0x0F;
        let left_side   = HEX_TABLE
          . get(high_nibble as usize)
          . unwrap_or(&(0x00,'0')).1;
        let right_side  = HEX_TABLE
          . get(low_nibble  as usize)
          . unwrap_or(&(0x00,'0')).1;
        format!("{}{}", left_side, right_side)
      })
      . collect::<String>()
  }
  
  pub fn hex_to_bytes(hex : &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    if hex.len() % 2 != 0 { return bytes; }
    (0..hex.len() / 2) . for_each(|i| {
      let high_nibble = hex
        . chars()
        . nth(i * 2)
        . unwrap_or_default();
      let low_nibble  = hex
        . chars()
        . nth(i * 2 + 1)
        . unwrap_or_default();
      let byte = u8::from_str_radix(&format!(
        "{}{}", high_nibble, low_nibble
      ), 16) 
        . unwrap_or(0x00);
      bytes.push(byte);
    });
    bytes
  }
  
  const HEX_TABLE: [(u8, char); 16] = [
    (0b0000, '0'), (0b0001, '1'), (0b0010, '2'), (0b0011, '3'), 
    (0b0100, '4'), (0b0101, '5'), (0b0110, '6'), (0b0111, '7'), 
    (0b1000, '8'), (0b1001, '9'), (0b1010, 'a'), (0b1011, 'b'), 
    (0b1100, 'c'), (0b1101, 'd'), (0b1110, 'e'), (0b1111, 'f')
  ];
}
/*
   End of con.
  //////////////////////////////////////////////////////////////////////// */
/* -------------------------------------------------------------------------------
                                                         MIT License 2024 Maka  */ 
 ////////////////////////////////////////////////////////////////////////////
