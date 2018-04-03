use block::BlockP;
pub enum BlockListT {}
pub type BlockListP = *mut BlockListT;
pub struct BlockList(BlockListP);

extern "C" {
    pub fn chain_block_list_construct_default() -> BlockListP;
    pub fn chain_block_list_push_back(list: BlockListP, block: BlockP);
    pub fn chain_block_list_destruct(list: BlockListP);
    pub fn chain_block_list_count(list: BlockListP) -> u64;
    pub fn chain_block_list_nth(list: BlockListP, n: u64) -> BlockP;
}


