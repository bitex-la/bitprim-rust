use output::{OutputP, Output};

opaque_droppable_resource!{
  OutputListT, OutputListP, OutputList {
    iter: u32, default: 0;
  }
  drop: chain_output_list_destruct
}

opaque_collection! {
  OutputList, OutputListP,
  Output, OutputP,
  chain_output_list_count,
  chain_output_list_nth
}

extern { 
  pub fn chain_output_list_construct_default() -> OutputListP;
  pub fn chain_output_list_push_back(list: OutputListP, output: OutputP);
}
