use enum_dispatch::enum_dispatch;
use nalgebra::Point2;

use super::Filters;

#[derive(Copy, Clone, Default)]
pub struct BoxFilter {
    /// The radius of the filter in the x and y directions.
    pub radius: (f32, f32),
}

impl BoxFilter {
    pub fn new(radius: (f32, f32)) -> BoxFilter {
        BoxFilter { radius }
    }
}

impl Filters for BoxFilter {
    fn evaluate(&self, point: Point2<f32>) -> f32 {
        1.0
    }
}
