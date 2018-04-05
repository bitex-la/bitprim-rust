use std::os::raw::c_int;
use transaction::TransactionP;
use header::HeaderP;

opaque_resource_mapper!{
  CompactBlockT, CompactBlockP, CompactBlock {}
  async_and_sync {}
  impl {}
  extern { 
    pub fn compact_block_header(block: CompactBlockP) -> HeaderP;
    pub fn compact_block_is_valid(block: CompactBlockP) -> c_int;
    pub fn compact_block_serialized_size(block: CompactBlockP, version: u32) -> u64;
    pub fn compact_block_transaction_count(block: CompactBlockP) -> u64;
    pub fn compact_block_transaction_nth(block: CompactBlockP, n: u64) -> TransactionP;
    pub fn compact_block_nonce(block: CompactBlockP) -> u64;
    pub fn compact_block_destruct(block: CompactBlockP);
    pub fn compact_block_reset(block: CompactBlockP);
  }
}
