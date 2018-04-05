use point::PointP;

opaque_resource_mapper!{
  PointListT, PointListP, PointList {}
  async_and_sync {}
  impl {}
  extern { 
    pub fn point_list_nth(point_list: PointListP, n: u64) -> PointP;
    pub fn point_list_count(point_list: PointListP) -> u64;
    pub fn point_list_destruct(point_list: PointListP);
  }
}
