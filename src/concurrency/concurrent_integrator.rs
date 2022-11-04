use enum_dispatch::enum_dispatch;

use super::multithreader::Multithreader;
use super::run_concurrently::RunConcurrently;
use crate::film::Film;

#[enum_dispatch(RunConcurrently)]
pub enum ConcurrentIntegrator {
    Multithreader(Multithreader),
}
