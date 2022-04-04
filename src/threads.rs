use crate::*;
use crate::vec::*;
use crate::traceable::*;
use crate::util::*;

use std::sync::Arc;
use std::sync::Barrier;
use std::sync::RwLock;
use std::sync::mpsc::*;
use std::thread;

#[derive (Copy, Clone)]
pub struct ImageSettings {
    pub image_width: usize,
    pub image_height: usize,
}



#[derive (Copy, Clone)]
pub struct RayTraceSettings {
    pub max_depth: i32,
    pub samples_per_pixel: usize
}

#[derive (Clone)]
pub struct Settings {
    pub raytrace_settings: RayTraceSettings,
    pub image_settings: ImageSettings,
    pub camera_settings: CameraSettings,
    pub scene: SceneData,
    pub draw_mode: DrawMode,
    pub id: i32
}

#[derive (Clone)]
pub struct ThreadSettings{
    draw_mode: DrawMode,
    id: usize
}

pub enum StatusUpdate {
    Paused(ThreadSettings),
    Awake(ThreadSettings),
    Running(ImageData)
}

#[derive (Copy, Clone)]
pub enum Progress {
    Complete,
    Interrupted(Instructions)
}

#[derive (Copy, Clone)]
pub enum Priority {
    Now,
    Next,
    Eventually
}

#[derive (Copy, Clone)]
pub enum Instructions {
    NewTask,
    Pause,
    Terminate
}

#[derive (Copy, Clone)]
pub struct Message {
    pub instructions: Instructions,
    pub priority: Priority,
}

#[derive (Copy, Clone, PartialEq)]
pub enum DrawMode {
    Raytrace,
    Rasterize
}
#[derive (Clone)]
pub struct ImageData{
    pub image: Image,
    pub draw_mode: DrawMode,
    pub id: i32
}

pub struct ThreadCoordinator {
    pub thread_to_gui_tx: Sender<StatusUpdate>,
    pub threads_to_gui_rx: Receiver<StatusUpdate>,
    pub gui_to_thread_txs: Vec<Sender<Message>>,
    pub global_settings: Arc<RwLock<Settings>>,
    pub local_settings: Vec<Arc<RwLock<ThreadSettings>>>,
    pub paused_threads: Vec<ThreadSettings>,
    pub raytracing_threads: i32,
    pub rasterizing_threads: i32,
    pub raytracing_samples: usize,
    pub rasterizing_samples: usize,
    pub image_id: i32,
    pub image: Image
}

impl ThreadCoordinator {

    pub fn new(initial_settings: Settings) -> ThreadCoordinator {
        let global_settings = Arc::new(RwLock::new(initial_settings.clone()));
        let (thread_to_gui_tx, threads_to_gui_rx): (Sender<StatusUpdate>, Receiver<StatusUpdate>) = channel();
        let local_settings = vec![];
        let gui_to_thread_txs = vec![];
        let raytracing_threads = 0;
        let rasterizing_threads = 0;
        let image_id = 0;

        let image_height = initial_settings.image_settings.image_height;
        let image_width = initial_settings.image_settings.image_width;
        let image = Image::new(image_width, image_height);
        let raytracing_samples = 0;
        let paused_threads = vec![];
        let rasterizing_samples = 0;

        ThreadCoordinator { thread_to_gui_tx, threads_to_gui_rx, gui_to_thread_txs, global_settings, local_settings, paused_threads, raytracing_threads, rasterizing_threads, raytracing_samples, rasterizing_samples, image_id, image }
    }

    pub fn spin_up(&mut self, num_raytracing_threads: usize, num_rasterizing_threads: usize) {
        
        let num_threads = num_rasterizing_threads + num_raytracing_threads;
        let barrier = Arc::new(Barrier::new((num_threads) as usize));
        
        for i in 0..num_rasterizing_threads {
            let thread_settings = ThreadSettings {draw_mode: DrawMode::Rasterize, id: i};
            self.local_settings.push(Arc::new(RwLock::new(thread_settings)));
        }

        for i in num_rasterizing_threads..num_threads {
            let thread_settings = ThreadSettings {draw_mode: DrawMode::Raytrace, id: i};
            self.local_settings.push(Arc::new(RwLock::new(thread_settings)));
        }

        for i in 0..num_threads {
            let (gui_to_thread_tx, gui_to_thread_rx): (Sender<Message>, Receiver<Message>) = channel();
            let barrier_clone = Arc::clone(&barrier);
            let thread_to_gui_tx = self.thread_to_gui_tx.clone();
            let global_settings = Arc::clone(&self.global_settings);
            let local_settings = self.local_settings[i].clone();
            self.gui_to_thread_txs.push(gui_to_thread_tx);
            thread::spawn(move || run_thread(global_settings, local_settings, thread_to_gui_tx,  gui_to_thread_rx, barrier_clone));
        }
    }

