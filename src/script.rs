use std::os::raw::{c_char, c_int};
use destructible::*;

opaque_destructible_resource!{
  ScriptT, ScriptP, Script {}
  chain_script_destruct
}

extern "C" {
    pub fn chain_script_is_valid(script: ScriptP) -> c_int;
    pub fn chain_script_is_valid_operations(script: ScriptP) -> c_int;
    pub fn chain_script_satoshi_content_size(script: ScriptP) -> u64;
    pub fn chain_script_serialized_size(script: ScriptP, prefix: c_int) -> u64;
    pub fn chain_script_to_string(script: ScriptP, active_forks: u32) -> *const c_char;
    pub fn chain_script_type(script: ScriptP) -> *const c_char;
    pub fn chain_script_to_data(script: ScriptP, prefix: c_int, out_size: *mut u64) -> *const u8;
    pub fn chain_script_sigops(script: ScriptP, embedded: c_int) -> u64;
    pub fn chain_script_embedded_sigops(script: ScriptP, prevout_script: ScriptP) -> u64;
}
