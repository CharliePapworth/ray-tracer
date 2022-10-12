use nalgebra::{Point2, Point3};

pub trait Point3ExtensionMethods {
    fn swap(&mut self, i: usize, j: usize);
}

impl Point3ExtensionMethods for Point3<f32> {
    fn swap(&mut self, i: usize, j: usize) {
        let temp = self[j];
        self[j] = self[i];
        self[i] = temp;
    }
}
