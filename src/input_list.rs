use input::{InputP, Input};
use destructible::*;
use opaque_collection::*;

opaque_destructible_resource! {
  InputListT, InputListP, InputList {}
  chain_input_list_destruct
}

derive_opaque_collection! {
  InputList, InputListP,
  Input, InputP,
  chain_input_list_count,
  chain_input_list_nth
}

extern { 
  pub fn chain_input_list_construct_default() -> InputListP;
  pub fn chain_input_list_push_back(list: InputListP, input: InputP);
}
