use history_compact::{HistoryCompactP, HistoryCompact};

opaque_resource_mapper!{
  HistoryCompactListT, HistoryCompactListP, HistoryCompactList {}
  async_and_sync {}

  impl {
    pub fn count(&self) -> u64 {
      unsafe{ chain_history_compact_list_count(self.raw) }
    }

    pub fn nth(&self, n: u64) -> HistoryCompact {
      HistoryCompact::new(unsafe{ chain_history_compact_list_nth(self.raw, n) })
    }

    pub fn items(&self) -> Vec<HistoryCompact> {
      let mut items = vec![];
      for i in 0..self.count() {
        items.push(self.nth(i));
      }
      items
    }
  }

  extern { 
    pub fn chain_history_compact_list_destruct(list: HistoryCompactListP);
    pub fn chain_history_compact_list_count(list: HistoryCompactListP) -> u64;
    pub fn chain_history_compact_list_nth(
        list: HistoryCompactListP,
        n: u64,
    ) -> HistoryCompactP;
  }
}
/*
impl Drop for HistoryCompactList {
  fn drop(&mut self){
    unsafe{ chain_history_compact_list_destruct(self.raw) }
  }
}
*/
