use block::{BlockP, Block};

opaque_droppable_resource!{
  BlockListT, BlockListP, BlockList {
    iter: u32, default: 0;
  }	
  drop: chain_block_list_destruct
}

opaque_collection! {
  BlockList, BlockListP,
	Block, BlockP,
  chain_block_list_count,
	chain_block_list_nth
}

extern { 
	pub fn chain_block_list_construct_default() -> BlockListP;
	pub fn chain_block_list_push_back(list: BlockListP, block: BlockP);
}
