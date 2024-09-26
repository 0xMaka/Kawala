/* ----------------------------------------------------------------------------

 @title  : Kawala - Just a Kwl Data Companion (Crafting Kawality)
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

impl Bytes {
  pub fn bytes(&self) -> &[u8] {
    match self {
      Bytes::Bytes4(bytes)  => bytes,
      Bytes::Bytes32(bytes) => bytes,
      Bytes::Array(bytes)   => bytes.as_slice()
    }
  }

  pub fn hex(&self) -> String {
    bytes_to_hex(self.bytes()) // will unwrap_or_default
  }

  pub fn len(&self) -> usize  { self.bytes().len() }
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

impl Calldata {
  fn new(bytes : Bytes) -> Self {
      Calldata { data : bytes }
  }

  pub fn bytes(&self) -> &[u8] {
    self.data.bytes()
  }

  pub fn hex(&self) -> String {
    self.data.hex()
  }

  pub fn from_bytes(bytes : &[u8]) -> Self {
    Self::new(Bytes::Array(bytes.to_vec()))
  }

  pub fn from_hex(string : &str) -> Self {
    Self::new(Bytes::Array(marshal_pre(string)))
  }

  pub fn hex_0x(&self) -> String {
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

impl Signature {

  fn new(bytes : Bytes) -> Self {
      Signature { data : bytes }
  }

  pub fn bytes(&self) -> &[u8] {
    self.data.bytes()
  }

  pub fn hex(&self)  -> String {
    self.data.hex()
  }

  pub fn len(&self)  -> usize {
    self.data.len()
  }

  pub fn from_bytes(array : &[u8]) -> Self {
    Self::new(Bytes::Bytes4(
      array[..std::cmp::min(SIG_LEN, array.len())]
      . try_into()
      . unwrap_or(EMPTY_SIG)
    ))
  }

  pub fn from_hex(string : &str) -> Self {
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

  /* Core view type */

#[derive(Debug)]
pub struct Word {
  data : Bytes
}

impl Word {

  fn new(bytes : Bytes) -> Self {
    Word { data : bytes }
  }

  pub fn bytes(&self) -> &[u8] {
    self.data.bytes()
  }

  pub fn hex(&self)  -> String {
    self.data.hex()
  }

  pub fn data(&self) -> &Bytes {
    &self.data
  }   

  pub fn len(&self)  -> usize  { self.bytes().len() }

  pub fn from_bytes(array : &[u8]) -> Self {
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

  pub fn from_hex(string : &str) -> Self {
    Self::from_bytes(marshal_pre(string).as_slice())
  }

  /*
  So many of the functions want to work on a common 32 byte word, and in the
  vast majority of cases that is fine. It's unlikely we would go through the
  overhead of a View struct to work on less, but there are cases where the end
  of a stream or array spans less than 32, and we still want to support that,
  as well as have an ability to craft from a very primitive state..
  add command byte > right pad > xor amount and address to create stream, etc.
  */
  pub fn as_bytes32(&self) -> [u8;32] {
    match &self.data {
      Bytes::Bytes32(x) => *x,
      Bytes::Array(x)   => pad32r(&x . as_slice()),
      Bytes::Bytes4(x)  => pad32r(x)
    }
  }

  pub fn hex_0x(&self) -> String {
    "0x".to_owned() + &self.data.hex()
  }
}

/* ----------------------------------------------------------------------------
 View structure
-----------------------------------------------------------------------------*/
  ///////////////////////////////////////////////////////////////////////////

  /* heavy, easy, general purpose. Foundational functionality can be
     extracted from Kwl32::util and Bai::con for more performance */

#[derive(Debug)]
pub struct View {
  sig  : Option<Signature>,
  page : Vec<Word>
}

pub enum WithSig { True, False }

impl View {

