use point::{Point, PointP};
use destructible::*;
use opaque_collection::*;

opaque_destructible_resource!{
  PointListT, PointListP, PointList {}
  point_list_destruct
}

derive_opaque_collection! {
  PointList, PointListP,
  Point, PointP,
  point_list_count,
  point_list_nth
}
