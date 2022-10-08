use std::ops::Mul;
use std::sync::mpsc::Receiver;

use crate::{
    film::{Film, FilmTile},
    light::{Emit, Light},
    material::{ReflectionModel, Scatter},
    multithreader::Instructions,
    multithreader::Multithreader,
    raytracing::{HitRecord, Ray},
    sampler::Sampler,
    scenes::Scene,
    spectrum::Spectrum,
    threader::{multithreader::Settings, Coordinate, Threader},
};

use super::Integrate;

pub struct DirectLightingIntegrator {
    max_depth: f32,
    sampler: Sampler,
    threader: Threader,
}

impl DirectLightingIntegrator {
    pub fn new(max_depth: f32, sampler: Sampler, threader: Threader) -> DirectLightingIntegrator {
        DirectLightingIntegrator {
            max_depth,
            sampler,
            threader,
        }
    }

    fn power_heuristic(
        light_weight: i32,
        light_probability_density: f32,
        material_weight: i32,
        material_probability_density: f32,
    ) -> f32 {
        let light_value = light_weight as f32 * light_probability_density;
        let material_value = material_weight as f32 * material_probability_density;
        (light_value * light_value) / (light_value * light_value + material_value * material_value)
    }

    fn sample_light_contribution(light: Light, scene: Scene, record: HitRecord) -> Spectrum {
        let emission = light.emit(record);
        let mut scattered_radiance: Spectrum;
        let probability_density: f32;

        if emission.probability_density > 0.0 && !emission.radiance.is_black() {
            scattered_radiance = record
                .surface_material
                .scatter(record.outbound_ray_direction, emission.direction)
                * f32::abs(emission.direction.dot(&record.surface_normal.cast::<f32>()));
            probability_density = record
                .surface_material
                .scatter_probability(record.outbound_ray_direction, emission.direction)
                * f32::abs(emission.direction.dot(&record.surface_normal.cast::<f32>()));
        }

        if !scattered_radiance.is_black() {
            //Check the light source is visible. If not, set the radiance to be black.
            let ray = Ray::new(emission.origin, emission.direction);
            if scene.hit(ray, 0.0, emission.time - record.time).is_some() {
                scattered_radiance = Spectrum::new(0.0);
            }
        }

        //Add contribution to reflected radiance
        match (scattered_radiance.is_black(), light.is_delta_distribution()) {
            (true, _) => return scattered_radiance,
            (false, false) => {
                return &scattered_radiance * &emission.radiance / emission.probability_density
            }
            (false, true) => {
                let weight = DirectLightingIntegrator::power_heuristic(
                    1,
                    emission.probability_density,
                    1,
                    probability_density,
                );
                return &scattered_radiance * &emission.radiance / emission.probability_density;
            }
        }
    }

    fn sample_material_contribution<L>(light: L, scene: Scene, record: HitRecord) -> Spectrum
    where
        L: Emit,
    {
        // let (mut scattered_radiance, inbound_direction) = record.surface_material.sample_scatter(record.outbound_ray_direction);
        // let mat = record.surface_material;
        // let material_scatter_probability = mat.scatter_probability(inbound_direction, record.outbound_ray_direction);
        // scattered_radiance *= inbound_direction.dot(&record.surface_normal).abs();
        // if mat.reflection_model() != ReflectionModel::Specular {
        //     //Account for light contributions along sampled direction
        //     let light_scatter_probability = light.emission_probability(record, inbound_direction);
        //     if light_scatter_probability == 0.0 {
        //         return Spectrum::new(0.0);
        //     } else {
        //         let weight = DirectLightingIntegrator::<S>::power_heuristic(1, material_scatter_probability, 1, light_scatter_probability);
        //         //Given a direction sampled by the material, we need to find out if the ray along that direction intersects this particular light source
        //         let ray = record.spawn_ray(inbound_direction);
        //         if let Some(light_intersection) = scene.hit(ray, 0.0, std::f32::MAX) {

        //         } else {
        //             scattered_radiance = Spectrum::new(0.0);
        //         }

        //     }
        // }

        //     todo!()
        Spectrum::new(0.0)
    }

    fn trace_ray(light: Light, scene: Scene, record: HitRecord) -> Spectrum {
        //No point sampling the material
        let light_contribution =
            DirectLightingIntegrator::sample_light_contribution(light, scene, record);
        if light.is_delta_distribution() {}
        todo!();
    }
}

impl Integrate for DirectLightingIntegrator {
    fn start(&self, num_threads: usize) {}

    fn change_scene(&self, new_scene: Scene) {}

    fn change_settings(&self, new_settings: Settings) {}

    fn output_image(&self) -> Film {
        self.threader.output_image()
    }
}