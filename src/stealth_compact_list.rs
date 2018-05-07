use stealth_compact::{StealthCompact, StealthCompactP};
use destructible::*;
use opaque_collection::*;

opaque_destructible_resource!{
  StealthCompactListT, StealthCompactListP, StealthCompactList {}
  stealth_compact_list_destruct
}

derive_opaque_collection! {
  StealthCompactList, StealthCompactListP,
  StealthCompact, StealthCompactP,
  stealth_compact_list_count,
  stealth_compact_list_nth
}
