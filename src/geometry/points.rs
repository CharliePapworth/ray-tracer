use crate::vec::{Vec3, Vec2};
use crate::geometry::plane::*;

pub type Point3 = Vec3;
pub type Point2 = Vec2;

impl Point3 {
    pub fn is_in_front(self, plane: Plane) -> bool {
        let (_, _, _, d) = plane.get_coefficients();
        plane.orientation.w.dot(self) + d <= 0.0
    } 

    pub fn distance_to_plane(self, plane: Plane) -> f64 {
        let (_, _, _, d) = plane.get_coefficients();
        let normal = plane.orientation.w;
        (self.dot(normal) + d).abs() / normal.length()
    }

    pub fn is_on_the_side_of(self, plane: Plane, other: Point3)  -> bool{
        let (_, _, _, d) = plane.get_coefficients();
        plane.orientation.w.dot(self) == plane.orientation.w.dot(other)
    }
}