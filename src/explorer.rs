struct Explorer {
  exec: Executor
}

impl Explorer {
  pub fn history_for(address: String) -> Address {
    /* Full history for address, with transactions by block */
  }

  pub fn incoming_for(address: String) -> Vec<Input> {
    /* Just incoming, and enough information to serialize them */
  }

  pub fn utxos_for(address: String) -> Vec<Utxo> {
    /* Just incoming, and enough information to serialize them */
  }
}

struct TransactionHash {
  hash: String
}

struct TransactionHash {
  transaction_hash: TransactionHash,
  inputs: Vec<Input>,
  outputs: Vec<Output>
}

struct Input {
  address: PaymentAddress,
  satoshis: i64
  position: i32,
  transaction_hash: TransactionHash,
  height: i64
}

struct Output {
  address: PaymentAddress,
  satoshis: i64,
  position: i32
  transaction_hash: TransactionHash,
  height: i64,
  spent: bool,
}

struct Utxo {
  output: Output,
  transaction: Transaction
}

enum AddressHistoryItem {
  Input(Input)
  Output(Output)
}

struct AddressHistory {
  address: PaymentAddress,
  received: u64,
  sent: u64,
  balance: u64,
  history: Vec<AddressHistoryItem>
}
