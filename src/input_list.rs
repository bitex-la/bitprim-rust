use input::InputP;

pub enum InputListT {}
pub type InputListP = *mut InputListT;
pub struct InputList(InputListP);

extern "C" {
    pub fn chain_input_list_construct_default() -> InputListP;
    pub fn chain_input_list_push_back(list: InputListP, input: InputP);
    pub fn chain_input_list_destruct(list: InputListP);
    pub fn chain_input_list_count(list: InputListP) -> u64;
    pub fn chain_input_list_nth(list: InputListP, n: u64) -> InputP;
}
