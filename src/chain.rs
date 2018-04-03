use std::mem;
use std::os::raw::c_void;
use exit_code::ExitCode;
use errors::*;
use hash::Hash;
use header::HeaderP;

pub enum ChainT {}
pub type ChainP = *mut ChainT;
pub struct Chain(ChainP);

macro_rules! extern_fetcher_and_getter {
  ($fetcher:ident, $getter:ident,
   {$($in:ident: $in_type:ty),*}, {$($out:ident: $out_type:ty),*}
  ) => {
    extern {
      pub fn $fetcher(
        chain: ChainP,
        context: *mut c_void,
        $($in: $in_type,)*
        handler: Option<unsafe extern fn(
          chain: ChainP,
          context: *mut c_void,
          exit_code: ExitCode,
          $($out: $out_type,)*
          )>);
      pub fn $getter(
        chain: ChainP,
        $($in: $in_type,)*
        $($out: *mut $out_type,)*
      ) -> ExitCode;
    }
  }
}

macro_rules! extern_fetchers_and_getters {
  ((($params:tt)),*) => { $(fetcher_and_getter!($params)),* }
}

extern_fetchers_and_getters!{
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
  )
}

extern {
/*
  pub fn chain_subscribe_blockchain(
      exec: executor_t,
      chain: ChainP,
      context: *mut c_void,
      handler: subscribe_blockchain_handler_t,
  );
  pub fn chain_subscribe_transaction(
      exec: executor_t,
      chain: ChainP,
      context: *mut c_void,
      handler: subscribe_transaction_handler_t,
  );
  pub fn chain_unsubscribe(chain: ChainP);
  pub fn chain_organize_block(
      chain: ChainP,
      context: *mut c_void,
      block: block_t,
      handler: result_handler_t,
  );
  pub fn chain_organize_block_sync(chain: ChainP, block: block_t) -> c_int;

  pub fn chain_organize_transaction(
      chain: ChainP,
      context: *mut c_void,
      transaction: transaction_t,
      handler: result_handler_t,
  );
  pub fn chain_organize_transaction_sync(
      chain: ChainP,
      transaction: transaction_t,
  ) -> c_int;
  pub fn hex_to_tx(tx_hex: *const c_char) -> transaction_t;
  pub fn chain_validate_tx(
      chain: ChainP,
      context: *mut c_void,
      tx: transaction_t,
      handler: validate_tx_handler_t,
  );
  pub fn chain_is_stale(chain: ChainP) -> c_int;
  */
}

extern {
  pub fn chain_fetch_spend(
      chain: ChainP,
      context: *mut c_void,
      op: output_point_t,
      handler: Option<unsafe extern fn(
        chain: ChainP,
        context: *mut c_void,
        exit_code: ExitCode,
        input_point: InputPointP)>);
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
