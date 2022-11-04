use crate::light::{Light, PointLight};
use crate::raytracing::HitRecord;
use enum_dispatch::enum_dispatch;
use nalgebra::UnitVector3;

use super::{EmissionData, Spectrum};

/// Interface for lights in the scene.

#[enum_dispatch]
pub trait Emit {
    /// Samples a point on the light source’s surface and computes the
    /// radiance arriving at a given point in the scene (as provided by the hit
    /// record) due to illumination from the light.
    fn emit(&self, record: HitRecord) -> EmissionData;
    fn power(&self) -> Spectrum;

    /// Returns true if the light is a delta distribution and false otherwise. A
    /// light is a delta distribution if...
    fn is_delta_distribution(&self) -> bool;

    /// Returns the likelihood of the light emitting in a given direction
    fn emission_probability(&self, record: HitRecord, direction: UnitVector3<f32>) -> f32;
}
