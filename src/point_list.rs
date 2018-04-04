use point::PointP;

pub enum PointListT {}
pub type PointListP = *mut PointListT;
pub struct PointList(PointListP);

extern "C" {
    pub fn point_list_nth(point_list: PointListP, n: u64) -> PointP;
    pub fn point_list_count(point_list: PointListP) -> u64;
    pub fn point_list_destruct(point_list: PointListP);
}
