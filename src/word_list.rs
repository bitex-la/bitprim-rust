use std::os::raw::c_char;
use long_hash::LongHash;

pub enum WordListT {}
pub type WordListP = *mut WordListT;
pub struct WordList(WordListP);

extern "C" {
    pub fn word_list_construct() -> WordListP;
    pub fn word_list_add_word(word_list: WordListP, word: *const c_char);
    pub fn word_list_destruct(word_list: WordListP);
    pub fn wallet_mnemonics_to_seed(mnemonics: WordListP) -> LongHash;
}
