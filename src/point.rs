use hash::Hash;
use std::os::raw::c_int;

opaque_resource_mapper!{
  PointT, PointP, Point {}
  async_and_sync {}
  impl {
    pub fn hash(&self) -> Hash {
      unsafe{ chain_point_get_hash(self.raw) }
    }

    pub fn is_valid(&self) -> bool {
      (unsafe{ chain_point_is_valid(self.raw) }) == 1

    }

    pub fn index(&self) -> u32 {
      unsafe{ chain_point_get_index(self.raw) }
    }
  }

  extern { 
    pub fn chain_point_get_hash(point: PointP) -> Hash;
    pub fn chain_point_get_hash_out(point: PointP, out_hash: *mut Hash);
    pub fn chain_point_is_valid(point: PointP) -> c_int;
    pub fn chain_point_get_index(point: PointP) -> u32;
    pub fn chain_point_get_checksum(point: PointP) -> u64;
  }
}
