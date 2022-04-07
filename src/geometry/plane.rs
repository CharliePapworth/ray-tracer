use core::time;
use std::ops::{Index, IndexMut, RangeBounds};

use crate::enum_dispatch::*;
use crate::ray::{Ray, RayPlaneIntersection};
use line_drawing::Bresenham;
use crate::points::Point3;
use crate::vec::{Vec2, Vec3};
use crate::camera::{Camera, CameraSettings, Orientation};

#[derive (PartialEq, Debug, Copy, Clone, Default)]
pub struct Plane {
    pub orientation: Orientation,
    pub origin: Point3
}

impl Plane {
    pub fn new(orientation: Orientation, origin: Point3) -> Plane {
        Plane { orientation, origin }
    }

    pub fn get_coefficients(&self) -> (f64, f64, f64, f64) {
        (self.orientation.w[0], self.orientation.w[1], self.orientation.w[2], - self.orientation.w.dot(self.origin))
    }
}