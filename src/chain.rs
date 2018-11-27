use std::mem;
use std::os::raw::{c_int, c_void};
use std::sync::mpsc::channel;
use exit_code::ExitCode;
use errors::*;
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
use input_point::*;
use output_point::*;
use destructible::*;

opaque_resource!{
  #[derive(Clone)]
  ChainT, ChainP, Chain {}
  pub_fields: {
    exec: Executor
  }
}

async_and_sync_methods! {
  Chain, ChainP,
  { chain_fetch_last_height: fetch_last_height,
    chain_get_last_height: get_last_height,
    in: [],
    out: [(height, u64)]
  },
  { chain_fetch_block_by_height: fetch_block_by_height,
    chain_get_block_by_height: get_block_by_height,
    in: [(height, u64)],
    out: [ (block, BlockP, Block, managed), (new_height, u64) ]
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
    out: [ ( history, HistoryCompactListP, HistoryCompactList, managed) ]
  },
  { chain_fetch_block_header_by_height: fetch_block_header_by_height,
    chain_get_block_header_by_height: get_block_header_by_height,
    in: [(height, u64)],
    out: [ (header, HeaderP, Header, managed), (new_height, u64) ]
  },
  { chain_fetch_block_header_by_hash: fetch_block_header_by_hash,
    chain_get_block_header_by_hash: get_block_header_by_hash,
    in: [(hash, Hash)],
    out: [ (header, HeaderP, Header, managed), (height, u64) ]
  },
  { chain_fetch_block_by_hash: fetch_block_by_hash,
    chain_get_block_by_hash: get_block_by_hash,
    in: [(hash, Hash)],
    out: [ (block, BlockP, Block, managed), (height, u64) ]
  },
  { chain_fetch_merkle_block_by_height: fetch_merkle_block_by_height,
    chain_get_merkle_block_by_height: get_merkle_block_by_height,
    in: [(height, u64)],
    out: [ (block, MerkleBlockP, MerkleBlock, managed), (new_height, u64) ]
  },
  { chain_fetch_merkle_block_by_hash: fetch_merkle_block_by_hash,
    chain_get_merkle_block_by_hash: get_merkle_block_by_hash,
    in: [(hash, Hash)],
    out: [ (block, MerkleBlockP, MerkleBlock, managed), (height, u64) ]
  },
  { chain_fetch_compact_block_by_height: fetch_compact_block_by_height,
    chain_get_compact_block_by_height: get_compact_block_by_height,
    in: [(height, u64)],
    out: [ (block, CompactBlockP, CompactBlock, managed), (new_height, u64) ]
  },
  { chain_fetch_compact_block_by_hash: fetch_compact_block_by_hash,
    chain_get_compact_block_by_hash: get_compact_block_by_hash,
    in: [(hash, Hash)],
    out: [ (block, CompactBlockP, CompactBlock, managed), (height, u64) ]
  },
  { chain_fetch_transaction: fetch_transaction,
    chain_get_transaction: get_transaction,
    in: [(hash, Hash), (require_confirmed, c_int)],
    out: [ (transaction, TransactionP, Transaction, managed), (height, u64), (index, u64) ]
  },
  { chain_fetch_transaction_position: fetch_transaction_position,
    chain_get_transaction_position: get_transaction_position,
    in: [(hash, Hash), (require_confirmed, c_int)],
    out: [ (position, u64), (height, u64) ]
  },
  { chain_fetch_stealth: fetch_stealth,
    chain_get_stealth: get_stealth,
    in: [(filter, BinaryP), (from_height, u64)],
    out: [ (out_list, StealthCompactListP, StealthCompactList, managed) ]
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

async_methods! {
  Chain,
  { chain_fetch_spend: fetch_spend,
    self: {
      outer: (this, Chain),
      inner: (this_raw, ChainP, Chain{raw: this_raw, ..this})
    },
    in: [(output_point, OutputPointP, OutputPoint)],
    out: [(input_point, InputPointP, InputPoint)]
  }
}

impl Chain {
    pub fn is_stale(&self) -> bool {
        (unsafe { chain_is_stale(self.raw) }) != 0
    }

    pub fn is_spent(&self, output_point: OutputPoint) -> bool {
        let (writex, readex) = channel();
        self.fetch_spend(output_point, |_, error, _| {
            writex.send(error != ExitCode::NotFound).unwrap();
        });
        readex.recv().unwrap()
    }

    pub fn broadcast(&self, raw_hash: &str) -> Hash {
        let transaction = Transaction::from_data(raw_hash);
        let (writex, readex) = channel();
        self.organize_transaction(transaction.raw, |_chain, error| {
            writex.send(error != ExitCode::NotFound).unwrap();
        });
        readex.recv().unwrap();
        transaction.hash()
    }
}

extern "C" {
    pub fn chain_is_stale(chain: ChainP) -> c_int;
}
