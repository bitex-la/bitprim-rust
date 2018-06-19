use std::os::raw::c_int;
use output_point::{OutputPoint, OutputPointP};
use script::ScriptP;
use destructible::*;

opaque_destructible_resource!{
  InputT, InputP, Input {}
  chain_input_destruct
}

impl Input {
    pub fn is_valid(&self) -> bool {
        (unsafe { chain_input_is_valid(self.raw) }) == 1
    }

    pub fn previous_output(&self) -> OutputPoint {
        OutputPoint::new(unsafe { chain_input_previous_output(self.raw) })
    }

    pub fn sequence(&self) -> u32 {
        unsafe { chain_input_sequence(self.raw) }
    }

    pub fn script(&self) -> ScriptP {
        unsafe { chain_input_script(self.raw) }
    }
}

extern "C" {
    pub fn chain_input_construct_default() -> InputP;
    pub fn chain_input_construct(
        previous_output: OutputPointP,
        script: ScriptP,
        sequence: u32,
    ) -> InputP;
    pub fn chain_input_is_valid(input: InputP) -> c_int;
    pub fn chain_input_is_final(input: InputP) -> c_int;
    pub fn chain_input_serialized_size(input: InputP, wire: c_int) -> u64;
    pub fn chain_input_sequence(input: InputP) -> u32;
    pub fn chain_input_signature_operations(input: InputP, bip16_active: c_int) -> u64;
    pub fn chain_input_script(input: InputP) -> ScriptP;
    pub fn chain_input_previous_output(input: InputP) -> OutputPointP;
}
