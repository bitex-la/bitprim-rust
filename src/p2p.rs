pub enum P2pT {}
pub type P2pP = *mut P2pT;
pub struct P2p(P2pP);

impl P2p {
	pub fn new(p2p_p: P2pP) -> P2p { P2p(p2p_p) }
}
