use input::{InputP, Input};

opaque_droppable_resource! {
  InputListT, InputListP, InputList {
    iter: u32, default: 0;
  }
  drop: chain_input_list_destruct
}

opaque_collection_destructible_inherit! {
  InputList, InputListP,
  Input, InputP,
  chain_input_list_count,
  chain_input_list_nth
}

impl InputList {
  pub fn destructible(raw: InputListP, destruct_on_drop: bool) -> InputList {
    InputList{raw, destruct_on_drop, iter: 0}
  }
}

extern { 
  pub fn chain_input_list_construct_default() -> InputListP;
  pub fn chain_input_list_push_back(list: InputListP, input: InputP);
}
