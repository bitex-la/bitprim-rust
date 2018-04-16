use point::{Point, PointP};

opaque_droppable_resource!{
  PointListT, PointListP, PointList {
    iter: u32, default: 0;
  }
  drop: point_list_destruct
}

opaque_collection! {
  PointList, PointListP,
  Point, PointP,
  point_list_count,
  point_list_nth
}
