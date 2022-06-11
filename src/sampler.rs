use nalgebra::Unit;

use crate::nalgebra::{Vector3};

//Generates random numbers between [min_inc, max_exc)
pub fn rand_double(min_inc: f64, max_exc: f64) -> f64{
    fastrand::f64()*(max_exc - min_inc) + min_inc
}

pub fn rand(min: f64, max:f64) -> Vector3<f64>{
    Vector3::<f64>::new(rand_double(min, max), rand_double(min, max), rand_double(min, max))
}

pub fn rand_in_unit_sphere() -> Vector3<f64>{
    loop{
        let p = rand(-1.0, 1.0);
        if p.norm() < 1.0{
            break(p)
        }
    }
}

pub fn rand_in_unit_disk() -> Vector3<f64>{
    loop{
        let p = Vector3::<f64>::new(rand_double(-1.0, 1.0), rand_double(-1.0, 1.0), 0.0);
        if p.norm() < 1.0{
            break(p)
        }
    }
}

pub fn rand_unit_vec() -> Unit<Vector3<f64>>{
    Unit::new_normalize(rand_in_unit_sphere())
}