use std::os::raw::{c_int};
use hash::Hash;
use point::PointP;
use point_kind::PointKind;

pub enum HistoryCompactT {}
pub type HistoryCompactP = *mut HistoryCompactT;
pub struct HistoryCompact(HistoryCompactP);

extern "C" {
    pub fn chain_history_compact_get_point_kind(history: HistoryCompactP) -> PointKind;
    pub fn chain_history_compact_get_point(history: HistoryCompactP) -> PointP;
    pub fn chain_history_compact_get_height(history: HistoryCompactP) -> u32;
    pub fn chain_history_compact_get_value_or_previous_checksum(
      history: HistoryCompactP) -> u64;
}
