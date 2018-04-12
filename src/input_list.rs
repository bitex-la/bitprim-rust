use input::{InputP, Input};

opaque_resource_mapper!{
  InputListT, InputListP, InputList {}
  async_and_sync {}
  impl {
    pub fn len(&self) -> u64 {
      unsafe{ chain_input_list_count(self.raw) }
    }

    pub fn nth(&self, n: u64) -> Input {
      let raw = unsafe{ chain_input_list_nth(self.raw, n) };
      Input{raw}
    }
  }
  extern { 
    pub fn chain_input_list_construct_default() -> InputListP;
    pub fn chain_input_list_push_back(list: InputListP, input: InputP);
    pub fn chain_input_list_destruct(list: InputListP);
    pub fn chain_input_list_count(list: InputListP) -> u64;
    pub fn chain_input_list_nth(list: InputListP, n: u64) -> InputP;
  }
}

/*
impl Drop for InputList {
  fn drop(&mut self){
    println!("Destruct input list {:?}", self.raw);
    //unsafe{ chain_input_list_destruct(self.raw) }
    println!("Destructed input list {:?}", self.raw);
  }
}
*/
