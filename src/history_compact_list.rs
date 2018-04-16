use std::iter::Iterator;
use history_compact::{HistoryCompactP, HistoryCompact};

opaque_droppable_resource! {
  HistoryCompactListT, HistoryCompactListP, HistoryCompactList {
    iter: u32, default: 0;
  }
  drop: chain_history_compact_list_destruct
}

opaque_collection! {
  HistoryCompactList,
  HistoryCompactListP,
  HistoryCompact,
  HistoryCompactP,
  chain_history_compact_list_count,
  chain_history_compact_list_nth
}
