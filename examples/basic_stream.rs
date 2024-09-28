//-----------------------------------------------------------------------------
use kawala::{ View, Calldata, WithSig };
//-----------------------------------------------------------------------------

fn build_basic_stream() {
  /*

     Basic stream creation, otherwise known as concat with extra steps.
     Obviously when we know our stream and our calls are working, we create
     dedicated functions for this. But when debugging streams for our
     contracts or troubleshooting the composition of streams for others,
     having a visual segmentation while still retaining a granular control 
     can be useful. Kawala `View`s provide an easy interface for this.

  */

  let command1  = "01"; // command byte
  let command2  = "FF"; // command byte
  let address   = "2791Bca1f2de4661ED88A30C99A7a9449Aa84174"; // address
  let amount    = "08E8925E5C2E7DE78EEA"; // value 42069123456789876543210

  // we can start from a single byte view
  let mut view = View::new(Calldata::from_hex(command1), WithSig::False);
  view.summary();
  /* Sig: 
     Data:  01 
     View:  ["01"] 
     Count: 1    */
  // we can append following words
  [command2, amount, address] . map(|x| view.append(x));

  view.summary();
  /* Sig: 
     Data:  01ff08e8925e5c2e7de78eea2791bca1f2de4661ed88a30c99a7a9449aa84174 
     View:  ["01", "ff", "08e8925e5c2e7de78eea", "2791bca1f2de4661ed88a30c99a7a9449aa84174"] 
     Count: 4
     Can see we already have our stream above if we just call the data() method
     but for the sake of keeping it simple while exploring functionality let's 
     build it from the elements */

  // we can pad the words to the desired side..

  // to reflect abi encoding
  view.left_pad(3); 
  //"0000000000000000000000002791bca1f2de4661ed88a30c99a7a9449aa84174"
  
  // or position where we need it
  view.right_pad(2);
  // "08e8925e5c2e7de78eea00000000000000000000000000000000000000000000",
  
  view.right_pad(1);
  // "ff00000000000000000000000000000000000000000000000000000000000000"
  
  view.right_pad(0); 
  // "0100000000000000000000000000000000000000000000000000000000000000"

  // we can then shift our bytes left or right to fine tune their positioning
  view.right_shift(1,1);
  // "00ff000000000000000000000000000000000000000000000000000000000000"
  

  view.right_shift(2,2);
  //"000008e8925e5c2e7de78eea0000000000000000000000000000000000000000"
  
  // we'll go into more uses for the log ops, but for now we can simply 
  // xor_fold_all(), to perform an exclusive or on the 2 tail elements 
  // consuming the last until we are left with a single 32 byte word. 
  view.xor_fold_all();

  view.summary();
  /* Sig: 
     Data:  01ff08e8925e5c2e7de78eea2791bca1f2de4661ed88a30c99a7a9449aa84174 
     View:  ["01ff08e8925e5c2e7de78eea2791bca1f2de4661ed88a30c99a7a9449aa84174"] 
     Count: 1  */
  
  // we have essentially concatonated our strings
  let desired = command1.to_owned() + command2 + amount + address;
  assert_eq!(view.data(), desired.to_lowercase());

}

//-----------------------------------------------------------------------------
fn main () { build_basic_stream(); }
//-----------------------------------------------------------------------------
