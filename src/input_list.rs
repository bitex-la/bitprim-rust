use input::InputP;

opaque_resource_mapper!{
  InputListT, InputListP, InputList {}
  async_and_sync {}
  impl {}
  extern { 
    pub fn chain_input_list_construct_default() -> InputListP;
    pub fn chain_input_list_push_back(list: InputListP, input: InputP);
    pub fn chain_input_list_destruct(list: InputListP);
    pub fn chain_input_list_count(list: InputListP) -> u64;
    pub fn chain_input_list_nth(list: InputListP, n: u64) -> InputP;
  }
}
