use nalgebra::UnitVector3;
use std::f32::consts::PI;

use crate::light::Spectrum;

use super::material::{ReflectionModel, Scatter};

#[derive(Copy, Clone, Default)]
///Specular materials, such as perfectly smooth surfaces, exhibit perfect
/// specular reflection and transmission: for an incident direction, all light
/// is scattered in a single outbound direction.
pub struct Lambertian {
    color: Spectrum,
}

impl Lambertian {
    pub fn new(color: Spectrum) -> Lambertian {
        Lambertian { color }
    }
}

impl Scatter for Lambertian {
    fn reflection_model(&self) -> ReflectionModel {
        ReflectionModel::Diffuse
    }

    fn transmits(&self) -> bool {
        false
    }

    fn reflects(&self) -> bool {
        true
    }

    fn scatter(&self, outbound_direction: UnitVector3<f32>, inbound_direction: UnitVector3<f32>) -> Spectrum {
        self.color * 1.0f32 / PI
    }

    fn sample_scatter(&self, outbound_direction: UnitVector3<f32>) -> (Spectrum, UnitVector3<f32>) {
        todo!()
    }

    fn hemispherical_directional_scatter(&self, direction: UnitVector3<f32>) -> Spectrum {
        self.color
    }

    fn hemispherical_hemispherical_scatter(&self) -> Spectrum {
        self.color
    }

    fn scatter_probability(&self, outbound_direction: UnitVector3<f32>, inbound_direction: UnitVector3<f32>) -> f32 {
        todo!()
    }
}
