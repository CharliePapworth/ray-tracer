use crate::camera::*;
use enum_dispatch::enum_dispatch;
use super::multithreader::{Multithreader};
use crate::ConcurrentIntegrator;

#[enum_dispatch]
pub trait RunConcurrently {
    fn do_work(&mut self);

    fn output_image(&self) -> Film;
}
