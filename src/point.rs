use hash::Hash;
use std::os::raw::c_int;

opaque_resource_mapper!{
  PointT, PointP, Point {}
  async_and_sync {}
  impl {}
  extern { 
    pub fn chain_point_get_hash(point: PointP) -> Hash;
    pub fn chain_point_get_hash_out(point: PointP, out_hash: *mut Hash);
    pub fn chain_point_is_valid(point: PointP) -> c_int;
    pub fn chain_point_get_index(point: PointP) -> u32;
    pub fn chain_point_get_checksum(point: PointP) -> u64;
  }
}