    pub fn transmit_message(&mut self, message: Message, recipient: DrawMode) {
        let mut threads_to_remove = vec!();
        for (index, transmitter, ) in self.gui_to_thread_txs.iter().enumerate() {
            let thread =  self.local_settings[index].read().unwrap();
            if thread.draw_mode == recipient {
                if transmitter.send(message).is_err() {
                    threads_to_remove.push(index);
                }
            }
        }
        for index in threads_to_remove {
            self.gui_to_thread_txs.remove(index);
        }
    }

    pub fn refresh_image(&mut self) {
        self.raytracing_samples = 0;
        self.rasterizing_samples = 0;
        self.wake_threads();
    }

    pub fn update_settings(&mut self, mut new_settings: Settings, priority: Priority) {
        new_settings.id += 1;
        let mut settings = self.global_settings.write().unwrap();
        *settings = new_settings;
        std::mem::drop(settings);
        self.refresh_image();
    }

    pub fn update_scene(&mut self, new_scene: SceneData, priority: Priority)  {
        let mut settings = self.global_settings.write().unwrap();
        settings.scene = new_scene;
        settings.id += 1;
        std::mem::drop(settings);
        self.refresh_image();
    }

    pub fn update_camera(&mut self, new_camera: CameraSettings, priority: Priority)  {
        let mut settings = self.global_settings.write().unwrap();
        settings.camera_settings = new_camera;
        settings.id += 1;
        std::mem::drop(settings);
        self.refresh_image();
    }

    pub fn is_done(&self) -> bool {
        let settings = self.global_settings.read().unwrap();
        self.raytracing_samples == settings.raytrace_settings.samples_per_pixel && self.rasterizing_samples == 1 && self.image_id == settings.id
    }
    
    pub fn wake_threads(&mut self) {
        let settings = self.global_settings.read().unwrap();
        let mut threads_to_remove = vec!();
        let message = Message { instructions: Instructions::NewTask, priority: Priority::Now };
        for thread in &self.paused_threads {
            if self.raytracing_samples < settings.raytrace_settings.samples_per_pixel && thread.draw_mode == DrawMode::Raytrace
            || self.rasterizing_samples < 1 && thread.draw_mode == DrawMode::Rasterize {
                let transmitter = &self.gui_to_thread_txs[thread.id];
                if transmitter.send(message).is_err() {
                    threads_to_remove.push(thread.id);
                }
            }
        }
        for index in threads_to_remove {
            self.gui_to_thread_txs.remove(index);
        }
    }

    pub fn sleep_threads(&mut self, recipient: DrawMode) {
        for thread in &self.paused_threads {
            if thread.draw_mode == recipient {
                return
            }
        }
        self.transmit_message(Message {instructions: Instructions::Pause, priority: Priority::Now}, recipient);
    }


