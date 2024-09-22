/* ----------------------------------------------------------------------------

 @title  : Kawala - Just a Kwl Data Companion (Crafted to a high Kawalaty)
 @author : Maka 
 
// --------------------------------------------------------------------------*/
// Bytes type
// ----------------------------------------------------------------------------
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

// ----------------------------------------------------------------------------
// Calldata structure
// ----------------------------------------------------------------------------
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

// ----------------------------------------------------------------------------
// Signature structure
// ----------------------------------------------------------------------------
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

// ----------------------------------------------------------------------------
// Word structure
// ----------------------------------------------------------------------------
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

// ----------------------------------------------------------------------------
// View structure
// ----------------------------------------------------------------------------
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
    self.page[std::cmp::min(index, &self.word_count() -ZERO_OFFSET)].hex() 
  }
  // returns a range of words, will trunc to max
  fn words(&self, start : usize, end : usize) -> Vec<String> { 
    self.page[
      std::cmp::min(start, end)..
      std::cmp::min(end, &self.page.len() -ZERO_OFFSET)
    ] 
    . iter()
    . map(|x|x.hex())
    . collect::<Vec<String>>()
  }
  // returns the number of word segments in array
  fn word_count(&self) -> usize { self.page.len() }
  // replaces a word on the page, will replace last if pass in overflow
  fn _replace_word(&mut self, index : usize, bytes : &[u8]) -> () {
    let slice_cap = std::cmp::min(WORD_LEN, bytes.len());
    let capped_id = std::cmp::min(index, &self.word_count() -ZERO_OFFSET);
    let slice     = &bytes[..slice_cap];
    self.page[capped_id] = Word::from_bytes(slice)
  }
  // replace word using bytes as source
  fn replace_word_from_bytes(&mut self, index : usize, bytes : &[u8]) -> () {
    self._replace_word(index, bytes)
  }
  // replace word using a hex str as source
  fn replace_word_from_hex(&mut self, index : usize, string: &str) -> () { 
    self._replace_word(index, hex_to_bytes(string).as_slice())
  }
  // quick prints a summary
  fn summary(&self) -> () {
    println!(
      "Signature: {}\nData:\n{}\nView:\n{:?}\nCount: {}",
      self.sig(), self.data(), self.page(), self.word_count()
    );
  }
}

// ----------------------------------------------------------------------------
// Appendix
// ----------------------------------------------------------------------------
static EMPTY_U8_SLICE  :    [u8;0]        =   [0;0]; 
const  SIG_LEN         :    usize         =   4;
const  EMPTY_SIG       :    [u8;4]        =   [0;4]; 
const  WORD_LEN        :    usize         =   32;
const  EMPTY_BYTES32   :    [u8;32]       =   [0;32];

const  ZERO_INDEX      :    usize         =   0;
const  ZERO_OFFSET     :    usize         =   1;
const  ONE_WORD        :    usize         =   1;

fn marshal_pre(fixed: &str) -> Vec<u8> {
  let ost = &fixed[..2] == "0x"; hex_to_bytes(&fixed[shift(ost)..])
}
fn shift(b : bool) -> usize { ((b as u8) << (b as u8) << 0) as usize }

use con::{ bytes_to_hex, hex_to_bytes };

// ----------------------------------------------------------------------------

fn main(){

  println!("Address => ");
  let word      = "000000000000000000000000f27ac2eed7f757038fde1ab1b242fa8f1389a0aa";
  
  let calldata  = Calldata::from_hex(word);
  println!("{:?}", calldata);
  
  let worddata  = Word::from_hex(word);
  println!("{:?}", worddata);
  
  let viewdata  = View::new(calldata, WithSig::False);
  println!("{:?}", viewdata.data());
  
  println!("{:?}", viewdata.prefixed());
  println!("{:-<1$}", "", 100);
   
  println!("View struct =>"); 
  
  let pagedata  = Calldata::from_hex("0x3593564c000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000004a817c80000000000000000000000000000000000000000000000000000000000000000030b000500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000001e000000000000000000000000000000000000000000000000000000000000000400000000000000000000000003fc91a3afd70395cd496c647d5a6cc9d4b2b7fad00000000000000000000000000000000000000000000000000005af3107a400000000000000000000000000000000000000000000000000000000000000001000000000000000000000000008c14ed4f602ac4d2be8ed9c4716307c73e9a83a800000000000000000000000000000000000000000000000000002d79883d2000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002b0d500b1d8e8ef31e21c99d1db9a6444d3adf12700001f42791bca1f2de4661ed88a30c99a7a9449aa8417400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000d500b1d8e8ef31e21c99d1db9a6444d3adf1270000000000000000000000000f27ac2eed7f757038fde1ab1b242fa8f1389a0aa8000000000000000000000000000000000000000000000000000000000000000");
  let page      = View::new(pagedata, WithSig::True);
  println!("{:?}", page);
  println!("{:?}", page.prefixed());
  println!("{:?}", page.data());
  println!("{:?}", page.page());
  println!("{:?}", page.data());
  println!("{:?}", page.word(20));
  println!("{:?}", page.words(0,20));
  
  page.summary();
  
  println!("{:-<1$}", "", 100);
   
  println!("Signature + Calldata struct =>"); 
  let siguint  = "0x2e1a7d4d0000000000000000000000000000000000000000000000000000000000000000";
  println!("{:?}", Signature::from_hex(&siguint[0..10])); // will trunc
  
  let call = Calldata::from_hex(siguint);
  println!("{:?}", call);
  println!("Bytes  => {:?}", call.bytes());
  println!("Hex    => {:?}", call.hex());
  println!("Prefix => {:?}", call.prefixed());
  
}

// ----------------------------------------------------------------------------
// @title  : Bai::con - Basic (as in) ascii (hex) integer (bytes) converter
// @author : Maka

// @note   : embedded style solution.. works on upper and lower nibbles 
//           using a minimal lookup table.
// errors  : put garbage in get garbage out.. if you pass a value that 
//           can't be parsed you can receive malformed data, else error 
//           handling adheres to the unwrap_or_default philosophy.
// ----------------------------------------------------------------------------

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

/* symmetry is performant
  ---------------------------------------------------------------------------
                                  MIT License Copyright (c) 2024 Maka Gucci  */
