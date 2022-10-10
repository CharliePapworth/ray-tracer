use nalgebra::Point2;

/// Holds all of the sample values needed to specify a camera ray.
pub struct CameraSample {
    /// Gives the point on the film to which the generated ray carries radiance
    pub ray_film_intersection: Point2<f32>,
    /// Gives the point on the lens that the ray passes through
    pub ray_lens_intersection: Point2<f32>,
}
