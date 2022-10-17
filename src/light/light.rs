use enum_dispatch::enum_dispatch;
use nalgebra::{Point3, UnitVector3};

#[rustfmt::skip]
use crate::{
    raytracing::HitRecord,
    light::Spectrum
};

use super::Emit;

#[enum_dispatch(Emit)]
#[derive(Clone, Copy)]
pub enum Light {
    PointLight(PointLight),
}

pub struct EmissionData {
    pub probability_density: f32,
    pub radiance: Spectrum,
    pub origin: Point3<f32>,
    pub direction: UnitVector3<f32>,
    pub time: f32,
}

impl EmissionData {
    pub fn new(
        probability_density: f32,
        spectrum: Spectrum,
        origin: Point3<f32>,
        direction: UnitVector3<f32>,
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
        let direction = UnitVector3::new_normalize(origin - point_in_scene);
        let time = record.time + (origin - point_in_scene).norm();
        EmissionData::new(probability_density, radiance, origin, direction, time)
    }

    fn power(&self) -> Spectrum {
        todo!()
    }

    fn is_delta_distribution(&self) -> bool {
        true
    }

    fn emission_probability(&self, record: HitRecord, direction: UnitVector3<f32>) -> f32 {
        todo!()
    }
}
