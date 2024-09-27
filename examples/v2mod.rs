//-------------------------------------------------------------------------------------------------
use kawala::{ View, Calldata, WithSig };
//-------------------------------------------------------------------------------------------------

/*  Foreword:
  Another example that could do without the overhead, though kept simple should make clear some 
  potential work flows. 
  Imagine, that instead of a v2 swap it's a swap for the new cross chain layer zero colab, and 
  while many of the contracts aren't verified yet, you can bet once they are that your usual 
  tooling and explorers won't decode the arrays of bytes intended for their destination chains.

  Inexperienced users could pile in to test the new product that devs are overworked on delivering. 
  In many cases being mistaken or maybe even dishonest about the slippage, fees, or amounts they 
  agreed to.

  At times like that the calldata holds the answers, so you find yourself steppin through it with a 
  fine comb. Back then I wrote a lamda to quickly chunk a call. 

  ```
  chunk = lambda l,s : list(map(lambda x : l[x:x+s], range(0,len(l),s)))
  ```

  It came in clutch, but would still find myself grabbing the wrong chunk or being off by one during 
  chaos, and there's nothing worse than feeding a sim with bad input and going on a goose chase over 
  the result. 
  
  Kawala' `View`s gives a structured and low level, but still relatively intuitive way to work on a 
  call, without any of the dependancy on a given chain, encoding, or list of imports.            */

// ------------------------------------------------------------------------------------------------
// EXAMPLE:
// ------------------------------------------------------------------------------------------------

 /* In this scenario we have a v2 swap that won't go through, and we are going to run a 
    simulation using tenderly so just want to change a couple of values and grab the hex.        */
  
  const AMOUNT_MIN_SLOT : usize = 1;
  const TIME_LIMIT_SLOT : usize = 4;
  const DISTANT_FUTURE  : &str  = "0x04A817C800"; // 20000000000

 /* Could be they have not accounted for tax on transfer, could be some vesting mechanic.
    Good test is trying the sim with `amountOutMin` at zero just to see if it should go 
    through at all, or if there is something more sinister (perhaps just more inept) to it.      */
  fn zero_amount_min(view: &mut View) -> () {
    view.clear(AMOUNT_MIN_SLOT);    // we can `clear()` to replace the word with a [0u8;32]
  }

 /* To be sure we don't smugly rush back proclaiming to have found the simple mistake.           */
  fn replace_deadline(view : &mut View) -> () {
    view.replace(TIME_LIMIT_SLOT, DISTANT_FUTURE);
    view.left_pad(TIME_LIMIT_SLOT);
  }
  
 /* To grab the before and after, we can use view.word(index) to pull a single word as a String  */
  fn print(view : &View, id : usize) {
    println!("[+] => {}", view.word(id));
  }

 /* Alternatively, we could use the built in `view.summary()`, for a crude but quick breakdown.    
  fn quick_sum(view : &View) {
    view.summary();               // prints an overview of the calldata
  } */

 /* Kawala stores and works on raw bytes, only using hex for pretty printing. By default the hex
    is not prefixed but can be included via a getter in line with the standard.                  */
  fn hex_0x(view : &View) {
    println!("[+] => {}", view.hex_0x());
  }

//-------------------------------------------------------------------------------------------------

fn main() {
    let string     = "0x791ac94700000000000000000000000000000000000000000000000000000000004c3f88000000000000000000000000000000000000000000000000000ac2d7237640f900000000000000000000000000000000000000000000000000000000000000a000000000000000000000000013a48c3e0a403b6cf1a59fbd600e284e620b37ed0000000000000000000000000000000000000000000000000000000065211d050000000000000000000000000000000000000000000000000000000000000002000000000000000000000000ff970a61a04b1ca14834a43f5de4533ebddb5cc800000000000000000000000082af49447d8a07e3bd95bd0d56f35241523fbab1";
    let mut view   = View::new(Calldata::from_hex(string), WithSig::True);
    
    print!("[>] Before:\n"); 
    print(&view, AMOUNT_MIN_SLOT);
    print(&view, TIME_LIMIT_SLOT);  
    
    zero_amount_min(&mut view);
    replace_deadline(&mut view);
    print!("[>] After:\n");
    print(&view, AMOUNT_MIN_SLOT);
    print(&view, TIME_LIMIT_SLOT);  
    print!("[>] Hex 0x:\n");
    hex_0x(&view);
    //quick_sum(&view);
  }
//-------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------
