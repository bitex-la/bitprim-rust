use stealth_compact::StealthCompactP;

opaque_resource_mapper!{
  StealthCompactListT, StealthCompactListP, StealthCompactList {}
  async_and_sync {}
  impl {}
  extern { 
    pub fn stealth_compact_list_destruct(list: StealthCompactListP);
    pub fn stealth_compact_list_count(list: StealthCompactListP) -> u64;
    pub fn stealth_compact_list_nth(list: StealthCompactListP, n: u64) -> StealthCompactP;
  }
}
