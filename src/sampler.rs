use nalgebra::{Point2, Unit, Vector3};

pub trait Sample {
    fn get_1d(&self, min_inc: f32, max_exc: f32) -> f32;
    fn get_2d(&self, min_inc: f32, max_exc: f32) -> Point2<f32>;
}
pub struct Sampler {}

impl Sample for Sampler {
    fn get_1d(&self, min_inc: f32, max_exc: f32) -> f32 {
        fastrand::f32() * (max_exc - min_inc) + min_inc
    }

    fn get_2d(&self, min_inc: f32, max_exc: f32) -> Point2<f32> {
        let multiple = (max_exc - min_inc) + min_inc;
        Point2::<f32>::new(fastrand::f32() * multiple, fastrand::f32() * multiple)
    }
}
//Generates random numbers between [min_inc, max_exc)
pub fn rand_double(min_inc: f32, max_exc: f32) -> f32 {
    fastrand::f32() * (max_exc - min_inc) + min_inc
}

pub fn rand(min: f32, max: f32) -> Vector3<f32> {
    Vector3::<f32>::new(rand_double(min, max), rand_double(min, max), rand_double(min, max))
}

pub fn rand_in_unit_sphere() -> Vector3<f32> {
    loop {
        let p = rand(-1.0, 1.0);
        if p.norm() < 1.0 {
            break (p);
        }
    }
}

pub fn rand_in_unit_disk() -> Vector3<f32> {
    loop {
        let p = Vector3::<f32>::new(rand_double(-1.0, 1.0), rand_double(-1.0, 1.0), 0.0);
        if p.norm() < 1.0 {
            break (p);
        }
    }
}

pub fn rand_unit_vec() -> Unit<Vector3<f32>> {
    Unit::new_normalize(rand_in_unit_sphere())
}
