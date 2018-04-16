use point::{PointP, Point};
use point_kind::PointKind;

opaque_resource!{
  HistoryCompactT, HistoryCompactP, HistoryCompact {}
}

impl HistoryCompact {
  pub fn point(&self) -> Point {
    Point::new(unsafe{ chain_history_compact_get_point(self.raw) })
  }

  /* If it's an output, this will be the value, otherwise it's the checksum */
  pub fn get_value_or_previous_checksum(&self) -> u64 {
    unsafe{ chain_history_compact_get_value_or_previous_checksum(self.raw) }
  }

  pub fn point_kind(&self) -> PointKind {
    unsafe{ chain_history_compact_get_point_kind(self.raw) }
  }

  pub fn height(&self) -> u32 {
    unsafe{ chain_history_compact_get_height(self.raw) }
  }
}

extern { 
  pub fn chain_history_compact_get_point_kind(history: HistoryCompactP) -> PointKind;
  pub fn chain_history_compact_get_point(history: HistoryCompactP) -> PointP;
  pub fn chain_history_compact_get_height(history: HistoryCompactP) -> u32;
  pub fn chain_history_compact_get_value_or_previous_checksum(
    history: HistoryCompactP) -> u64;
}
