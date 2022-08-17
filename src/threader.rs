pub mod multithreader;
use std::sync::mpsc::Receiver;

use enum_dispatch::enum_dispatch;
use nalgebra::Point3;

use crate::{scenes::Scene, film::{FilmTile, Film}};

use self::multithreader::{Multithreader, ThreadData, Instructions};

#[enum_dispatch(Coordinate)]
pub enum Threader {
    Multithreader(Multithreader)
}

#[enum_dispatch]
pub trait Coordinate {
    fn start_threads(&mut self, num_threads: usize, function: Box<dyn Fn(ThreadData, &mut FilmTile, Receiver<Instructions>) -> Option<Instructions> + Send + Sync + 'static>);
    fn output_image(&self) -> Film;
}
pub trait IntegrateTile {
    /// Renders the scene within the confines of the film tile. Accepts a callback function,
    /// which allows the process to be interrupted.
    fn render(scene: &Scene, tile: &mut FilmTile, receiver: Receiver<Instructions>) -> Option<Instructions>;
}

