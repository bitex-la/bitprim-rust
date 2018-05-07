use history_compact::{HistoryCompactP, HistoryCompact};
use destructible::*;
use opaque_collection::*;

opaque_destructible_resource! {
  HistoryCompactListT, HistoryCompactListP, HistoryCompactList {}
  chain_history_compact_list_destruct
}

derive_opaque_collection! {
  HistoryCompactList,
  HistoryCompactListP,
  HistoryCompact,
  HistoryCompactP,
  chain_history_compact_list_count,
  chain_history_compact_list_nth
}
