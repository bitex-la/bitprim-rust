opaque_resource_mapper!{
  P2pT, P2pP, P2p {}
  async_and_sync {}
  impl {}
  extern { 
    pub fn p2p_address_count(p2p: P2pP) -> u64;
  }
}
