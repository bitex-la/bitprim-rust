use std::os::raw::{c_char, c_int};
use header::HeaderP;
use hash::Hash;
pub enum BlockT {}
pub type BlockP = *mut BlockT;
pub struct Block(BlockP);

extern "C" {
    pub fn chain_block_construct_default() -> BlockP;
    pub fn chain_block_construct(header: HeaderP, transactions: transaction_list_t) -> BlockP;
    pub fn chain_block_destruct(block: BlockP);
    pub fn chain_block_is_valid(block: BlockP) -> c_int;
    pub fn chain_block_header(block: BlockP) -> HeaderP;
    pub fn chain_block_hash(block: BlockP) -> Hash;
    pub fn chain_block_hash_out(block: BlockP, out_hash: *mut Hash);
    pub fn chain_block_proof(block: BlockP) -> *const c_char;
    pub fn chain_block_transaction_count(block: BlockP) -> u64;
    pub fn chain_block_transaction_nth(block: BlockP, n: u64) -> transaction_t;
    pub fn chain_block_serialized_size(block: BlockP, version: u32) -> u64;
    pub fn chain_block_subsidy(height: u64) -> u64;
    pub fn chain_block_fees(block: BlockP) -> u64;
    pub fn chain_block_claim(block: BlockP) -> u64;
    pub fn chain_block_reward(block: BlockP, height: u64) -> u64;
    pub fn chain_block_generate_merkle_root(block: BlockP) -> Hash;
    pub fn chain_block_generate_merkle_root_out(block: BlockP, out_merkle: *mut Hash);
    pub fn chain_block_signature_operations(block: BlockP) -> u64;
    pub fn chain_block_signature_operations_bip16_active(
        block: BlockP,
        bip16_active: c_int,
    ) -> u64;
    pub fn chain_block_total_inputs(block: BlockP, with_coinbase: c_int) -> u64;
    pub fn chain_block_is_extra_coinbases(block: BlockP) -> c_int;
    pub fn chain_block_is_final(
        block: BlockP,
        height: u64,
        block_time: u32,
    ) -> c_int;
    pub fn chain_block_is_distinct_transaction_set(block: BlockP) -> c_int;
    pub fn chain_block_is_valid_coinbase_claim(
        block: BlockP,
        height: u64,
    ) -> c_int;
    pub fn chain_block_is_valid_coinbase_script(
        block: BlockP,
        height: u64,
    ) -> c_int;
    pub fn chain_block_is_internal_double_spend(block: BlockP) -> c_int;
    pub fn chain_block_is_valid_merkle_root(block: BlockP) -> c_int;
    pub fn chain_block_to_data(
        block: BlockP,
        wire: c_int,
        out_size: *mut u64,
    ) -> *const u8;
}
