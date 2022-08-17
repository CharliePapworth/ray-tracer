pub mod direct_lighting_integrator;
use std::sync::mpsc::Receiver;

use nalgebra::Point3;

use crate::{scenes::Scene, film::FilmTile, multithreader::Instructions};


pub trait IntegrateTile {
    /// Renders the scene within the confines of the film tile. Accepts a callback function,
    /// which allows the process to be interrupted.
    fn render(scene: &Scene, tile: &mut FilmTile, receiver: Receiver<Instructions>) -> Option<Instructions>;
}

