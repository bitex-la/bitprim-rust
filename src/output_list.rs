use output::OutputP;

pub enum OutputListT {}
pub type OutputListP = *mut OutputListT;
pub struct OutputList(OutputListP);

extern "C" {
    pub fn chain_output_list_construct_default() -> OutputListP;
    pub fn chain_output_list_push_back(list: OutputListP, output: OutputP);
    pub fn chain_output_list_destruct(list: OutputListP);
    pub fn chain_output_list_count(list: OutputListP) -> u64;
    pub fn chain_output_list_nth(list: OutputListP, n: u64) -> OutputP;
}
