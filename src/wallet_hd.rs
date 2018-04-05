use wallet_ec::EcSecret;

opaque_resource_mapper!{
  HdPrivateT, HdPrivateP, HdPrivate {}
  async_and_sync {}
  impl {}
  extern {
    pub fn wallet_hd_new(seed: *mut u8, n: u64, version: u32) -> HdPrivateP;
    pub fn wallet_hd_private_to_ec(key: HdPrivateP) -> EcSecret;
  }
}
