use std::os::raw::{c_char, c_int};
use std::ffi::CStr;
use std::slice;
use destructible::*;

opaque_destructible_resource!{
  ScriptT, ScriptP, Script {}
  chain_script_destruct
}

impl Script {
    pub fn to_str(&self, active_forks: u32) -> &str {
        unsafe {  CStr::from_ptr(chain_script_to_string(self.raw, active_forks)).to_str().unwrap() }
    }

    pub fn to_hex(&self, prefix: i32) -> String {
        let mut out_size = 0;
        let pointer = unsafe { chain_script_to_data(self.raw, prefix, &mut out_size) };
        let byte_array = unsafe { slice::from_raw_parts(pointer, out_size as usize) };

        unsafe { platform_free(pointer) };

        byte_array
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("")
    }
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
    pub fn platform_free(ptr: *const u8);
}
