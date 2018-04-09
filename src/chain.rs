use std::mem;
use std;
use std::os::raw::{c_int, c_void, c_char};
use std::ffi::CString;
use exit_code::ExitCode;
use std::sync::{Arc,Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use errors::*;
use std::{thread, time};
use hash::Hash;
use header::*;
use block::*;
use merkle_block::*;
use compact_block::*;
use history_compact_list::*;
use payment_address::*;
use binary::*;
use transaction::*;
use executor::*;
use stealth_compact_list::*;
use block_list::*;
use output_point::*;
use input_point::*;

opaque_resource_mapper!{
  #[derive(Clone)]
  ChainT, ChainP, Chain {
    exec: Executor
  }

  async_and_sync {
    { chain_fetch_last_height: fetch_last_height,
      chain_get_last_height: get_last_height,
      in: [],
      out: [(height, u64)]
    },
    { chain_fetch_block_by_height: fetch_block_by_height,
      chain_get_block_by_height: get_block_by_height,
      in: [(height, u64)],
      out: [ (block, BlockP, Block), (new_height, u64) ]
    },
    { chain_fetch_block_height: fetch_block_height,
      chain_get_block_height: get_block_height,
      in: [(hash, Hash)],
      out: [(height, u64)]
    },
    { chain_fetch_history: fetch_history,
      chain_get_history: get_history,
      in: [
        (address, PaymentAddressP, PaymentAddress),
        (limit, u64),
        (from_height, u64)
      ],
      out: [ ( history, HistoryCompactListP, HistoryCompactList ) ]
    },
    { chain_fetch_block_header_by_height: fetch_block_header_by_height,
      chain_get_block_header_by_height: get_block_header_by_height,
      in: [(height, u64)],
      out: [ (header, HeaderP, Header ), (new_height, u64) ]
    },
    { chain_fetch_block_header_by_hash: fetch_block_header_by_hash,
      chain_get_block_header_by_hash: get_block_header_by_hash,
      in: [(hash, Hash)],
      out: [ (header, HeaderP, Header), (height, u64) ]
    },
    { chain_fetch_block_by_hash: fetch_block_by_hash,
      chain_get_block_by_hash: get_block_by_hash,
      in: [(hash, Hash)],
      out: [ (block, BlockP, Block), (height, u64) ]
    },
    { chain_fetch_merkle_block_by_height: fetch_merkle_block_by_height,
      chain_get_merkle_block_by_height: get_merkle_block_by_height,
      in: [(height, u64)],
      out: [ (block, MerkleBlockP, MerkleBlock), (new_height, u64) ]
    },
    { chain_fetch_merkle_block_by_hash: fetch_merkle_block_by_hash,
      chain_get_merkle_block_by_hash: get_merkle_block_by_hash,
      in: [(hash, Hash)],
      out: [ (block, MerkleBlockP, MerkleBlock), (height, u64) ]
    },
    { chain_fetch_compact_block_by_height: fetch_compact_block_by_height,
      chain_get_compact_block_by_height: get_compact_block_by_height,
      in: [(height, u64)],
      out: [ (block, CompactBlockP, CompactBlock), (new_height, u64) ]
    },
    { chain_fetch_compact_block_by_hash: fetch_compact_block_by_hash,
      chain_get_compact_block_by_hash: get_compact_block_by_hash,
      in: [(hash, Hash)],
      out: [ (block, CompactBlockP, CompactBlock), (height, u64) ]
    },
    { chain_fetch_transaction: fetch_transaction,
      chain_get_transaction: get_transaction,
      in: [(hash, Hash), (require_confirmed, c_int)],
      out: [ (transaction, TransactionP, Transaction), (height, u64) ]
    },
    { chain_fetch_transaction_position: fetch_transaction_position,
      chain_get_transaction_position: get_transaction_position,
      in: [(hash, Hash), (require_confirmed, c_int)],
      out: [ (position, u64), (height, u64) ]
    },
    { chain_fetch_stealth: fetch_stealth,
      chain_get_stealth: get_stealth,
      in: [(filter, BinaryP), (from_height, u64)],
      out: [ (out_list, StealthCompactListP, StealthCompactList) ]
    },
    { chain_organize_block: organize_block,
      chain_organize_block_sync: organize_block_sync,
      in: [(block, BlockP)],
      out: []
    },
    { chain_organize_transaction: organize_transaction,
      chain_organize_transaction_sync: organize_transaction_sync,
      in: [(transaction, TransactionP)],
      out: []
    }
  }

  impl {
    pub fn is_stale(&self) -> bool {
      (unsafe{ chain_is_stale(self.raw) }) != 0
    }

    pub fn explore_address_history(a, b, c)
  }

  extern {
    pub fn chain_is_stale(chain: ChainP) -> c_int;
  }
}

/*
extern_async!{
  ChainP,
  chain_validate_tx,
  [(tx, TransactionP)],
  [(something, *const c_char)]
}

extern_async!{
  ChainP,
  chain_fetch_spend,
  [(output_point, OutputPointP)],
  [(input_point, InputPointP)]
}
*/
