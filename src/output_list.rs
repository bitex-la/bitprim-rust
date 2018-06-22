use output::{Output, OutputP};
use destructible::*;
use opaque_collection::*;

opaque_destructible_resource!{
  #[derive(Debug, Clone, PartialEq, Eq)]
  OutputListT, OutputListP, OutputList {}
  chain_output_list_destruct
}

derive_opaque_collection! {
  OutputList, OutputListP,
  Output, OutputP,
  chain_output_list_count,
  chain_output_list_nth
}

impl OutputList {
    pub fn construct_default() -> OutputList {
        OutputList::new(unsafe { chain_output_list_construct_default() })
    }

    pub fn push(&self, output: OutputP) {
        unsafe { chain_output_list_push_back(self.raw, output) }
    }
}
extern "C" {
    pub fn chain_output_list_construct_default() -> OutputListP;
    pub fn chain_output_list_push_back(list: OutputListP, output: OutputP);
}
