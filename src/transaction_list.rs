use transaction::TransactionP;

opaque_resource_mapper!{
  TransactionListT, TransactionListP, TransactionList {}
  async_and_sync {}
  impl {}
  extern {
    pub fn chain_transaction_list_construct_default() -> TransactionListP;
    pub fn chain_transaction_list_push_back(list: TransactionListP, transaction: TransactionP);
    pub fn chain_transaction_list_destruct(list: TransactionListP);
    pub fn chain_transaction_list_count(list: TransactionListP) -> u64;
    pub fn chain_transaction_list_nth(list: TransactionListP, n: u64) -> TransactionP;
  }
}
