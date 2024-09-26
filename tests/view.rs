

#[cfg(test)]
mod tests {
  
  use kawala::{ View, Calldata, WithSig, DataTrait };

  #[test]
  fn trial_view() -> (){
    
    /* Create a blank View */
    let view = View::new(Calldata::from_bytes(&[0u8;32]), WithSig::False);
    
    /* Run some sanity checks */
    assert_eq!(view.sig(), "");
    assert_eq!(view.word_count(), 1);
  }

  #[test]
  fn place_holder() {
    /*
       upload test suite 
    */
  }


}

