use crate::*;
use crate::camera::Camera;
use crate::image::CompositeImage;
use crate::image::Raster;
use crate::image::RaytracedImage;
use crate::scenes::Scene;
use crate::film::FilmTile;

use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::mpsc::*;
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
    pub coordinator_to_thread_txs: Vec<Sender<Instructions>>,
    pub global_settings: Arc<RwLock<ThreadData>>,
    pub film_tiles_queue: ArrayQueue<Arc<Mutex<FilmTile>>>,
}

impl ThreadCoordinator {

    pub fn new(initial_settings: ThreadData, film_tiles: Vec<Arc<Mutex<FilmTile>>>) -> ThreadCoordinator {
        let global_settings = Arc::new(RwLock::new(initial_settings.clone()));
        let local_settings = vec![];
        let gui_to_thread_txs = vec![];

        let image_width = initial_settings.scene.camera.film.resoloution.0;
        let image_height = initial_settings.scene.camera.film.resoloution.1;
        let film_tiles_queue = ArrayQueue::<Arc<Mutex<FilmTile>>>::new(film_tiles.len()); 
        
        for tile in film_tiles {
            film_tiles_queue.push(tile);
        }
        ThreadCoordinator {coordinator_to_thread_txs: gui_to_thread_txs, global_settings, film_tiles_queue }
    }

    pub fn spin_up<F>(&mut self, num_threads: usize, function: &F) where F: Fn(ThreadData) {
        
        for i in 0..num_threads {
            let (gui_to_thread_tx, gui_to_thread_rx): (Sender<Instructions>, Receiver<Instructions>) = channel();
            let global_settings = Arc::clone(&self.global_settings);
            self.coordinator_to_thread_txs.push(gui_to_thread_tx);
            thread::scope(|_| run_thread(global_settings, local_settings, image,  gui_to_thread_rx, function));
        }
    }

    /// Returns the number of raytracing samples completed.
    pub fn get_progress(&self) -> usize {
        let image = self.image.0.lock().unwrap();
        image.image.raytrace.samples
    }

    pub fn transmit_instructions(&mut self, instructions: Instructions) {
        let mut threads_to_remove = vec!();
        for (index, transmitter, ) in self.coordinator_to_thread_txs.iter().enumerate() {
            let thread =  self.local_settings[index].read().unwrap();
            if transmitter.send(instructions).is_err() {
                threads_to_remove.push(index);
            }
        }
        for index in threads_to_remove {
            self.coordinator_to_thread_txs.remove(index);
        }
    }

    pub fn refresh_image(&mut self) {
        let mut settings = self.global_settings.read().unwrap().clone();
        settings.id += 1;
        self.transmit_instructions(Instructions::NewTask);
        //let mut image = self.image.0.lock().unwrap();
        //image.id += 1;
        //image.rasterization_samples = 0;
        //image.raytracing_samples = 0;
    }

    pub fn update_samples(&mut self, samples: usize) {
        let settings = self.global_settings.read().unwrap().clone();
        if samples < settings.settings.samples_per_pixel {
            self.refresh_image();
        } else {
            let mut settings = self.global_settings.write().unwrap();
            settings.settings.samples_per_pixel = samples;
        }
    }

    pub fn update_settings(&mut self, mut new_settings: ThreadData) {
        let mut settings = self.global_settings.write().unwrap();
        new_settings.id = settings.id + 1;
        *settings = new_settings;
        self.image.1.notify_all();
    }

    pub fn is_done(&self) -> bool {
        let settings = self.global_settings.read().unwrap();
        let image = self.image.0.lock().unwrap();
        image.image.raytrace.samples >= settings.settings.samples_per_pixel && image.rasterization_samples >= 1 && image.id == settings.id
    }

    pub fn output_image(&self) -> CompositeImage {
        let image = self.image.0.lock().unwrap();
        image.image.clone()
    }
}


 pub fn run_thread<F>(global_settings: Arc<RwLock<ThreadData>>, local_settings: Arc<RwLock<LocalSettings>>, image_data: Arc<(Mutex<TrackedCompositeImage>, Condvar)>, coordinator_to_thread_rx: Receiver<Instructions>, function: &F) where F: Fn(ThreadData) {

    let mut terminated = false;

    while !terminated {
        let global_settings = global_settings.read().unwrap().clone();
        let settings_id = global_settings.id;
        let local_settings = local_settings.read().unwrap().clone();
        let cond_var = &image_data.1;
        let desired_raytracing_samples = global_settings.settings.samples_per_pixel;
        let desired_rasterization_samples = 1;
        let image = image_data.0.lock().unwrap();
        if !image.is_finished_rasterizing(settings_id, desired_rasterization_samples) && local_settings.rasterizing == true {
            drop(image);
            if let Some(contribution) = outline(global_settings) {
                let mut image = image_data.0.lock().unwrap();
                image.add_outline(contribution, settings_id);
            }
            
       } else if !image.is_finished_raytracing(settings_id, desired_raytracing_samples) && local_settings.raytracing == true {
            drop(image);
            if let Some(contribution) = raytrace(global_settings, &coordinator_to_thread_rx) {
                let mut image = image_data.0.lock().unwrap();
                image.add_raytraced_sample(contribution, settings_id, desired_raytracing_samples);
            }
        } else {
            cond_var.wait(image);
        }
    }
 }

 pub fn outline(settings: ThreadData) -> Option<Raster>{
    
    let image_height = settings.scene.camera.film.resoloution.1;
    let image_width = settings.scene.camera.film.resoloution.0;
    let mut raster = Raster::new(image_width, image_height);

    raster = rasterizing::rasterize(raster, settings.scene.camera, &settings.scene.rasterization_primitives);
    Some(raster)
 }


pub fn raytrace(settings: ThreadData, gui_to_thread_rx: &Receiver<Instructions>) -> Option<RaytracedImage> {

    let image_height = settings.scene.camera.film.resoloution.1;
    let image_width = settings.scene.camera.film.resoloution.0;

    let mut raytrace = RaytracedImage::new(image_width, image_height);
    raytrace.samples = 1;
    let cam = settings.scene.camera;
    for j in 0..image_height {
        for i in 0..image_width {
            if let Ok(instructions) = gui_to_thread_rx.try_recv() {
                match instructions {
                    Instructions::Terminate => return None,
                    Instructions::NewTask => return None,
                }
            }
            raytrace = raytracing::raytrace_pixel(raytrace, cam, &settings.scene.raytracing_primitives, settings.settings.max_depth, (i, j));
        }
    }
    Some(raytrace)
}