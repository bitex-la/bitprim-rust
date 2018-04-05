use hash::Hash;

opaque_resource_mapper!{
  OutputPointT, OutputPointP, OutputPoint {}
  async_and_sync {}
  impl {}
  extern { 
    pub fn output_point_get_hash(op: OutputPointP) -> Hash;
    pub fn output_point_get_hash_out(op: OutputPointP, out_hash: *mut Hash);
    pub fn output_point_construct() -> OutputPointP;
    pub fn output_point_construct_from_hash_index(hash: Hash, index: u32) -> OutputPointP;
    pub fn output_point_get_index(output: OutputPointP) -> u32;
    pub fn output_point_destruct(op: OutputPointP);
  }
}
