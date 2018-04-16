use std::os::raw::{c_char, c_int};
use std::ffi::CString;

opaque_droppable_resource!{
  PaymentAddressT, PaymentAddressP, PaymentAddress {}
  drop: chain_payment_address_destruct
}

impl PaymentAddress {
  pub fn from_str(hex: &str) -> PaymentAddress {
    let c_hex = CString::new(hex).expect("Invalid hex");
    PaymentAddress::new(
      unsafe { chain_payment_address_construct_from_string(c_hex.as_ptr()) }
    )
  }
}

extern { 
  pub fn chain_payment_address_encoded(payment_address: PaymentAddressP) 
    -> *const c_char;
  pub fn chain_payment_address_construct_from_string(address: *const c_char)
    -> PaymentAddressP;
  pub fn chain_payment_address_version(payment_address: PaymentAddressP) -> u8;
  pub fn chain_payment_address_is_valid(payment_address: PaymentAddressP)
    -> c_int;
}
