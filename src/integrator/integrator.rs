use enum_dispatch::enum_dispatch;

use super::direct_lighting_integrator::DirectLightingIntegrator;

use crate::{
    light::{Light, Spectrum},
    raytracing::HitRecord,
    scenes::Scene,
};

#[enum_dispatch(Integrate)]
pub enum Integrator {
    DirectLightingIntegrator(DirectLightingIntegrator),
}

#[enum_dispatch]
pub trait Integrate {
    fn trace_ray(&self, light: Light, scene: Scene, record: HitRecord) -> Spectrum;
}
