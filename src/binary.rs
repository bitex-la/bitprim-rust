use std::os::raw::c_char;

opaque_droppable_resource! {
  BinaryT, BinaryP, Binary {}
  drop: binary_destruct
}

extern { 
  pub fn binary_construct() -> BinaryP;
  pub fn binary_construct_string(string: *const c_char) -> BinaryP;
  pub fn binary_construct_blocks(bits_size: u64, blocks: *mut u8, n: u64) -> BinaryP;
  pub fn binary_blocks(binary: BinaryP, out_n: *mut u64) -> *const u8;
  pub fn binary_encoded(binary: BinaryP) -> *mut c_char;
}