  pub fn new(call : Calldata, with_sig : WithSig) -> Self {

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

/* ----------------------------------------------------------------------------
View cont..                 common functionality
-----------------------------------------------------------------------------*/
  ///////////////////////////////////////////////////////////////////////////

  /* callers for most cases, to chunk a long call or get a quick summary */

  // returns the 4 byte function signature as a hex str
  pub fn sig(&self) -> String {
    if let Some(x) = &self.sig {
      bytes_to_hex(&x.data.bytes())
    } else { String::from("") }
  }
  // returns the `arguments` portion of the calldata
  pub fn data(&self) -> String {
    if &self.word_count() <= &ONE_WORD {
      self.word(0)
    } else { self.page().join("") }
  }
  // returns the complete calldata, minus `0x` prefix
  pub fn calldata(&self) -> String {
    self.sig() + &self.data()
  }
  // returns the complete calldata with `0x` prefix
  pub fn hex_0x(&self) -> String {
    "0x".to_owned() + &self.sig() + &self.data()
  }
  // returns all 32 byte words as hex str's, in an array
  pub fn page(&self) -> Vec<String> {
    self.words(ZERO_INDEX, self.word_count())
  }
  // returns a single word, will truncate overflow to max len
  pub fn word(&self, index : usize) -> String {
    self.__word(index).hex()
  }
  // returns a range of words, will trunc to max
  pub fn words(&self, start : usize, end : usize) -> Vec<String> {
    self.__words(start, end)
    . iter()
    . map(|x|x.hex())
    . collect::<Vec<String>>()
  }
  // returns the number of word segments in array
  pub fn word_count(&self) -> usize { self.page.len() }
  // quick prints a summary
  pub fn summary(&self) {
    if self.word_count() > SUMMARY_COUNT {
      print!("Sig: {}\nData:\n{}\nView:\n{:?}\nCount: {}\n",
      self.sig(), self.data(), self.page(), self.word_count()) 
    } else {
      print!("Sig: {}\nData:  {}\nView:  {:?}\nCount: {}\n",
      self.sig(), self.data(), self.page(), self.word_count()) 
    }
  }

/* ----------------------------------------------------------------------------
View cont..      destructive functions that mutate state
-----------------------------------------------------------------------------*/
  ///////////////////////////////////////////////////////////////////////////

  // will trunk to 4 bytes, but [0u8;4] if under, since wouldn't be a sig
  pub fn replace_sig_from_bytes(&mut self, bytes : &[u8]) -> () {
    self.sig = Some (Signature::from_bytes(bytes))
  }
  // as above but from string. Signature constructor will do any lifting
  pub fn replace_sig(&mut self, string : &str) -> () {
    self.sig = Some (Signature::from_hex(string))
  }
  // *private* replaces a word, will replace last if pass out of bounds
  fn _replace_word(&mut self, index : usize, bytes : &[u8]) -> () {
    let slice_cap = std::cmp::min(WORD_LEN, bytes.len());
    let capped_id = std::cmp::min(index, &self.word_count() -ZERO_OFFSET);
    let slice     = &bytes[..slice_cap];
    self.page[capped_id] = Word::from_bytes(slice)
  }
  // replace word using a byte array as source
  pub fn replace_from_bytes(&mut self, index : usize, bytes : &[u8]) -> () {
    self._replace_word(index, bytes)
  }
  // replace word, first converting from a hex str as source
  pub fn replace(&mut self, index : usize, string : &str) -> () {
    self._replace_word(index, hex_to_bytes(string).as_slice())
  }
  // append a word using a byte array as source
  pub fn append_from_bytes(&mut self, bytes : &[u8]) -> () {
    self.__append(Word::from_bytes(bytes))
  }
  // append a word using a hex as source
  pub fn append(&mut self, string: &str) -> () {
    self.__append(Word::from_hex(string))
  }
  // append empty word
  pub fn append_empty(&mut self) -> () {
    self.__append(Word::from_bytes(&EMPTY_BYTES32))
  }
  // zero a word - equivalant of &'ing with empty but faster here
  pub fn clear(&mut self, index : usize) -> () {
    self._replace_word(index, &EMPTY_BYTES32)
  }
  // pop the last item, return the result of its hex method
  pub fn pop(&mut self) -> String {
    self.__pop().hex()
  }
  // replace word with left padded equivalent
  pub fn left_pad(&mut self, index : usize) -> () {
    let word = self.__word(index);
    self.replace_from_bytes(index, &pad32l(word.data.bytes()))
  }
  // replace word with left padded equivalent
  pub fn right_pad(&mut self, index : usize) -> (){
    let word = self.__word(index);
    self.replace_from_bytes(index, &pad32r(word.data.bytes()))
  }
  // replace with merge no overlap
  pub fn xor_into(&mut self, index: usize, array : &[u8]) -> () {
    let _a = Word::from_bytes(array); let _b = self.__word(index);
    self.replace_from_bytes(index, &self.__xor_words((&_a, _b)).bytes())
  }
  // replace with difference removed
  pub fn and_into(&mut self, index: usize, array : &[u8]) -> () {
    let _a = Word::from_bytes(array); let _b = self.__word(index);
    self.replace_from_bytes(index, &self.__and_words((&_a, _b)).bytes())
  }
  // replace with merge and overlap
  pub fn or_into(&mut self, index: usize, array : &[u8]) -> () {
    let _a = Word::from_bytes(array); let _b = self.__word(index);
    self.replace_from_bytes(index, &self.__or_words((&_a, _b)).bytes())
  }
  // flip 'em, the bird.. and toggle all the bits
  pub fn not(&mut self, index: usize) -> () {
    let _a = self.__word(index);
    self.replace_from_bytes(index, &self.__not_word(&_a).bytes())
  }
  // *private* perform xor on the 2 tail elements, consume the tail
  fn _fold (&mut self) -> () {
    let buf =  self.__pop(); self.xor_into(self.page.len() - ONE, buf.bytes())
  }
  // public caller for above 
  pub fn xor_fold(&mut self) -> () {
    if self.page.len() < MIN_FOLD { return };
    self._fold()
  }
  // xor fold down to the last element 
  pub fn xor_fold_all(&mut self) -> () { 
    let len = self.page.len(); if len < 2 { return };
    (1..len).for_each(|_|self._fold())
  }
  // wrap the word at index around to the right by the given shift 
  pub fn right_shift(&mut self, index : usize, shift : usize) -> () {
    let word = self.__word(index);
    self.replace_from_bytes(index, self.__right_shift(word, shift).bytes())
  }
  // wrap around the word at index around to the left by the given shift 
  pub fn left_shift(&mut self, index : usize, shift : usize) -> () {
    let word = self.__word(index);
    self.replace_from_bytes(index, self.__left_shift(word, shift).bytes())
  }
  
/* ----------------------------------------------------------------------------
View cont..   exposed functions that take or return Kawala types
-----------------------------------------------------------------------------*/
  ///////////////////////////////////////////////////////////////////////////

  // append a word to the end
  pub fn __append(&mut self, word : Word) -> () {
    self.page.push(word)
  }
  // pop a word from the end
  pub fn __pop(&mut self) -> Word {
    self.page.pop() . unwrap_or(Word::from_bytes(&EMPTY_BYTES32))
  }
  
/* -----------------------*NOTE*: end of destructive ------------------------ */

  // returns a ref to all 32 byte Words
  pub fn __page(&self) -> &[Word] {
    &self.__words(ZERO_INDEX, self.word_count())
  }
  // returns a ref to a single Word, will truncate out of bounds to max len
  pub fn __word(&self, index : usize) -> &Word {
    &self.page[std::cmp::min(index, &self.word_count() -ZERO_OFFSET)]
  }
  // returns a ref to a range of Words, will trunc to max
  pub fn __words(&self, start : usize, end : usize) -> &[Word] {
    &self.page[
      std::cmp::min(start, end)..
      std::cmp::min(end, self.page.len())
    ]
  }
  // roll bytes in word right a given number of shifts
  pub fn __right_shift(&self, word : &Word, shift : usize) -> Word {
    Word::from_bytes(&roll32r(&word.as_bytes32(), shift))
  }
  // roll bytes in word left a given number of shifts
  pub fn __left_shift(&self, word : &Word, shift : usize) -> Word {
    Word::from_bytes(&roll32l(&word.as_bytes32(), shift))
  }
  // xor two words, these and the rest are useful for building byte streams
  pub fn __xor_words(&self, words : (&Word, &Word)) -> Word {
    self._fops(&xor32, (words.0, words.1))
  }
  // return result of a logical and on two 32 byte words
  pub fn __and_words(&self, words : (&Word, &Word)) -> Word {
    self._fops(&and32, (words.0, words.1))
  }
  // return the result of flipping the bits in a single 32 byte word
  pub fn __not_word(&self, word : &Word) -> Word {
    Word::from_bytes(&not32(&word.as_bytes32()))
  }
  // return result of a logical or on two 32 byte words
  pub fn __or_words(&self, words : (&Word, &Word)) -> Word {
    self._fops(&or32, (words.0, words.1))
  }
 // higher order abstraction to save some repetition
  pub fn _fops(
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
const  SUMMARY_COUNT   :    usize         =   2;
const  ONE             :    usize         =   1;
const  TWO             :    usize         =   2;
const  MAX_FOLD        :    usize         =   2;

// marshall through prefixed hex strings
fn marshal_pre(fixed: &str) -> Vec<u8> {
  let ost = &fixed[..2] == "0x"; hex_to_bytes(&fixed[shift(ost)..])
}
// just removes the need for an if to check hex, or multiplication of the bool.
fn shift(b : bool) -> usize { ((b as u8) << (b as u8) << 0) as usize }

/* mod imports */     mod bai; mod kwl32;
use    bai::con::{ bytes_to_hex, hex_to_bytes };
use kwl32::util::{       pad32l, pad32r       };
use kwl32::util::{ xor32, and32, not32, or32  };
use kwl32::util::{      roll32l, roll32r      };
/*
   End of core.
  //////////////////////////////////////////////////////////////////////// */
/* ----------------------------------------------------------------------------
                                                     MIT License 2024 Maka  */
