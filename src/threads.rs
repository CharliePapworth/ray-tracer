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

#[derive (Copy, Clone)]
pub struct Settings {
    pub raytrace_settings: RayTraceSettings,
    pub image_settings: ImageSettings,
    pub camera_settings: CameraSettings,
    pub draw_mode: DrawMode,
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
    ChangeSettings,
    Pause,
    Terminate
}

#[derive (Copy, Clone)]
pub struct Message {
    pub instructions: Instructions,
    pub priority: Priority,
}

#[derive (Copy, Clone)]
pub enum DrawMode {
    Raytrace,
    Rasterize
}
#[derive (Clone, Default)]
pub struct ImageData{
    pub pixel_colors: Vec<Color>,
    pub image_width: usize,
    pub image_height: usize,
    pub samples: usize
}

pub fn initialise_threads<H, W>(settings_lock: Arc<RwLock<Settings>>, scene_data: Arc<StaticData<H, W>>, thread_to_gui_tx: Sender<ImageData>, num_threads: i32) -> Vec<Sender<Message>>
where H: Hit + 'static, W: Outline + 'static {
    let mut senders = vec![];
    let barrier = Arc::new(Barrier::new((num_threads) as usize));
    for _ in 0..num_threads {
        let barrier_clone = Arc::clone(&barrier);
        let thread_to_gui_tx = thread_to_gui_tx.clone();
        let (gui_to_thread_tx, gui_to_thread_rx): (Sender<Message>, Receiver<Message>) = channel();
        let static_data = Arc::clone(&scene_data);
        let settings_lock = Arc::clone(&settings_lock);
        thread::spawn(move || run_thread(settings_lock, static_data, thread_to_gui_tx,  gui_to_thread_rx, barrier_clone));
        senders.push(gui_to_thread_tx);
    }
    senders
}

 pub fn run_thread<H, W>(settings_lock: Arc<RwLock<Settings>>, static_data: Arc<StaticData<H, W>>, thread_to_gui_tx: Sender<ImageData>, gui_to_thread_rx: Receiver<Message>, barrier: Arc<Barrier>)
 where H: Hit +'static, W: Outline + 'static{

    let mut terminated = false;
    let mut paused = false; 

    while !terminated {
        if !paused {
            let settings = *settings_lock.read().unwrap();
            match settings.draw_mode {
                DrawMode::Raytrace => {
                    raytrace(settings, Arc::clone(&static_data), thread_to_gui_tx.clone() , &gui_to_thread_rx);
                }
                DrawMode::Rasterize => {
                    rasterize(settings, Arc::clone(&static_data), thread_to_gui_tx.clone() , &gui_to_thread_rx);
                    paused = true;
                }
            }
        }
        
        else {
            let message = gui_to_thread_rx.recv();
            match message {
                Ok(message) => {
                    match message.instructions {
                        Instructions::Terminate => terminated = true,
                        Instructions::Pause => {},
                        _ => paused = false
                    }
                }
                Err(_) => terminated = true
            }
        }
    }
 }

 pub fn rasterize<H, W>(settings: Settings, static_data: Arc<StaticData<H, W>>, thread_to_gui_tx: Sender<ImageData>, gui_to_thread_rx: &Receiver<Message>)
 where H: Hit + 'static, W: Outline + 'static{
    
    let image_height = settings.image_settings.image_height;
    let image_width = settings.image_settings.image_width;
    let mut pixel_colors = vec![Color::new(0.0,0.0,0.0); image_height * image_width];

    let cam = Camera::new(settings.camera_settings);
    if let Some(pixels) = static_data.primitives.outline(&cam) {
        for pixel in pixels {
            let pixel_index = (image_height - pixel[1] - 1) * image_width + pixel[0];
            pixel_colors[pixel_index] = Color::new(1.0, 1.0, 1.0);
        }
    }

    let output = ImageData{pixel_colors, image_width, image_height, samples: 1};
    thread_to_gui_tx.send(output);
 }


pub fn raytrace<H, W>(settings: Settings, static_data: Arc<StaticData<H, W>>, thread_to_gui_tx: Sender<ImageData>, gui_to_thread_rx: &Receiver<Message>)
where H: Hit + 'static, W: Outline +'static {

    let image_height = settings.image_settings.image_height;
    let image_width = settings.image_settings.image_width;
    let mut pixel_colors = vec![Color::new(0.0,0.0,0.0); image_height * image_width];
    let cam = Camera::new(settings.camera_settings);
    for j in 0..image_height{
        for i in 0..image_width{
            let u = (rand_double(0.0, 1.0) + i as f64)/(image_width as f64 - 1.0);
            let v = (rand_double(0.0, 1.0) + (image_height - j) as f64)/((image_height - 1) as f64);
            let r = cam.get_ray(u,v);
            let pixel_index = (j*image_width + i) as usize;
            pixel_colors[pixel_index] = pixel_colors[pixel_index] + ray_color(&r, static_data.background, &static_data.world, settings.raytrace_settings.max_depth);
            if let Ok(message) = gui_to_thread_rx.try_recv() {
                match message.instructions {
                    Instructions::Terminate => return,
                    _ => {}
                }
            }
        }
    }

    let output = ImageData{pixel_colors, image_width, image_height, samples: 1};
    thread_to_gui_tx.send(output);
}