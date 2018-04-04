use std::os::raw::{c_char, c_int};

pub enum PaymentAddressT {}
pub type PaymentAddressP = *mut PaymentAddressT;
pub struct PaymentAddress(PaymentAddressP);

extern "C" {
    pub fn chain_payment_address_encoded(
        payment_address: PaymentAddressP,
    ) -> *const c_char;
    pub fn chain_payment_address_construct_from_string(
        address: *const c_char,
    ) -> PaymentAddressP;
    pub fn chain_payment_address_version(payment_address: PaymentAddressP) -> u8;
    pub fn chain_payment_address_is_valid(
        payment_address: PaymentAddressP,
    ) -> c_int;
    pub fn chain_payment_address_destruct(payment_address: PaymentAddressP);
}
