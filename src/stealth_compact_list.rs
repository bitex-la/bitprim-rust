use stealth_compact::StealthCompactP;

pub enum StealthCompactListT {}
pub type StealthCompactListP = *mut StealthCompactListT;
pub struct StealthCompactList(StealthCompactListP);

extern "C" {
    pub fn stealth_compact_list_destruct(list: StealthCompactListP);
    pub fn stealth_compact_list_count(list: StealthCompactListP) -> u64;
    pub fn stealth_compact_list_nth(list: StealthCompactListP, n: u64) -> StealthCompactP;
}
