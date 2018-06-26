use input::{Input, InputP};
use destructible::*;
use opaque_collection::*;

opaque_destructible_resource! {
  #[derive(Debug, Clone, PartialEq, Eq)]
  InputListT, InputListP, InputList {}
  chain_input_list_destruct
}

derive_opaque_collection! {
  InputList, InputListP,
  Input, InputP,
  chain_input_list_count,
  chain_input_list_nth
}

impl InputList {
    pub fn construct_default() -> InputList {
        InputList::new(unsafe { chain_input_list_construct_default() })
    }

    pub fn push(&self, input: InputP) {
        unsafe { chain_input_list_push_back(self.raw, input) }
    }
}

extern "C" {
    pub fn chain_input_list_construct_default() -> InputListP;
    pub fn chain_input_list_push_back(list: InputListP, input: InputP);
}
