pub mod box_filter;
use enum_dispatch::enum_dispatch;
use nalgebra::Point2;

use self::box_filter::BoxFilter;

#[enum_dispatch(Filters)]
#[derive (Copy, Clone)]
pub enum Filter {
    BoxFilter(BoxFilter)
}

#[enum_dispatch]
pub trait Filters {
    fn evaluate(&self, point: Point2<f32>) -> f32;
}
