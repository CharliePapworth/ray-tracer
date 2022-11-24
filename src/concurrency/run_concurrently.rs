use super::multithreader::Multithreader;
use crate::camera::*;
use super::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait RunConcurrently {
    fn do_work(&mut self);

    fn output_image(&self) -> Film;
}