    pub fn update_image(&mut self) {
        loop {
            let message_result = self.threads_to_gui_rx.try_recv();
            if let Ok(message) = message_result {
                let settings = self.global_settings.read().unwrap().clone();
                if let StatusUpdate::Running(image_data) = message {
                    if image_data.id > self.image_id {  
                        self.image_id = image_data.id;
                        self.image= image_data.image;
                        match image_data.draw_mode {
                            DrawMode::Rasterize => {
                                self.raytracing_samples = 0;
                                self.rasterizing_samples = 1;
                            }

                            DrawMode::Raytrace => {
                                self.rasterizing_samples = 0;
                                self.raytracing_samples = 1;
                            }
                        }
                    }
                    else if image_data.id == self.image_id {
                        match image_data.draw_mode {
                            DrawMode::Raytrace => {
                                if self.raytracing_samples < settings.raytrace_settings.samples_per_pixel {
                                    self.image = self.image.clone() + image_data.image;
                                    self.raytracing_samples += 1;
                                } 

                                if self.raytracing_samples >= settings.raytrace_settings.samples_per_pixel {
                                    self.sleep_threads(DrawMode::Raytrace);
                                }
                            }

                            DrawMode::Rasterize => {
                                if self.rasterizing_samples < 1 {
                                    self.image = self.image.clone() + image_data.image;
                                }

                            }
                        }
                    }
                }  else if let StatusUpdate::Paused(thread) = message {
                    self.paused_threads.push(thread);
                } else if let StatusUpdate::Awake(awake_thread) = message {
                    self.paused_threads.retain(|thread| thread.id != awake_thread.id);
                }
            } else {
                return
            }
        } 
    }
}


 pub fn run_thread(global_settings: Arc<RwLock<Settings>>, local_settings: Arc<RwLock<ThreadSettings>>, thread_to_gui_tx: Sender<StatusUpdate>, gui_to_thread_rx: Receiver<Message>, barrier: Arc<Barrier>) {

    let mut terminated = false;
    let mut paused = false; 

    while !terminated {
        if !paused {
            let global_settings = global_settings.read().unwrap().clone();
            let local_settings = local_settings.read().unwrap().clone();
            match local_settings.draw_mode {
                DrawMode::Raytrace => {
                    raytrace(global_settings, thread_to_gui_tx.clone() , &gui_to_thread_rx, &mut paused);
                }
                DrawMode::Rasterize => {
                    rasterize(global_settings, thread_to_gui_tx.clone() , &gui_to_thread_rx, &mut paused);
                    paused = true;
                }
            }
        }
        
        else {
            thread_to_gui_tx.send(StatusUpdate::Paused(local_settings.read().unwrap().clone()));
            let message = gui_to_thread_rx.recv();
            match message {
                Ok(message) => {
                    match message.instructions {
                        Instructions::Terminate => terminated = true,
                        Instructions::Pause => {}
                        Instructions::NewTask => {
                            thread_to_gui_tx.send(StatusUpdate::Awake(local_settings.read().unwrap().clone()));
                            paused = false;
                        }
                    }
                }
                Err(_) => terminated = true
            }
        }
    }
 }

 pub fn rasterize(settings: Settings, thread_to_gui_tx: Sender<StatusUpdate>, gui_to_thread_rx: &Receiver<Message>, paused: &mut bool) {
    
    let image_height = settings.image_settings.image_height;
    let image_width = settings.image_settings.image_width;
    let mut image = Image::new(image_width, image_height);
    let id = settings.id;

    let cam = Camera::new(settings.camera_settings);
    if let Some(pixels) = settings.scene.geometric_primitives.outline(&cam) {
        for pixel in pixels {
            let pixel_index = (image_height - pixel[1] - 1) * image_width + pixel[0];
            image[pixel_index] = Pixel::Outline;
        }
    }

    let output = ImageData{image, draw_mode: DrawMode::Rasterize, id};
    thread_to_gui_tx.send(StatusUpdate::Running(output));
 }


pub fn raytrace(settings: Settings, thread_to_gui_tx: Sender<StatusUpdate>, gui_to_thread_rx: &Receiver<Message>, paused: &mut bool) {

    let image_height = settings.image_settings.image_height;
    let image_width = settings.image_settings.image_width;
    let id = settings.id;

    let mut image = Image::new(image_width, image_height);
    let cam = Camera::new(settings.camera_settings);
    for j in 0..image_height{
        for i in 0..image_width{
            let u = (rand_double(0.0, 1.0) + i as f64)/(image_width as f64 - 1.0);
            let v = (rand_double(0.0, 1.0) + (image_height - j) as f64)/((image_height - 1) as f64);
            let r = cam.get_ray(u,v);
            let pixel_index = (j*image_width + i) as usize;
            image[pixel_index] = Pixel::Color(ray_color(&r, settings.scene.background, &settings.scene.primitives, settings.raytrace_settings.max_depth));
            image.samples_added[pixel_index] += 1;
            if let Ok(message) = gui_to_thread_rx.try_recv() {
                match message.instructions {
                    Instructions::Terminate => return,
                    Instructions::Pause => {
                        *paused = true;
                        return;
                    }
                    _ => {}
                }
            }
        }
    }

    let output = ImageData { image, draw_mode: DrawMode::Raytrace, id };
    thread_to_gui_tx.send(StatusUpdate::Running(output));
}