use stealth_compact::{StealthCompact, StealthCompactP};

opaque_droppable_resource!{
  StealthCompactListT, StealthCompactListP, StealthCompactList {
    iter: u32, default: 0;
  }
  drop: stealth_compact_list_destruct
}

opaque_collection! {
  StealthCompactList, StealthCompactListP,
  StealthCompact, StealthCompactP,
  stealth_compact_list_count,
  stealth_compact_list_nth
}
