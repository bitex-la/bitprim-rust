use history_compact::HistoryCompactP;

opaque_resource_mapper!{
  HistoryCompactListT, HistoryCompactListP, HistoryCompactList {}
  async_and_sync {}
  impl {}
  extern { 
    pub fn chain_history_compact_list_destruct(list: HistoryCompactListP);
    pub fn chain_history_compact_list_count(list: HistoryCompactListP) -> u64;
    pub fn chain_history_compact_list_nth(
        list: HistoryCompactListP,
        n: u64,
    ) -> HistoryCompactP;
  }
}
