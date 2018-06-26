use script::ScriptP;
use payment_address::PaymentAddressP;
use std::os::raw::c_int;
use destructible::*;

opaque_destructible_resource!{
  #[derive(Debug, Clone)]
  OutputT, OutputP, Output {}
  chain_output_destruct
}

impl Output {
    pub fn construct_default() -> OutputP {
        unsafe { chain_output_construct_default() }
    }

    pub fn script(&self) -> ScriptP {
        unsafe { chain_output_script(self.raw) }
    }
}

extern "C" {
    pub fn chain_output_construct_default() -> OutputP;
    pub fn chain_output_construct(value: u64, script: ScriptP) -> OutputP;
    pub fn chain_output_is_valid(output: OutputP) -> c_int;
    pub fn chain_output_serialized_size(output: OutputP, wire: c_int) -> u64;
    pub fn chain_output_value(output: OutputP) -> u64;
    pub fn chain_output_signature_operations(output: OutputP) -> u64;
    pub fn chain_output_script(output: OutputP) -> ScriptP;
    pub fn chain_output_payment_address(
        output: OutputP,
        use_testnet_rules: c_int,
    ) -> PaymentAddressP;
}
