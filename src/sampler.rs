use std::f32::consts::PI;

use nalgebra::{Point2, Unit, UnitVector3, Vector3};

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

/// Chooses a direction on the hemisphere using a sample produced from a unit
/// square.
/// # Remarks
/// The output is not guaranteed (or even remotely likely) to be normalised.
/// # Further Reading
/// https://pbr-book.org/3ed-2018/Monte_Carlo_Integration/2D_Sampling_with_Multidimensional_Transformations#UniformlySamplingaHemisphere
pub fn uniformly_sample_hemisphere(square_sample: Point2<f32>) -> Vector3<f32> {
    let z = square_sample.x;
    let r = f32::sqrt(f32::max(0.0, 1.0 - z * z));
    let phi = 2 * PI * square_sample.y;
    Vector3::<f32>::new(r * f32::cos(phi), r * f32::sin(phi), z)
}

pub fn get_uniform_hemisphere_pdf_with_respect_to_solid_angle() -> f32 {
    0.5 / PI
}

/// Chooses a direction on a sphere using a sample produced from a unit
/// square.
/// # Remarks
/// The output is not guaranteed (or even remotely likely) to be normalised.
/// # Further Reading
/// https://pbr-book.org/3ed-2018/Monte_Carlo_Integration/2D_Sampling_with_Multidimensional_Transformations#UniformlySamplingaHemisphere
pub fn uniformly_sample_sphere(square_sample: Point2<f32>) {
    let z = 1.0 - 2.0 * square_sample.x;
    let radius = f32::sqrt(f32::max(0.0, 1.0 - z * z));
    let phi = 2 * PI * square_sample.y;
    Vector3::<f32>::new(radius * f32::cos(phi), radius * f32::sin(phi), z)
}

pub fn get_uniform_sphere_pdf_with_respect_to_solid_angle() -> f32 {
    0.25 / PI
}

/// Calculates a sample from a unit disk using a sample produced from a unit
/// square.
///  # Further Reading
/// https://pbr-book.org/3ed-2018/Monte_Carlo_Integration/2D_Sampling_with_Multidimensional_Transformations#ConcentricSampleDisk
pub fn concentrically_sample_from_disk(square_sample: Point2<f32>) -> Point2<f32> {
    // Map uniform random numbers to [-1, 1] x [-1, 1]
    let remapped_square_sample = Point2::new(square_sample.x * 2.0 - 1.0, square_sample.y * 2.0 - 1.0);

    // Handle degeneracy at the origin
    if remapped_square_sample.x == 0.0 && remapped_square_sample.y == 0.0 {
        return Point2::new(0.0, 0.0);
    }

    // Apply concentric mapping
    let (theta, radius) = if remapped_square_sample.x.abs() > remapped_square_sample.y.abs() {
        (remapped_square_sample.x, 0.25 * PI * (remapped_square_sample.y / remapped_square_sample.x));
    } else {
        (remapped_square_sample.y, 0.5 * PI - 0.25 * PI * (remapped_square_sample.x / remapped_square_sample.y));
    };

    Point2::new(radius * f32::cos(theta), radius * f32::sin(theta))
}

pub fn rand_unit_vec() -> Unit<Vector3<f32>> {
    Unit::new_normalize(rand_in_unit_sphere())
}
