use crate::*;
use crate::camera::Camera;
use crate::film::FilmPixel;
use crate::image::CompositeImage;
use crate::image::Raster;
use crate::image::RaytracedImage;
use crate::scenes::Scene;
use crate::film::FilmTile;

use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;
use std::sync::mpsc::*;
use std::thread::JoinHandle;
use std::thread::park;
use crossbeam::*;
use crossbeam::queue::ArrayQueue;

#[derive (Copy, Clone)]
pub struct Settings {
    pub max_depth: i32,
    pub samples_per_pixel: usize
}

#[derive (Clone)]
pub struct ThreadData {
    pub settings: Settings,
    pub scene: Scene,
    pub id: i32
}

#[derive (Copy, Clone)]
pub enum Instructions {
    NewTask,
    Terminate
}

pub struct ThreadCoordinator {
    pub threads: Vec<Thread>,
    pub coordinator_to_thread_txs: Vec<Sender<Instructions>>,
    pub thread_data: Arc<RwLock<ThreadData>>,
    pub film_tiles_in_progress: ArrayQueue<Arc<Mutex<FilmTile>>>,
    pub film_tiles_finished: ArrayQueue<Arc<Mutex<FilmTile>>>,
    pub film_tiles: Vec<Arc<Mutex<FilmTile>>>
}

impl ThreadCoordinator {

    pub fn new(initial_settings: ThreadData, film_tiles: Vec<Arc<Mutex<FilmTile>>>) -> ThreadCoordinator {
        let thread_data = Arc::new(RwLock::new(initial_settings.clone()));
        let gui_to_thread_txs = vec![];

        let image_width = initial_settings.scene.camera.film.resoloution.0;
        let image_height = initial_settings.scene.camera.film.resoloution.1;
        let film_tiles_in_progress = ArrayQueue::<Arc<Mutex<FilmTile>>>::new(film_tiles.len()); 
        let film_tiles_finished = ArrayQueue::<Arc<Mutex<FilmTile>>>::new(film_tiles.len()); 
        let condvar = Condvar::new();
        let threads = vec![];
        for tile in film_tiles {
            film_tiles_in_progress.push(tile);
        }
        ThreadCoordinator {threads, coordinator_to_thread_txs: gui_to_thread_txs, thread_data, film_tiles_in_progress, film_tiles_finished, film_tiles }
    }

    pub fn spin_up(&mut self, num_threads: usize, function: Box<dyn Fn(ThreadData, (usize, usize)) -> FilmPixel + Send + Sync + 'static>) {
        
        let shareable_function = Arc::new(function);
        for i in 0..num_threads {
            let (gui_to_thread_tx, coordinator_to_thread_rx): (Sender<Instructions>, Receiver<Instructions>) = channel();
            let thread_data = Arc::clone(&self.thread_data);
            self.coordinator_to_thread_txs.push(gui_to_thread_tx);
            thread::spawn(|| run_thread(thread_data, self.film_tiles_in_progress, self.film_tiles_finished, coordinator_to_thread_rx, shareable_function.clone()));
            //self.threads.push(*thread_handle.thread());
        }
    }

    /// Returns the number of raytracing samples completed.
    pub fn get_progress(&self) -> i32 {
        let minimum_progress = i32::MAX;
        for tile_mutex in self.film_tiles {
            let tile = tile_mutex.lock().unwrap();
            minimum_progress = i32::min(minimum_progress, tile.samples);
        }
        minimum_progress
    }

    pub fn transmit_instructions(&mut self, instructions: Instructions) {
        let mut threads_to_remove = vec!();
        for (index, transmitter, ) in self.coordinator_to_thread_txs.iter().enumerate() {
             if transmitter.send(instructions).is_err() {
                threads_to_remove.push(index);
            }
        }
        for index in threads_to_remove {
            self.coordinator_to_thread_txs.remove(index);
        }
    }

    pub fn refresh_image(&mut self) {
        let mut settings = self.thread_data.read().unwrap().clone();
        settings.id += 1;
        self.transmit_instructions(Instructions::NewTask);
        //let mut image = self.image.0.lock().unwrap();
        //image.id += 1;
        //image.rasterization_samples = 0;
        //image.raytracing_samples = 0;
    }

    pub fn update_samples(&mut self, samples: usize) {
        let settings = self.thread_data.read().unwrap().clone();
        if samples < settings.settings.samples_per_pixel {
            self.refresh_image();
        } else {
            let mut settings = self.thread_data.write().unwrap();
            settings.settings.samples_per_pixel = samples;
        }
    }

    pub fn update_settings(&mut self, mut new_settings: ThreadData) {
        let mut settings = self.thread_data.write().unwrap();
        new_settings.id = settings.id + 1;
        *settings = new_settings;
        self.threads.iter().for_each(|thread| thread.unpark());
    }

    pub fn is_done(&self) -> bool {
        let settings = self.thread_data.read().unwrap();
        let image = self.image.0.lock().unwrap();
        image.image.raytrace.samples >= settings.settings.samples_per_pixel && image.rasterization_samples >= 1 && image.id == settings.id
    }

    pub fn output_image(&self) -> CompositeImage {
        let image = self.image.0.lock().unwrap();
        image.image.clone()
    }
}


 pub fn run_thread(thread_data: Arc<RwLock<ThreadData>>, film_tiles_in_progress: ArrayQueue<Arc<Mutex<FilmTile>>>, film_tiles_finished: ArrayQueue<Arc<Mutex<FilmTile>>>, coordinator_to_thread_rx: Receiver<Instructions>, function: Arc<Box<dyn Fn(ThreadData, (usize, usize)) -> FilmPixel + Send + Sync + 'static>>) {

    let mut terminated = false;

    while !terminated {
        let thread_data = thread_data.read().unwrap().clone();
        let desired_raytracing_samples = thread_data.settings.samples_per_pixel;
        if let Some(next_tile) = film_tiles_in_progress.pop() {
            let mut local_tile: FilmTile;
            //Inner scope to unlock mutex as quickly as possible.
            {
                local_tile = next_tile.lock().unwrap().clone();
            }

            function(thread_data, &mut local_tile);
            *next_tile.lock().unwrap() = local_tile;
        } else {
            park();
        }
    }
 }


pub fn raytrace<F>(settings: ThreadData, gui_to_thread_rx: &Receiver<Instructions>, tile: &mut FilmTile, function: &F) -> Option<Instructions>  where F: Fn(ThreadData, &mut FilmTile, (usize, usize)) {

    let image_height = settings.scene.camera.film.resoloution.1;
    let image_width = settings.scene.camera.film.resoloution.0;

    let cam = settings.scene.camera;
    for j in tile.bottom_left.1..tile.top_right.1 {
        for i in tile.bottom_left.1..tile.top_right.1 {
            if let Ok(instructions) = gui_to_thread_rx.try_recv() {
                return Some(instructions)
            }
            function(settings, tile, (i, j));
        }
    }
    None
}