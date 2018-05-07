opaque_resource!{
  P2pT, P2pP, P2p {}
}

extern "C" {
    pub fn p2p_address_count(p2p: P2pP) -> u64;
}
