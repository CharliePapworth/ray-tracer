use crate::*;
use crate::camera::Camera;
use crate::image::CompositeImage;
use crate::image::Raster;
use crate::image::RaytracedImage;
use crate::scenes::Scene;

use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::mpsc::*;
use std::thread;

#[derive (Copy, Clone)]
pub struct ImageSettings {
    pub image_width: usize,
    pub image_height: usize,
}

#[derive (Clone)]
pub struct TrackedCompositeImage {
    pub image: CompositeImage,
    pub id: i32,
    pub rasterization_samples: usize,
}

impl TrackedCompositeImage {
    pub fn new (image_width: usize, image_height: usize, id: i32) -> TrackedCompositeImage {
        let image = CompositeImage::new(image_width, image_height);
        TrackedCompositeImage { image, id, rasterization_samples: 0 }
    }

    pub fn from_image(image: CompositeImage, rasterization_samples: usize, raytracing_samples: usize, id: i32) -> TrackedCompositeImage {
        TrackedCompositeImage { image, id, rasterization_samples}
    }

    pub fn is_finished_rasterizing(&self, desired_id: i32, desired_rasterization_samples: usize) -> bool{
        self.id >= desired_id && self.rasterization_samples >= desired_rasterization_samples
    }

    pub fn is_finished_raytracing(&self, desired_id: i32, desired_raytracing_samples: usize) -> bool{
        self.id >= desired_id && self.image.raytrace.samples >= desired_raytracing_samples
    }

    pub fn add_outline(&mut self, outline: Raster, id: i32) {
        if self.image.image_height != outline.image.image_height || self.image.image_width != outline.image.image_width {
            panic!("Image dimensions do not match");
        }

        if id > self.id {
            self.rasterization_samples = 0;
            self.image.raytrace.samples = 0;
            self.id = id;
        }

        if self.rasterization_samples == 0 {
            self.image.outline = outline;
            self.rasterization_samples = 1;
        }
    }

    pub fn add_raytraced_sample(&mut self, sample: RaytracedImage, id: i32, max_samples: usize) {
        if self.image.image_height != sample.image.image_height || self.image.image_width != sample.image.image_width {
            panic!("Image dimensions do not match");
        }

        if self.image.raytrace.samples >= max_samples {
            return;
        }

        if id > self.id {
            self.rasterization_samples = 0;
            self.image.raytrace.samples = 1;
            self.id = id;
            self.image.raytrace = sample;
        } else if self.image.raytrace.samples == 0 && id == self.id {
            self.image.raytrace = sample;
            self.image.raytrace.samples = 1;
        } else if self.image.raytrace.samples > 0 && id == self.id {
            self.image.raytrace += &sample;
        }
    }
}



#[derive (Copy, Clone)]
pub struct RayTraceSettings {
    pub max_depth: i32,
    pub samples_per_pixel: usize
}

#[derive (Clone)]
pub struct GlobalSettings {
    pub raytrace_settings: RayTraceSettings,
    pub image_settings: ImageSettings,
    pub camera: Camera,
    pub scene: Scene,
    pub id: i32
}

#[derive (Copy, Clone)]
pub struct LocalSettings {
    pub rasterizing: bool,
    pub raytracing: bool
}

#[derive (Copy, Clone)]
pub enum Instructions {
    NewTask,
    Terminate
}


pub struct ThreadCoordinator {
    pub gui_to_thread_txs: Vec<Sender<Instructions>>,
    pub global_settings: Arc<RwLock<GlobalSettings>>,
    pub local_settings: Vec<Arc<RwLock<LocalSettings>>>,
    pub image: Arc<(Mutex<TrackedCompositeImage>, Condvar)>
}

impl ThreadCoordinator {

    pub fn new(initial_settings: GlobalSettings) -> ThreadCoordinator {
        let global_settings = Arc::new(RwLock::new(initial_settings.clone()));
        let local_settings = vec![];
        let gui_to_thread_txs = vec![];

        let image_height = initial_settings.image_settings.image_height;
        let image_width = initial_settings.image_settings.image_width;
        let image = Arc::new((Mutex::new(TrackedCompositeImage::new(image_width, image_height, 0)), Condvar::new()));

        ThreadCoordinator {gui_to_thread_txs, global_settings, local_settings, image }
    }

