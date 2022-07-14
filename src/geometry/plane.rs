use crate::nalgebra::{Vector3, Point3};
use crate::camera::Orientation;

#[derive (PartialEq, Debug, Copy, Clone)]
/// Represents an infinite, 3-dimensional plane. The plane is defined by two basis vectors and a normal (given by its orientation),
/// along with an origin.
pub struct Plane {
    pub orientation: Orientation,
    pub origin: Point3<f32>
}

impl Plane {
    pub fn new(orientation: Orientation, origin: Point3<f32>) -> Plane {
        Plane { orientation, origin }
    }

    /// Returns the general form of the plane:  `ax + by + cz + d = 0`
    pub fn get_coefficients(&self) -> (f32, f32, f32, f32) {
        (self.orientation.w[0], self.orientation.w[1], self.orientation.w[2], - self.orientation.w.dot(&self.origin.coords))
    }
}