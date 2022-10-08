use enum_dispatch::enum_dispatch;

use crate::{film::Film, scenes::Scene, threader::multithreader::Settings};

use self::direct_lighting_integrator::DirectLightingIntegrator;

pub mod direct_lighting_integrator;

#[enum_dispatch(Integrate)]
pub enum Integrator {
    DirectLightingIntegrator(DirectLightingIntegrator),
}

#[enum_dispatch]
pub trait Integrate {
    fn start(&self, num_threads: usize);
    fn change_scene(&self, new_scene: Scene);
    fn change_settings(&self, new_settings: Settings);
    fn output_image(&self) -> Film;
}
