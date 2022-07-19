use nalgebra::Point2;

#[derive (Clone)]
pub struct BoxFilter {
    /// The radius of the filter in the x and y directions.
    pub radius: (f32, f32)
}

impl BoxFilter {
    pub fn new(radius: (f32, f32)) -> BoxFilter {
        BoxFilter { radius }
    }

    pub fn evaluate(&self, point: Point2<f32>) -> f32{
        1.0
    }
}