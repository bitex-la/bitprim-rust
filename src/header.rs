use std::os::raw::c_int;
use hash::Hash;

opaque_resource_mapper!{
  HeaderT, HeaderP, Header {}
  async_and_sync {}
  impl {}

  extern {
      pub fn chain_header_factory_from_data(version: u32, data: *mut u8, n: u64) -> HeaderP;
      pub fn chain_header_satoshi_fixed_size(version: u32) -> u64;
      pub fn chain_header_reset(header: HeaderP);
      pub fn chain_header_serialized_size(header: HeaderP, version: u32) -> u64;
      pub fn chain_header_to_data(header: HeaderP, version: u32, out_size: *mut u64) -> *const u8;
      pub fn chain_header_construct_default() -> HeaderP;
      pub fn chain_header_construct(
          version: u32,
          previous_block_hash: *mut u8,
          merkle: *mut u8,
          timestamp: u32,
          bits: u32,
          nonce: u32,
      ) -> HeaderP;
      pub fn chain_header_destruct(header: HeaderP);
      pub fn chain_header_is_valid(header: HeaderP) -> c_int;
      pub fn chain_header_version(header: HeaderP) -> u32;
      pub fn chain_header_set_version(header: HeaderP, version: u32);
      pub fn chain_header_timestamp(header: HeaderP) -> u32;
      pub fn chain_header_set_timestamp(header: HeaderP, timestamp: u32);
      pub fn chain_header_bits(header: HeaderP) -> u32;
      pub fn chain_header_set_bits(header: HeaderP, bits: u32);
      pub fn chain_header_nonce(header: HeaderP) -> u32;
      pub fn chain_header_set_nonce(header: HeaderP, nonce: u32);
      pub fn chain_header_previous_block_hash(header: HeaderP) -> Hash;
      pub fn chain_header_previous_block_hash_out(
          header: HeaderP,
          out_previous_block_hash: *mut Hash,
      );
      pub fn chain_header_merkle(header: HeaderP) -> Hash;
      pub fn chain_header_merkle_out(header: HeaderP, out_merkle: *mut Hash);
      pub fn chain_header_hash(header: HeaderP) -> Hash;
      pub fn chain_header_hash_out(header: HeaderP, out_hash: *mut Hash);
  }
}
