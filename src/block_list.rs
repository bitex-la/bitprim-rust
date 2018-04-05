use block::BlockP;

opaque_resource_mapper!{
  BlockListT, BlockListP, BlockList {}
  async_and_sync {}
  impl {}
  extern { 
    pub fn chain_block_list_construct_default() -> BlockListP;
    pub fn chain_block_list_push_back(list: BlockListP, block: BlockP);
    pub fn chain_block_list_destruct(list: BlockListP);
    pub fn chain_block_list_count(list: BlockListP) -> u64;
    pub fn chain_block_list_nth(list: BlockListP, n: u64) -> BlockP;
  }
}
