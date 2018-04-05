use output::OutputP;

opaque_resource_mapper!{
  OutputListT, OutputListP, OutputList {}
  async_and_sync {}
  impl {}
  extern { 
    pub fn chain_output_list_construct_default() -> OutputListP;
    pub fn chain_output_list_push_back(list: OutputListP, output: OutputP);
    pub fn chain_output_list_destruct(list: OutputListP);
    pub fn chain_output_list_count(list: OutputListP) -> u64;
    pub fn chain_output_list_nth(list: OutputListP, n: u64) -> OutputP;
  }
}
