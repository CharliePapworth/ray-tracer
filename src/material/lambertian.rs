use std::f64::consts::PI;

use nalgebra::{Unit, Vector3};

use crate::{image::{Color}, raytracing::Ray, spectra::Spectrum};

use super::{Scatter, ReflectionModel};

#[derive(Clone, Default)]
///Specular materials, such as perfectly smooth surfaces, exhibit perfect specular reflection
/// and transmission: for an incident direction, all light is scattered in
/// a single outbound direction.
pub struct Lambertian {
    color: Spectrum
}


impl Lambertian {
    pub fn new(color: Spectrum) -> Lambertian {
        Lambertian {color}
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
 
     fn scatter(&self, outbound_direction: Vector3<f64>, inbound_direction: Vector3<f64>) -> Spectrum {
        self.color * 1.0 / PI;
     }
 
     fn opinionated_scatter(&self, outbound_direction: Vector3<f64>) -> Option<(Spectrum, Vector3<f64>)> {
        None
     }
 
     fn hemispherical_directional_scatter(&self, direction: Vector3<f64>) -> Spectrum {
        self.color
     }
 
     fn hemispherical_hemispherical_scatter(&self) -> Spectrum {
        self.color
     }
}

