use std::os::raw::c_int;
use output_point::OutputPointP;
use script::ScriptP;

opaque_resource_mapper!{
  InputT, InputP, Input {}
  async_and_sync {}
  impl {}
  extern { 
    pub fn chain_input_construct_default() -> InputP;
    pub fn chain_input_construct(
        previous_output: OutputPointP,
        script: ScriptP,
        sequence: u32,
    ) -> InputP;
    pub fn chain_input_destruct(input: InputP);
    pub fn chain_input_is_valid(input: InputP) -> c_int;
    pub fn chain_input_is_final(input: InputP) -> c_int;
    pub fn chain_input_serialized_size(input: InputP, wire: c_int) -> u64;
    pub fn chain_input_sequence(input: InputP) -> u32;
    pub fn chain_input_signature_operations(
        input: InputP,
        bip16_active: c_int,
    ) -> u64;
    pub fn chain_input_script(input: InputP) -> ScriptP;
    pub fn chain_input_previous_output(input: InputP) -> OutputPointP;
  }
}
