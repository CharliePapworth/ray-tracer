use nalgebra::Point3;

use crate::spectrum::Spectrum;

pub struct EmissionData {
    probability_density: f32,
    radiance: Spectrum,
    origin: Point3<f32>,
    intersection_point: Point3<f32>
}

impl EmissionData {
    pub fn new(probability_density: f32, spectrum: Spectrum, origin: Point3<f32>, intersection_point: Point3<f32>) -> EmissionData {
        EmissionData { probability_density, radiance: spectrum, origin, intersection_point}
    }
}

pub enum Emission {
    SinglePoint(EmissionData),
    SingleDirection(EmissionData),
    AreaLight(EmissionData),
    InfiniteLight(EmissionData)
}

pub trait Light {
    fn emit(&self, point_in_scene: Point3<f32>) -> Emission;
    fn power(&self) -> Spectrum;
}

pub struct PointLight {
    spectrum: Spectrum,
    position: Point3<f32>
}

impl Light for PointLight {
    fn emit(&self, point_in_scene: Point3<f32>) -> Emission {
        let origin = self.position;
        let radiance = self.spectrum / (point_in_scene - self.position).norm_squared();
        let probability_density = 1.0;
        let origin = self.position;
        let emission_data = EmissionData::new(probability_density, radiance, origin, point_in_scene);
        Emission::SinglePoint(emission_data)
    }

    fn power(&self) -> Spectrum {
        todo!()
    }
}