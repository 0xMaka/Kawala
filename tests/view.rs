#[cfg(test)]
mod view {
  
  use kawala::{ View, Calldata, WithSig, Word };
   #[test]
  fn view() -> (){
    
    /* Create a blank View */
    let view = View::new(Calldata::from_bytes(&[0u8;32]), WithSig::False);
    
    /* Run some sanity checks */
    assert_eq!(view.sig(),  "");
    assert_eq!(view.word_count(),  1);
  }

  #[test]
  fn simple_view() {
    let  sig      = "0x095ea7b3";
    let word1     = "000000000000000000000000000000000022d473030f116ddee9f6b43ac78ba3";
    let word2     = "0000000000000000000000000000000000000000000000000000000000002710";
    let data      = word1.to_owned() + word2;
    let call      = sig.to_owned() + word1 + word2;
    let view      = View::new(Calldata::from_hex(&call), WithSig::True);

    assert_eq!(view.page().len(),  2);

    // can not out of bounds
    assert_eq!(view.word(1),   view.word(100));

    // test things
    assert_eq!(view.hex_0x(),  call);
    assert_eq!(view.sig(),     sig[2..]);
    assert_eq!(view.data(),    data);

    assert_eq!(view.page().len(),  view.words(0,2).len());
    assert_eq!(view.page().len(),  view.word_count());
    assert_eq!(view.word_count(),  2);

    assert_eq!(view.__page().len(),  view.__words(0,2).len());
  }

  #[test]
  fn mutable_view() {
    let  sig      = "0x095ea7b3";
    let word1     = "000000000000000000000000000000000022d473030f116ddee9f6b43ac78ba3";
    let word2     = "0000000000000000000000000000000000000000000000000000000000002710";
    let call      = sig.to_owned() + word1 + word2;
    let mut view  = View::new(Calldata::from_hex(&call), WithSig::True);

    //pop
    assert_eq!(view.page().len(),  2);
    let first  = view.word(0);
    assert_ne!(first,  view.word(100));
    view.pop();
    assert_eq!(first,  view.word(100));
    
    // clear
    assert_ne!(view.__word(0).bytes(),  &[0;32]);
    view.clear(0);
    assert_eq!(view.__word(0).bytes(),  &[0;32]);
    
    // replace sig
    let baseball = "ba5eba11"; 
    view.replace_sig(baseball);
    assert_eq!(view.sig(),  baseball);
    
    // append
    assert_eq!(view.word_count(),  1);
    view.append(word1);
    assert_eq!(view.word_count(),  2);
    assert_eq!(view.word(1),       word1);
    
    // xor fold
    view.xor_fold();
    assert_eq!(view.word_count(),  1);
    assert_eq!(view.word(0),       word1);

    // xor fold all
    assert_eq!(view.word_count(),  1);
    view.append(word1);
    assert_eq!(view.word(1),       word1);
    
    view.clear(0);
    view.xor_fold_all();
    assert_eq!(view.word_count(),  1);
    assert_eq!(view.word(0),       word1);

    // append empty
    view.append_empty();
    view.append_empty();
    view.append_empty();
    view.append_empty();
    view.append_empty();
    view.append_empty(); 
    
    assert_ne!(view.word_count(),  1);

    // clear with and
    view.and_into(0,&[0u8;32].to_vec().as_slice());
    assert_ne!(view.word(0),       word1);

    // replace
    view.replace(0,word1);
    assert_eq!(view.word(0),       word1);
    view.replace(0,word2);
    assert_eq!(view.word(0),       word2);
  }


   #[test]
  fn __remove_word() -> (){
    let mut view = View::new(Calldata::from_bytes(&[[0u8;32],[1u8;32]].concat()), WithSig::False);
    let expected = Word::from_bytes(&[1u8;32]);
    assert_eq!(view.word_count(),  2);
    let input = view.__remove(1);
    assert_eq!(input, expected);
    assert_eq!(view.word_count(),  1);
  }


}

