use wallet_ec::EcSecret;

pub enum HdPrivateT {}
pub type HdPrivateP = *mut HdPrivateT;
pub struct HdPrivate(HdPrivateP);

extern {
    pub fn wallet_hd_new(seed: *mut u8, n: u64, version: u32) -> HdPrivateP;
    pub fn wallet_hd_private_to_ec(key: HdPrivateP) -> EcSecret;
}
