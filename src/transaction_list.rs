use transaction::{Transaction, TransactionP};
use destructible::*;
use opaque_collection::*;

opaque_destructible_resource!{
  TransactionListT, TransactionListP, TransactionList {}
  transaction_list_destruct
}

derive_opaque_collection! {
  TransactionList, TransactionListP,
  Transaction, TransactionP,
  chain_transaction_list_count,
  chain_transaction_list_nth
}

extern {
  pub fn chain_transaction_list_construct_default() -> TransactionListP;
  pub fn chain_transaction_list_push_back(list: TransactionListP, transaction: TransactionP);
}
