use std::os::raw::c_char;
use long_hash::LongHash;

opaque_droppable_resource!{
  WordListT, WordListP, WordList {}
  drop: word_list_destruct
}
extern {
  pub fn word_list_construct() -> WordListP;
  pub fn word_list_add_word(word_list: WordListP, word: *const c_char);
  pub fn wallet_mnemonics_to_seed(mnemonics: WordListP) -> LongHash;
}
