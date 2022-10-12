use nalgebra::Vector3;

use crate::spectrum::spectrum::Spectrum;

use super::lambertian::Lambertian;

#[derive(Copy, Clone, PartialEq)]
pub enum ReflectionModel {
    Diffuse,
    Glossy,
    Specular,
}

///Implemented by materials. Encapsulates the interaction between light and a
/// given material.
pub trait Scatter: Clone {
    ///Returns the reflection model of the material.
    fn reflection_model(&self) -> ReflectionModel;

    ///True if the material transmits light, and false otherwise.
    fn transmits(&self) -> bool;

    ///True if the material reflects light, and false otherwise.
    fn reflects(&self) -> bool;

    ///Returns the radiance (given by a spectrum) reflected from an incident
    /// ray of light along a given direction.
    fn scatter(&self, outbound_direction: Vector3<f32>, inbound_direction: Vector3<f32>) -> Spectrum;

    ///Returns the probability that light is reflected in the outbound
    /// direction after arriving at the inbound direction;
    fn scatter_probability(&self, outbound_direction: Vector3<f32>, inbound_direction: Vector3<f32>) -> f32;

    ///Returns the radiance (given by a spectrum) reflected from an incident
    /// ray of light along a samppled direction. The direction
    /// of the incident ray of light is chosen by the Material. This is useful
    /// in cases where the probability of choosing a direction from which
    /// light will be reflected in the desired outbound direction is low (e.g.
    /// as would be the case for a mirror).
    fn sample_scatter(&self, outbound_direction: Vector3<f32>) -> (Spectrum, Vector3<f32>);

    ///The hemispherical-directional reflectance is a 2D function that gives
    /// the total reflection in a given direction due to constant
    ///  illumination over the hemisphere, or, equivalently, total reflection
    /// over the hemisphere due to light from a given direction.
    fn hemispherical_directional_scatter(&self, direction: Vector3<f32>) -> Spectrum;

    ///The hemispherical-hemispherical reflectance of a surface is a spectral
    /// value that gives the fraction of incident light reflected by a
    /// surface when the incident light is the same from all directions.
    fn hemispherical_hemispherical_scatter(&self) -> Spectrum;
}

#[derive(Copy, Clone)]
///Encompassess all material types. Achieves polymorphism via static dispatch.
pub enum Material {
    Lambertian(Lambertian),
}

impl Scatter for Material {
    fn reflection_model(&self) -> ReflectionModel {
        todo!()
    }

    fn transmits(&self) -> bool {
        todo!()
    }

    fn reflects(&self) -> bool {
        todo!()
    }

    fn scatter(&self, outbound_direction: Vector3<f32>, inbound_direction: Vector3<f32>) -> Spectrum {
        todo!()
    }

    fn sample_scatter(&self, outbound_direction: Vector3<f32>) -> (Spectrum, Vector3<f32>) {
        todo!()
    }

    fn hemispherical_directional_scatter(&self, direction: Vector3<f32>) -> Spectrum {
        todo!()
    }

    fn hemispherical_hemispherical_scatter(&self) -> Spectrum {
        todo!()
    }

    fn scatter_probability(&self, outbound_direction: Vector3<f32>, inbound_direction: Vector3<f32>) -> f32 {
        todo!()
    }
}

impl Material {
    pub fn new_lambertian(color: Spectrum) -> Material {
        Material::Lambertian(Lambertian::new(color))
    }
}

#[cfg(test)]
mod tests {}
