use input::{InputP, Input};

opaque_droppable_resource! {
  InputListT, InputListP, InputList {
    iter: u32, default: 0;
  }
  drop: chain_input_list_destruct
}

opaque_collection! {
  InputList, InputListP,
  Input, InputP,
  chain_input_list_count,
  chain_input_list_nth
}

extern { 
  pub fn chain_input_list_construct_default() -> InputListP;
  pub fn chain_input_list_push_back(list: InputListP, input: InputP);
}
