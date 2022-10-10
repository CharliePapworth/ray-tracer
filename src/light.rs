use enum_dispatch::enum_dispatch;
use nalgebra::{Point3, Vector3};

use crate::{raytracing::HitRecord, spectrum::Spectrum};

#[enum_dispatch(Emit)]
#[derive(Clone, Copy)]
pub enum Light {
    PointLight(PointLight),
}

pub struct EmissionData {
    pub probability_density: f32,
    pub radiance: Spectrum,
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
    pub time: f32,
}

impl EmissionData {
    pub fn new(
        probability_density: f32,
        spectrum: Spectrum,
        origin: Point3<f32>,
        direction: Vector3<f32>,
        time: f32,
    ) -> EmissionData {
        EmissionData {
            probability_density,
            radiance: spectrum,
            origin,
            direction,
            time,
        }
    }
}

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
    fn emission_probability(&self, record: HitRecord, direction: Vector3<f32>) -> f32;
}

#[derive(Clone, Copy)]
pub struct PointLight {
    spectrum: Spectrum,
    position: Point3<f32>,
}

impl PointLight {
    pub fn new(spectrum: Spectrum, position: Point3<f32>) -> PointLight {
        PointLight { spectrum, position }
    }
}

impl Emit for PointLight {
    fn emit(&self, record: HitRecord) -> EmissionData {
        let point_in_scene = record.point_in_scene.cast::<f32>();
        let origin = self.position;
        let radiance = self.spectrum / (point_in_scene - self.position).norm_squared();
        let probability_density = 1.0;
        let origin = self.position;
        let direction = (origin - point_in_scene).normalize();
        let time = record.time + (origin - point_in_scene).norm();
        EmissionData::new(probability_density, radiance, origin, direction, time)
    }

    fn power(&self) -> Spectrum {
        todo!()
    }

    fn is_delta_distribution(&self) -> bool {
        true
    }

    fn emission_probability(&self, record: HitRecord, direction: Vector3<f32>) -> f32 {
        todo!()
    }
}
