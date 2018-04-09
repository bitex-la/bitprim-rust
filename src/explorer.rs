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

  pub fn utxos_for(address: String) -> Vec<Output> {
    /* Just incoming, and enough information to serialize them */
  }
}

type Address = String;

struct Block {
  height: i32
}

struct Transaction {
  txid: String,
  position: i32,
  block: Block
  /* Inputs and outputs are optional just to break recursion */
  inputs: Option<Vec<Input>>,
  outputs: Option<Vec<Output>>,
}

impl Transaction {
  pub fn with_inputs(self: Self) -> Self {
    self
  }
}

struct Input {
  address: Address,
  satoshis: i64,
  position: i32,
  prev_out: Output,
  script: String,
  transaction: Transaction
}

struct Output {
  address: Address,
  satoshis: i64,
  position: i32,
  script_type: ScriptType,
  script: String,
  spent: bool,
  transaction: Transaction
}

struct ScryptType {
  PayToAddress,
  PayToScriptHash
}

enum AddressHistoryItem {
  Input(Input),
  Output(Output)
}

struct AddressHistory {
  address: Address,
  received: u64,
  sent: u64,
  balance: u64,
  history: AddressHistoryItem
}
