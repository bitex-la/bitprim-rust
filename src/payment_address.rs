use std::os::raw::{c_char, c_int};
use std::ffi::CStr;
use std::ffi::CString;
use std::str::FromStr;
use std::string::ParseError;
use destructible::*;

opaque_destructible_resource!{
  PaymentAddressT, PaymentAddressP, PaymentAddress {}
  chain_payment_address_destruct
}

impl PaymentAddress {
    pub fn to_str(&self) -> &str {
        unsafe {  CStr::from_ptr(wallet_payment_address_encoded(self.raw)).to_str().unwrap() }
    }
}

impl FromStr for PaymentAddress {
    type Err = ParseError;

    fn from_str(hex: &str) -> Result<PaymentAddress, Self::Err> {
        let c_hex = CString::new(hex).expect("Invalid hex");
        Ok(PaymentAddress::new(unsafe {
            wallet_payment_address_construct_from_string(c_hex.as_ptr())
        }))
    }
}

extern "C" {
    pub fn wallet_payment_address_encoded(payment_address: PaymentAddressP) -> *const c_char;
    pub fn wallet_payment_address_construct_from_string(address: *const c_char) -> PaymentAddressP;
    pub fn wallet_payment_address_version(payment_address: PaymentAddressP) -> u8;
    pub fn wallet_payment_address_is_valid(payment_address: PaymentAddressP) -> c_int;
}
