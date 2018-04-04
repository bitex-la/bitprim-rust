use std::mem;
use std::os::raw::{c_int, c_void, c_char};
use exit_code::ExitCode;
use errors::*;
use hash::Hash;
use header::HeaderP;
use block::BlockP;
use merkle_block::MerkleBlockP;
use compact_block::CompactBlockP;
use history_compact_list::HistoryCompactListP;
use payment_address::PaymentAddressP;
use binary::BinaryP;
use transaction::TransactionP;
use executor::ExecutorP;
use stealth_compact_list::StealthCompactListP;
use block_list::BlockListP;
use output_point::OutputPointP;
use input_point::InputPointP;

pub enum ChainT {}
pub type ChainP = *mut ChainT;
pub struct Chain(ChainP);

extern_asyncs_and_syncs!{
  ChainP,
  ( chain_fetch_last_height,
    chain_get_last_height,
    {},
    {height: u64}
  ),
  ( chain_fetch_block_height,
    chain_get_block_height,
    {hash: Hash},
    {height: u64}
  ),
  ( chain_fetch_history,
    chain_get_history,
    {address: PaymentAddressP, limit: u64, from_heigth: u64},
    {history: HistoryCompactListP}
  ),
  ( chain_fetch_block_header_by_height,
    chain_get_block_header_by_height,
    {height: u64},
    {header: HeaderP, height: u64}
  ),
  ( chain_fetch_block_header_by_hash,
    chain_get_block_header_by_hash,
    {hash: Hash},
    {header: HeaderP, height: u64}
  ),
  ( chain_fetch_block_by_height, 
    chain_get_block_by_height,
    {height: u64},
    {block: BlockP, height: u64}
  ),
  ( chain_fetch_block_by_hash,
    chain_get_block_by_hash,
    {hash: Hash},
    {block: BlockP, height: u64}
  ),
  ( chain_fetch_merkle_block_by_height,
    chain_get_merkle_block_by_height,
    {height: u64},
    {block: MerkleBlockP, height: u64}
  ),
  ( chain_fetch_merkle_block_by_hash,
    chain_get_merkle_block_by_hash,
    {hash: Hash},
    {block: MerkleBlockP, height: u64}
  ),
  ( chain_fetch_compact_block_by_height,
    chain_get_compact_block_by_height,
    {height: u64},
    {block: CompactBlockP, height: u64}
  ),
  ( chain_fetch_compact_block_by_hash,
    chain_get_compact_block_by_hash,
    {hash: Hash},
    {block: CompactBlockP, height: u64}
  ),
  ( chain_fetch_transaction,
    chain_get_transaction,
    {hash: Hash, require_confirmed: c_int},
    {transaction: TransactionP, height: u64, index: u64}
  ),
  ( chain_fetch_transaction_position,
    chain_get_transaction_position,
    {hash: Hash, required_confirmed: c_int},
    {position: u64, height: u64}
  ),
  ( chain_fetch_stealth,
    chain_get_stealth,
    {filter: BinaryP, from_height: u64},
    {out_list: StealthCompactListP}
  ),
  ( chain_organize_block,
    chain_organize_block_sync,
    {block: BlockP},
    {}
  ),
  ( chain_organize_transaction,
    chain_organize_transaction_sync,
    {transaction: TransactionP},
    {}
  )
}

extern_async!{
  ChainP,
  chain_validate_tx,
  {tx: TransactionP},
  {something: *const c_char}
}

extern_async!{
  ChainP,
  chain_fetch_spend,
  {output_point: OutputPointP},
  {input_point: InputPointP}
}

extern {
  pub fn hex_to_tx(tx_hex: *const c_char) -> TransactionP;
  pub fn chain_is_stale(chain: ChainP) -> c_int;
  pub fn chain_unsubscribe(chain: ChainP);
  pub fn chain_subscribe_blockchain(
      exec: ExecutorP,
      chain: ChainP,
      context: *mut c_void,
      handler: Option< unsafe extern fn(
        exec: ExecutorP,
        chain: ChainP,
        context: *mut c_void,
        exit_code: ExitCode,
        height: u64,
        blocks_a: BlockListP,
        blocks_b: BlockListP)
        -> c_int >
  );
  pub fn chain_subscribe_transaction(
      exec: ExecutorP,
      chain: ChainP,
      context: *mut c_void,
      handler: Option< unsafe extern fn(
        exec: ExecutorP,
        chain: ChainP,
        context: *mut c_void,
        exit_code: ExitCode,
        transaction: TransactionP
        ) -> c_int >
  );
}

impl Chain {
	pub fn new(chain_p: ChainP) -> Chain { Chain(chain_p) }
  
  pub fn get_last_height(&self) -> Result<u64> {
	  let mut height = unsafe{ mem::uninitialized() };
    match unsafe{ chain_get_last_height(self.0, &mut height) } {
			ExitCode::Success => Ok(height),
			result => bail!(ErrorKind::ErrorExitCode(result))
		}
	}
}
