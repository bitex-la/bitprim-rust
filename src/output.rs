use script::{ Script, ScriptP };
use payment_address::{ PaymentAddress, PaymentAddressP };
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

    pub fn script(&self) -> Script {
        Script::new(unsafe { chain_output_script(self.raw) })
    }

    pub fn value(&self) -> u64 {
        unsafe { chain_output_value(self.raw) }
    }

    pub fn address(&self, testnet: i32) -> String {
        let payment_address = PaymentAddress::new(unsafe { chain_output_payment_address(self.raw, testnet) }  );
        payment_address.to_str().to_string()
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
