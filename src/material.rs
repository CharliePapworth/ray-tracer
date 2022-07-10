pub mod lambertian;

use nalgebra::{Vector3, Unit};

use crate::image::{Color};
use crate::spectra::Spectrum;
use crate::vec::VecExtensionMethods;
use crate::{util::*, sampler};
use crate::raytracing::{HitRecord, Ray};
use crate::sampler::*;

use self::lambertian::Lambertian;

pub enum ReflectionModel {
    Diffuse,
    Glossy,
    Specular
}

///Implemented by materials. Encapsulates the interaction between light and a given material.
pub trait Scatter<'a>: Clone {

    ///Returns the reflection model of the material.
    fn reflection_model(&self) -> ReflectionModel;

    ///True if the material transmits light, and false otherwise.
    fn transmits(&self) -> bool;

    ///True if the material reflects light, and false otherwise.
    fn reflects(&self) -> bool;

    ///Returns the radiance (given by a spectrum) reflected from an incident ray of light along a given direction.
    fn scatter(&self, outbound_direction: Vector3<f64>, inbound_direction: Vector3<f64>) -> Spectrum<'a>;

    ///Returns the radiance (given by a spectrum) reflected from an incident ray of light along a given direction. The direction
    /// of the incident ray of light is chosen by the Material. This is useful in cases where the probability of choosing a
    /// direction from which light will be reflected in the desired outbound direction is low (e.g. as would be the case for a mirror).
    fn opinionated_scatter(&self, outbound_direction: Vector3<f64>) -> Option<(Spectrum<'a>, Vector3<f64>)>;

    ///The hemispherical-directional reflectance is a 2D function that gives the total reflection in a given direction due to constant
    ///  illumination over the hemisphere, or, equivalently, total reflection over the hemisphere due to light from a given direction.
    fn hemispherical_directional_scatter(&self, direction: Vector3<f64>) -> Spectrum<'a>;

    ///The hemispherical-hemispherical reflectance of a surface is a spectral value that gives the fraction of incident 
    /// light reflected by a surface when the incident light is the same from all directions. 
    fn hemispherical_hemispherical_scatter(&self) -> Spectrum<'a>;
}


#[derive(Clone)]
pub enum Material<'a>{
    Lambertian(Lambertian<'a>),
}

impl<'a> Scatter<'a> for Material<'a> {
    fn reflection_model(&self) -> ReflectionModel {
        todo!()
    }

    fn transmits(&self) -> bool {
        todo!()
    }

    fn reflects(&self) -> bool {
        todo!()
    }

    fn scatter(&self, outbound_direction: Vector3<f64>, inbound_direction: Vector3<f64>) -> Spectrum<'a> {
        todo!()
    }

    fn opinionated_scatter(&self, outbound_direction: Vector3<f64>) -> Option<(Spectrum<'a>, Vector3<f64>)> {
        todo!()
    }

    fn hemispherical_directional_scatter(&self, direction: Vector3<f64>) -> Spectrum<'a> {
        todo!()
    }

    fn hemispherical_hemispherical_scatter(&self) -> Spectrum<'a> {
        todo!()
    }
}

impl<'a> Material<'a> {
    pub fn new_lambertian(color: &'a Spectrum) -> Material<'a> {
        Material::Lambertian(Lambertian::new(color))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{GeometricPrimitive};
    use crate::raytracing::{Hit, Ray};
    use crate::nalgebra::{Point3};

    
}