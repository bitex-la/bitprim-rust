use hash::Hash;
use header::HeaderP;
use std::os::raw::c_int;

opaque_droppable_resource!{
  MerkleBlockT, MerkleBlockP, MerkleBlock {
    iter: u32, default: 0;
  }
  drop: chain_merkle_block_destruct
}

opaque_collection! {
  MerkleBlock, MerkleBlockP,
  Hash, Hash,
  chain_merkle_block_hash_count,
  chain_merkle_block_hash_nth
}

extern { 
  pub fn chain_merkle_block_header(block: MerkleBlockP) -> HeaderP;
  pub fn chain_merkle_block_is_valid(block: MerkleBlockP) -> c_int;
  pub fn chain_merkle_block_serialized_size(block: MerkleBlockP, version: u32) -> u64;
  pub fn chain_merkle_block_total_transaction_count(block: MerkleBlockP) -> u64;
  pub fn chain_merkle_block_reset(block: MerkleBlockP);
}
