use std::os::raw::c_int;
use transaction::{Transaction, TransactionP};
use header::HeaderP;

opaque_droppable_resource!{
  CompactBlockT, CompactBlockP, CompactBlock {
    iter: u32, default: 0;
  }
  drop: compact_block_destruct
}

opaque_collection! {
  CompactBlock, CompactBlockP,
  Transaction, TransactionP,
  compact_block_transaction_count,
  compact_block_transaction_nth
}

extern { 
  pub fn compact_block_header(block: CompactBlockP) -> HeaderP;
  pub fn compact_block_is_valid(block: CompactBlockP) -> c_int;
  pub fn compact_block_serialized_size(block: CompactBlockP, version: u32) -> u64;
  pub fn compact_block_nonce(block: CompactBlockP) -> u64;
  pub fn compact_block_reset(block: CompactBlockP);
}
