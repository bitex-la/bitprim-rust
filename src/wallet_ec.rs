use std::os::raw::c_int;
use payment_address::PaymentAddressP;

#[derive(Debug)]
#[repr(C, packed)]
pub struct EcSecret {
  pub hash: [u8; 32usize],
}

pub enum EcPublicT {}
pub type EcPublicP = *mut EcPublicT;
pub struct EcPublic(EcPublicP);

extern {
    pub fn wallet_ec_new(seed: *mut u8, n: u64) -> EcSecret;
    pub fn wallet_ec_to_public(
        secret: EcSecret,
        uncompressed: c_int,
    ) -> EcPublicP;
    pub fn wallet_ec_to_address(point: EcPublicP, version: u32) -> PaymentAddressP;
}
