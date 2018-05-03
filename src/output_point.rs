use hash::Hash;

opaque_droppable_resource!{
  OutputPointT, OutputPointP, OutputPoint {}
  drop: output_point_destruct
}

impl OutputPoint {
  pub fn from_hash_index(hash: Hash, index: u32) -> Self {
    let raw = unsafe{ output_point_construct_from_hash_index(hash, index) };
    OutputPoint::new(raw)
  }

  pub fn destructible(raw: OutputPointP, destruct_on_drop: bool) -> OutputPoint {
    OutputPoint{raw, destruct_on_drop}
  }

  pub fn hash(&self) -> Hash {
    unsafe{ output_point_get_hash(self.raw) }
  }
}

extern { 
  pub fn output_point_get_hash(op: OutputPointP) -> Hash;
  pub fn output_point_get_hash_out(op: OutputPointP, out_hash: *mut Hash);
  pub fn output_point_construct() -> OutputPointP;
  pub fn output_point_construct_from_hash_index(hash: Hash, index: u32) -> OutputPointP;
  pub fn output_point_get_index(output: OutputPointP) -> u32;
}
