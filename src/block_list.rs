use block::{BlockP, Block};
use destructible::*;
use opaque_collection::*;

opaque_destructible_resource!{
  BlockListT, BlockListP, BlockList {}	
  chain_block_list_destruct
}

derive_opaque_collection! {
  BlockList, BlockListP,
	Block, BlockP,
  chain_block_list_count,
	chain_block_list_nth
}

extern { 
	pub fn chain_block_list_construct_default() -> BlockListP;
	pub fn chain_block_list_push_back(list: BlockListP, block: BlockP);
}
