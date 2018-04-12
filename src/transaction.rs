use std::os::raw::{c_int, c_char};
use hash::Hash;
use input_list::{InputListP, InputList};
use output_list::OutputListP;
use script::ScriptP;

opaque_resource_mapper!{
  TransactionT, TransactionP, Transaction {}
  async_and_sync {}
  impl {
    pub fn hash(&self) -> Hash {
      unsafe{ chain_transaction_hash(self.raw) }
    }

    pub fn inputs(&self) -> InputList {
      let raw = unsafe{ chain_transaction_inputs(self.raw) };
      InputList{raw}
    }
  }
  extern { 
    pub fn hex_to_tx(tx_hex: *const c_char) -> TransactionP;
    pub fn chain_transaction_construct_default() -> TransactionP;
    pub fn chain_transaction_construct(
        version: u32,
        locktime: u32,
        inputs: InputListP,
        outputs: OutputListP,
    ) -> TransactionP;
    pub fn chain_transaction_destruct(transaction: TransactionP);
    pub fn chain_transaction_is_valid(transaction: TransactionP) -> c_int;
    pub fn chain_transaction_version(transaction: TransactionP) -> u32;
    pub fn chain_transaction_set_version(transaction: TransactionP, version: u32);
    pub fn chain_transaction_hash(transaction: TransactionP) -> Hash;
    pub fn chain_transaction_hash_out(transaction: TransactionP, out_hash: *mut Hash);
    pub fn chain_transaction_hash_sighash_type(
        transaction: TransactionP,
        sighash_type: u32,
    ) -> Hash;
    pub fn chain_transaction_hash_sighash_type_out(
        transaction: TransactionP,
        sighash_type: u32,
        out_hash: *mut Hash,
    );
    pub fn chain_transaction_locktime(transaction: TransactionP) -> u32;
    pub fn chain_transaction_serialized_size(
        transaction: TransactionP,
        wire: c_int,
    ) -> u64;
    pub fn chain_transaction_fees(transaction: TransactionP) -> u64;
    pub fn chain_transaction_signature_operations(transaction: TransactionP) -> u64;
    pub fn chain_transaction_signature_operations_bip16_active(
        transaction: TransactionP,
        bip16_active: c_int,
    ) -> u64;
    pub fn chain_transaction_total_input_value(transaction: TransactionP) -> u64;
    pub fn chain_transaction_total_output_value(transaction: TransactionP) -> u64;
    pub fn chain_transaction_is_coinbase(transaction: TransactionP) -> c_int;
    pub fn chain_transaction_is_null_non_coinbase(
        transaction: TransactionP,
    ) -> c_int;
    pub fn chain_transaction_is_oversized_coinbase(
        transaction: TransactionP,
    ) -> c_int;
    pub fn chain_transaction_is_mature(
        transaction: TransactionP,
        target_height: u64,
    ) -> c_int;
    pub fn chain_transaction_is_overspent(transaction: TransactionP) -> c_int;
    pub fn chain_transaction_is_double_spend(
        transaction: TransactionP,
        include_unconfirmed: c_int,
    ) -> c_int;
    pub fn chain_transaction_is_missing_previous_outputs(
        transaction: TransactionP,
    ) -> c_int;
    pub fn chain_transaction_is_final(
        transaction: TransactionP,
        block_height: u64,
        block_time: u32,
    ) -> c_int;
    pub fn chain_transaction_is_locktime_conflict(
        transaction: TransactionP,
    ) -> c_int;
    pub fn chain_transaction_outputs(transaction: TransactionP) -> OutputListP;
    pub fn chain_transaction_inputs(transaction: TransactionP) -> InputListP;
    pub fn chain_transaction_to_data(
        script: ScriptP,
        wire: c_int,
        out_size: *mut u64,
    ) -> *const u8;
  }
}

/*
impl Drop for Transaction {
  fn drop(&mut self){
    println!("Destruct transaction {:?}", self.raw);
    unsafe{ chain_transaction_destruct(self.raw) }
    println!("Destructed trensaction {:?}", self.raw);
  }
}
*/
