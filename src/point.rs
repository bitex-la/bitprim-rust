use hash::Hash;
use std::os::raw::c_int;
use output_point::OutputPoint;

opaque_resource!{
  PointT, PointP, Point {}
}

impl Point {
  pub fn hash(&self) -> Hash {
    unsafe{ chain_point_get_hash(self.raw) }
  }

  pub fn is_valid(&self) -> bool {
    (unsafe{ chain_point_is_valid(self.raw) }) == 1

  }

  pub fn index(&self) -> u32 {
    unsafe{ chain_point_get_index(self.raw) }
  }

  pub fn to_output_point(&self) -> OutputPoint {
    OutputPoint::from_hash_index(self.hash(), self.index())
  }
}

extern { 
  pub fn chain_point_get_hash(point: PointP) -> Hash;
  pub fn chain_point_get_hash_out(point: PointP, out_hash: *mut Hash);
  pub fn chain_point_is_valid(point: PointP) -> c_int;
  pub fn chain_point_get_index(point: PointP) -> u32;
  pub fn chain_point_get_checksum(point: PointP) -> u64;
}