    pub fn spin_up(&mut self, num_threads: usize) {
        
        for i in 0..num_threads {
            let mut raytracing = true;
            let mut rasterizing = false;
            if i == 0 {
                raytracing = false;
                rasterizing = true;
            }
            let thread_settings = LocalSettings { raytracing, rasterizing };
            self.local_settings.push(Arc::new(RwLock::new(thread_settings)));
            let (gui_to_thread_tx, gui_to_thread_rx): (Sender<Instructions>, Receiver<Instructions>) = channel();
            let image = Arc::clone(&self.image);
            let global_settings = Arc::clone(&self.global_settings);
            let local_settings = self.local_settings[i].clone();
            self.gui_to_thread_txs.push(gui_to_thread_tx);
            thread::spawn(move || run_thread(global_settings, local_settings, image,  gui_to_thread_rx));
        }
    }

    /// Returns the number of raytracing samples completed.
    pub fn get_progress(&self) -> usize {
        let image = self.image.0.lock().unwrap();
        image.image.raytrace.samples
    }

    pub fn transmit_instructions(&mut self, instructions: Instructions) {
        let mut threads_to_remove = vec!();
        for (index, transmitter, ) in self.gui_to_thread_txs.iter().enumerate() {
            let thread =  self.local_settings[index].read().unwrap();
            if transmitter.send(instructions).is_err() {
                threads_to_remove.push(index);
            }
        }
        for index in threads_to_remove {
            self.gui_to_thread_txs.remove(index);
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
        if samples < settings.raytrace_settings.samples_per_pixel {
            self.refresh_image();
        } else {
            let mut settings = self.global_settings.write().unwrap();
            settings.raytrace_settings.samples_per_pixel = samples;
        }
    }

    pub fn update_settings(&mut self, mut new_settings: GlobalSettings) {
        let mut settings = self.global_settings.write().unwrap();
        new_settings.id = settings.id + 1;
        *settings = new_settings;
        self.image.1.notify_all();
    }

    pub fn is_done(&self) -> bool {
        let settings = self.global_settings.read().unwrap();
        let image = self.image.0.lock().unwrap();
        image.image.raytrace.samples >= settings.raytrace_settings.samples_per_pixel && image.rasterization_samples >= 1 && image.id == settings.id
    }

    pub fn output_image(&self) -> CompositeImage {
        let image = self.image.0.lock().unwrap();
        image.image.clone()
    }
}


 pub fn run_thread(global_settings: Arc<RwLock<GlobalSettings>>, local_settings: Arc<RwLock<LocalSettings>>, image_data: Arc<(Mutex<TrackedCompositeImage>, Condvar)>, coordinator_to_thread_rx: Receiver<Instructions>) {

    let mut terminated = false;

    while !terminated {
        let global_settings = global_settings.read().unwrap().clone();
        let settings_id = global_settings.id;
        let local_settings = local_settings.read().unwrap().clone();
        let cond_var = &image_data.1;
        let desired_raytracing_samples = global_settings.raytrace_settings.samples_per_pixel;
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

 pub fn outline(settings: GlobalSettings) -> Option<Raster>{
    
    let image_height = settings.image_settings.image_height;
    let image_width = settings.image_settings.image_width;
    let mut raster = Raster::new(image_width, image_height);

    raster = rasterizing::rasterize(raster, settings.camera, settings.scene.background, &settings.scene.rasterization_primitives);
    Some(raster)
 }


pub fn raytrace(settings: GlobalSettings, gui_to_thread_rx: &Receiver<Instructions>) -> Option<RaytracedImage> {

    let image_height = settings.image_settings.image_height;
    let image_width = settings.image_settings.image_width;

    let mut raytrace = RaytracedImage::new(image_width, image_height);
    raytrace.samples = 1;
    let cam = settings.camera;
    for j in 0..image_height {
        for i in 0..image_width {
            if let Ok(instructions) = gui_to_thread_rx.try_recv() {
                match instructions {
                    Instructions::Terminate => return None,
                    Instructions::NewTask => return None,
                }
            }
            raytrace = raytracing::raytrace_pixel(raytrace, cam, settings.scene.background, &settings.scene.raytracing_primitives, settings.raytrace_settings.max_depth, (i, j));
        }
    }
    Some(raytrace)
}