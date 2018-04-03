use std::os::raw::{c_int};
use hash::Hash;
use history_compact::HistoryCompactP;

pub enum HistoryCompactListT {}
pub type HistoryCompactListP = *mut HistoryCompactListT;
pub struct HistoryCompactList(HistoryCompactListP);

extern "C" {
    pub fn chain_history_compact_list_destruct(list: HistoryCompactListP);
    pub fn chain_history_compact_list_count(list: HistoryCompactListP) -> u64;
    pub fn chain_history_compact_list_nth(
        list: HistoryCompactListP,
        n: u64,
    ) -> HistoryCompactP;
}
