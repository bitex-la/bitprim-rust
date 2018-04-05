use script::ScriptP;
use payment_address::PaymentAddressP;
use std::os::raw::c_int;

opaque_resource_mapper!{
  OutputT, OutputP, Output {}
  async_and_sync {}
  impl {}
  extern { 
    pub fn chain_output_construct_default() -> OutputP;
    pub fn chain_output_construct(value: u64, script: ScriptP) -> OutputP;
    pub fn chain_output_destruct(output: OutputP);
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
}
