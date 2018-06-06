use hash::Hash;
use short_hash::ShortHash;

opaque_resource!{
  StealthCompactT, StealthCompactP, StealthCompact {}
}

extern "C" {
    pub fn stealth_compact_get_ephemeral_public_key_hash(stealth: StealthCompactP) -> Hash;
    pub fn stealth_compact_get_ephemeral_public_key_hash_out(
        stealth: StealthCompactP,
        out_epk_hash: *mut Hash,
    );
    pub fn stealth_compact_get_transaction_hash(stealth: StealthCompactP) -> Hash;
    pub fn stealth_compact_get_transaction_hash_out(
        stealth: StealthCompactP,
        out_tx_hash: *mut Hash,
    );
    pub fn stealth_compact_get_public_key_hash(stealth: StealthCompactP) -> ShortHash;
    pub fn stealth_compact_get_public_key_hash_out(
        stealth: StealthCompactP,
        out_pk_hash: *mut ShortHash,
    );
}
