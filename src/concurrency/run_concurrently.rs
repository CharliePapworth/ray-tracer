use std::sync::mpsc::Receiver;
use enum_dispatch::enum_dispatch;
use crate::camera::*;
use super::multithreader::*;

#[enum_dispatch]
pub trait RunConcurrently {
    fn do_work(&mut self);

    fn output_image(&self) -> Film;
}
