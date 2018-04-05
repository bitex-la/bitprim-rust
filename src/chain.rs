use std::mem;
use std::os::raw::{c_int, c_void, c_char};
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
use block_list::*;
use output_point::*;
use input_point::*;

opaque_resource_mapper!{
  ChainT, ChainP, Chain {
    exec: Executor
  }

  async_and_sync {
    { chain_fetch_last_height: fetch_last_height,
      chain_get_last_height: get_last_height,
      in: [],
      out: [(height, u64, u64, height)]
    },
    { chain_fetch_block_by_height: fetch_block_by_height,
      chain_get_block_by_height: get_block_by_height,
      in: [(height, u64)],
      out: [
        (block, BlockP, Block, Block::new(block)),
        (new_height, u64, u64, new_height)
      ]
    },
    { chain_fetch_block_height: fetch_block_height,
      chain_get_block_height: get_block_height,
      in: [(hash, Hash)],
      out: [(height, u64, u64, height)]
    },
    { chain_fetch_history: fetch_history,
      chain_get_history: get_history,
      in: [
        (address, PaymentAddressP),
        (limit, u64),
        (from_height, u64)
      ],
      out: [
        ( history, HistoryCompactListP, HistoryCompactList,
          HistoryCompactList::new(history))
      ]
    },
    { chain_fetch_block_header_by_height: fetch_block_header_by_height,
      chain_get_block_header_by_height: get_block_header_by_height,
      in: [(height, u64)],
      out: [
        (header, HeaderP, Header, Header::new(header)),
        (new_height, u64, u64, new_height)
      ]
    },
    { chain_fetch_block_header_by_hash: fetch_block_header_by_hash,
      chain_get_block_header_by_hash: get_block_header_by_hash,
      in: [(hash, Hash)],
      out: [
        (header, HeaderP, Header, Header::new(header)),
        (height, u64, u64, height)
      ]
    },
    { chain_fetch_block_by_hash: fetch_block_by_hash,
      chain_get_block_by_hash: get_block_by_hash,
      in: [(hash, Hash)],
      out: [
        (block, BlockP, Block, Block::new(block)),
        (height, u64, u64, height)
      ]
    },
    { chain_fetch_merkle_block_by_height: fetch_merkle_block_by_height,
      chain_get_merkle_block_by_height: get_merkle_block_by_height,
      in: [(height, u64)],
      out: [
        (block, MerkleBlockP, MerkleBlock, MerkleBlock::new(block)),
        (new_height, u64, u64, new_height)
      ]
    },
    { chain_fetch_merkle_block_by_hash: fetch_merkle_block_by_hash,
      chain_get_merkle_block_by_hash: get_merkle_block_by_hash,
      in: [(hash, Hash)],
      out: [
        (block, MerkleBlockP, MerkleBlock, MerkleBlock::new(block)),
        (height, u64, u64, height)
      ]
    },
    { chain_fetch_compact_block_by_height: fetch_compact_block_by_height,
      chain_get_compact_block_by_height: get_compact_block_by_height,
      in: [(height, u64)],
      out: [
        (block, CompactBlockP, CompactBlock, CompactBlock::new(block)),
        (new_height, u64, u64, new_height)
      ]
    },
    { chain_fetch_compact_block_by_hash: fetch_compact_block_by_hash,
      chain_get_compact_block_by_hash: get_compact_block_by_hash,
      in: [(hash, Hash)],
      out: [
        (block, CompactBlockP, CompactBlock, CompactBlock::new(block)),
        (height, u64, u64, height)
      ]
    },
    { chain_fetch_transaction: fetch_transaction,
      chain_get_transaction: get_transaction,
      in: [(hash, Hash), (require_confirmed, c_int)],
      out: [
        (transaction, TransactionP, Transaction, Transaction::new(transaction)),
        (height, u64, u64, height)
      ]
    },
    { chain_fetch_transaction_position: fetch_transaction_position,
      chain_get_transaction_position: get_transaction_position,
      in: [(hash, Hash), (require_confirmed, c_int)],
      out: [
        (position, u64, u64, position),
        (height, u64, u64, height)
      ]
    },
    { chain_fetch_stealth: fetch_stealth,
      chain_get_stealth: get_stealth,
      in: [(filter, BinaryP), (from_height, u64)],
      out: [
        ( out_list,
          StealthCompactListP,
          StealthCompactList,
          StealthCompactList::new(out_list))
      ]
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

  impl { }

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
}

extern_async!{
  ChainP,
  chain_validate_tx,
  [(tx, TransactionP)],
  [(something, *const c_char, *const c_char, something)]
}

extern_async!{
  ChainP,
  chain_fetch_spend,
  [(output_point, OutputPointP)],
  [(input_point, InputPointP, InputPoint, InputPoint::new(input_point))]
}

