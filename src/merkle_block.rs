use hash::Hash;
use header::HeaderP;
use std::os::raw::c_int;

opaque_resource_mapper!{
  MerkleBlockT, MerkleBlockP, MerkleBlock {}
  async_and_sync {}
  impl {}
  extern { 
    pub fn chain_merkle_block_hash_nth(block: MerkleBlockP, n: u64) -> Hash;
    pub fn chain_merkle_block_hash_nth_out(block: MerkleBlockP, n: u64, out_hash: *mut Hash);
    pub fn chain_merkle_block_header(block: MerkleBlockP) -> HeaderP;
    pub fn chain_merkle_block_is_valid(block: MerkleBlockP) -> c_int;
    pub fn chain_merkle_block_hash_count(block: MerkleBlockP) -> u64;
    pub fn chain_merkle_block_serialized_size(block: MerkleBlockP, version: u32) -> u64;
    pub fn chain_MerkleBlockPotal_transaction_count(block: MerkleBlockP) -> u64;
    pub fn chain_merkle_block_destruct(block: MerkleBlockP);
    pub fn chain_merkle_block_reset(block: MerkleBlockP);
  }
}
