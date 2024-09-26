use kawala;
use kawala::{ View, Calldata, WithSig, DataTrait };

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
  println!("{:?}",view.summary());

  // we can append following words
  [command2, amount, address] . map(|x| view.append(x));
  println!("{:?}",view.summary());

  // we can pad the words to the desired side
  view.right_pad(0);
  view.right_pad(1);
  view.right_pad(2);
  view.left_pad(3);

  // and even shift our bytes left or right to fine tune their position
  view.right_shift(1,1);
  view.right_shift(2,2);

  // we'll go into more uses for the log ops, but for now we use a simple 
  // xor_fold_all(), that will perform xor on the tail elements consuming 
  // the last element untill we are left with one. 
  view.xor_fold_all();
  println!("{:?}",view.summary());
  
  // the result is the same as having concatonated our stream as a string
  let desired = command1.to_owned() + command2 + amount + address;
  assert_eq!(view.data(), desired.to_lowercase());
}

fn main () { 

  build_basic_stream();

}
