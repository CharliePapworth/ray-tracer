pub mod multithreader;
pub mod direct_lighting_integrator;

use nalgebra::Point3;

use crate::{scenes::Scene, film::FilmTile};


pub trait IntegrateTile {
    /// Renders the scene within the confines of the film tile. Accepts a callback function,
    /// which allows the process to be interrupted.
    fn render<C>(scene: &Scene, tile: &mut FilmTile, interrupt_callback: C) where C: Fn() -> bool;

}

