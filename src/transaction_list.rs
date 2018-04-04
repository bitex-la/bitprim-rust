use transaction::TransactionP;

pub enum TransactionListT {}
pub type TransactionListP = *mut TransactionListT;
pub struct TransactionList(TransactionListP);

extern "C" {
    pub fn chain_transaction_list_construct_default() -> TransactionListP;
    pub fn chain_transaction_list_push_back(list: TransactionListP, transaction: TransactionP);
    pub fn chain_transaction_list_destruct(list: TransactionListP);
    pub fn chain_transaction_list_count(list: TransactionListP) -> u64;
    pub fn chain_transaction_list_nth(list: TransactionListP, n: u64) -> TransactionP;
}
