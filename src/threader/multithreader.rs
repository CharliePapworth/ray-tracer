use crate::film::Film;
use crate::film::FilmTile;
use crate::scenes::Scene;
use crate::threader::Coordinate;
use crate::*;

use crossbeam::queue::ArrayQueue;
use std::sync::mpsc::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;
use std::thread::park;

#[derive(Copy, Clone)]
pub struct Settings {
    pub max_depth: i32,
    pub samples_per_pixel: i32,
}

#[derive(Clone)]
pub struct ThreadData {
    pub settings: Settings,
    pub scene: Scene,
    pub id: i32,
}

#[derive(Copy, Clone)]
pub enum Instructions {
    StartTask,
    StopTask,
    Terminate,
}

/// A wrapper around other integrators which multithreads them. Additionally
/// implements Update, which allows the gui to call it for incremental progress
/// updates.
pub struct Multithreader {
    pub threads: Vec<Thread>,
    pub coordinator_to_thread_txs: Vec<Sender<Instructions>>,
    pub thread_data: Arc<RwLock<ThreadData>>,
    pub film_tiles_in_progress: Arc<ArrayQueue<FilmTile>>,
    pub film_tiles_finished: Arc<ArrayQueue<FilmTile>>,
    pub film: Arc<Mutex<Film>>,
    pub function: Box<dyn Fn(ThreadData, &mut FilmTile, Receiver<Instructions>) -> Option<Instructions> + Send + Sync + 'static>,
}

impl Multithreader {
    pub fn new(
        initial_settings: ThreadData,
        num_threads: usize,
        film_tiles: Vec<FilmTile>,
        function: Box<dyn Fn(ThreadData, &mut FilmTile, Receiver<Instructions>) -> Option<Instructions> + Send + Sync + 'static>,
    ) -> Multithreader {
        let resoloution = initial_settings.scene.camera.resoloution;
        let film = Arc::new(Mutex::new(Film::new(resoloution)));
        let thread_data = Arc::new(RwLock::new(initial_settings.clone()));
        let gui_to_thread_txs = vec![];

        let film_tiles_in_progress = Arc::new(ArrayQueue::<FilmTile>::new(film_tiles.len()));
        let film_tiles_finished = Arc::new(ArrayQueue::<FilmTile>::new(film_tiles.len()));
        let threads = vec![];
        for tile in film_tiles {
            film_tiles_in_progress.push(tile);
        }
        Multithreader {
            threads,
            coordinator_to_thread_txs: gui_to_thread_txs,
            thread_data,
            film_tiles_in_progress,
            film_tiles_finished,
            film,
            function,
        }
    }

    /// Returns the number of raytracing samples completed.
    pub fn get_progress(&self) -> i32 {
        todo!()
    }

    pub fn transmit_instructions(&mut self, instructions: Instructions) {
        let mut threads_to_remove = vec![];
        for (index, transmitter) in self.coordinator_to_thread_txs.iter().enumerate() {
            if transmitter.send(instructions).is_err() {
                threads_to_remove.push(index);
            }
        }
        for index in threads_to_remove {
            self.coordinator_to_thread_txs.remove(index);
        }
    }

    pub fn send_update<F>(&self, update_function: F)
    where
        F: Fn(),
    {
        self.transmit_instructions(Instructions::StopTask);
        update_function();
        self.threads.iter().for_each(|thread| thread.unpark());
        self.transmit_instructions(Instructions::StartTask);
    }

    pub fn update_samples(&mut self, samples: i32) {
        self.send_update(|| {
            let thread_data = self.thread_data.write().unwrap();
            thread_data.settings.samples_per_pixel = samples;
        });
    }

    pub fn update_settings(&mut self, mut new_settings: ThreadData) {
        self.send_update(|| {
            let thread_data = self.thread_data.write().unwrap();
            *thread_data = new_settings;
        });
    }

    pub fn is_done(&self) -> bool {
        todo!()
    }
}

pub fn run_thread(
    thread_data: Arc<RwLock<ThreadData>>,
    film: Arc<Mutex<Film>>,
    film_tiles_in_progress: Arc<ArrayQueue<FilmTile>>,
    film_tiles_finished: Arc<ArrayQueue<FilmTile>>,
    multithreader_to_thread_rx: Receiver<Instructions>,
    function: Arc<Box<dyn Fn(ThreadData, &mut FilmTile, Receiver<Instructions>) -> Option<Instructions> + Send + Sync + 'static>>,
) {
    let mut terminated = false;
    let mut instructions: Instructions;

    while !terminated {
        let thread_data = *thread_data.read().unwrap();
        let desired_raytracing_samples = thread_data.settings.samples_per_pixel;
        if let Some(tile) = film_tiles_in_progress.pop() {
            if let Some(instructions) = function(thread_data, &mut tile, multithreader_to_thread_rx) {
                match instructions {
                    Instructions::StopTask => {
                        drop(thread_data);
                        multithreader_to_thread_rx.recv().expect("Main thread has hung up.");
                    }
                    Instructions::Terminate => {
                        terminated = true;
                        break;
                    }
                    Instructions::StartTask => {
                        panic!("A StartTask command was read before a StopTask command");
                    }
                }
            } else {
                film.lock().unwrap().merge_tile(&tile);
                if tile.samples < thread_data.settings.samples_per_pixel {
                    film_tiles_in_progress.push(tile);
                } else {
                    film_tiles_finished.push(tile);
                }
            }
        } else {
            park();
        }
    }
}

pub fn raytrace<F>(
    settings: ThreadData,
    gui_to_thread_rx: &Receiver<Instructions>,
    tile: &mut FilmTile,
    function: &F,
) -> Option<Instructions>
where
    F: Fn(ThreadData, &mut FilmTile, (usize, usize)),
{
    let image_height = settings.scene.camera.resoloution.1;
    let image_width = settings.scene.camera.resoloution.0;

    let cam = settings.scene.camera;
    for j in tile.bottom_left.y..tile.top_right.y {
        for i in tile.bottom_left.x..tile.top_right.x {
            if let Ok(instructions) = gui_to_thread_rx.try_recv() {
                return Some(instructions);
            }
            function(settings, tile, (i, j));
        }
    }
    None
}

impl Coordinate for Multithreader {
    fn start_threads(
        &mut self,
        num_threads: usize,
        function: Box<dyn Fn(ThreadData, &mut FilmTile, Receiver<Instructions>) -> Option<Instructions> + Send + Sync + 'static>,
    ) {
        let shareable_function = Arc::new(function);
        for i in 0..num_threads {
            let (gui_to_thread_tx, multithreader_to_thread_rx): (Sender<Instructions>, Receiver<Instructions>) = channel();
            let thread_data = Arc::clone(&self.thread_data);
            let film_tiles_in_progress = Arc::clone(&self.film_tiles_in_progress);
            let film_tiles_finished = Arc::clone(&self.film_tiles_finished);
            let film = self.film;
            self.coordinator_to_thread_txs.push(gui_to_thread_tx);
            thread::spawn(|| {
                run_thread(
                    thread_data,
                    film,
                    film_tiles_in_progress,
                    film_tiles_finished,
                    multithreader_to_thread_rx,
                    Arc::clone(&shareable_function),
                )
            });
        }
    }

    fn output_image(&self) -> Film {
        let image = self.film.lock().unwrap();
        image.clone()
    }
}
