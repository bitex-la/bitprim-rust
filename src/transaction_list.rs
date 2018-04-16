use transaction::{Transaction, TransactionP};

opaque_droppable_resource!{
  TransactionListT, TransactionListP, TransactionList {
    iter: u32, default: 0;
  }
  drop: transaction_list_destruct
}

opaque_collection! {
  TransactionList, TransactionListP,
  Transaction, TransactionP,
  chain_transaction_list_count,
  chain_transaction_list_nth
}

extern {
  pub fn chain_transaction_list_construct_default() -> TransactionListP;
  pub fn chain_transaction_list_push_back(list: TransactionListP, transaction: TransactionP);
}
