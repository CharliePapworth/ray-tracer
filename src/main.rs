#[macro_use] 

extern crate impl_ops;
extern crate fastrand;
extern crate tobj;
extern crate num_cpus;
extern crate enum_dispatch;
extern crate eframe;

mod vec;
mod ray;
mod sphere;
mod traceable;
mod camera;
mod material;
mod util;
mod bvh;
mod rect;
mod triangle;
mod scenes;
mod primitive;
mod bounding_box;
mod gui;

use eframe::egui::Vec2;

use crate::vec::*;
use crate::ray::*;
use crate::traceable::*;
use crate::camera::*;
use crate::util::*;
use crate::gui::*;

use std::f64::INFINITY;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::mpsc::*;
use std::thread;

#[derive (Copy, Clone)]
pub struct InputData {
    pub image_width: usize,
    pub image_height: usize,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub run: bool
}

#[derive (Clone)]
pub struct StaticData<H> where H: Hit{
    pub world: H,
    pub background: Color,
    pub cam: Camera,    
}

#[derive (Clone, Default)]
pub struct ImageData{
    pub pixel_colors: Vec<Color>,
    pub image_width: usize,
    pub image_height: usize,
    pub samples: usize
}

fn main(){

    //Scene
    let (world, background, look_from, look_at) = scenes::sphere_world();
    let world = world.to_Bvh();

    //Image
    let aspect_ratio = 3.0/2.0;
    let image_width = 800 as usize;
    let image_height=  ((image_width as f64)/aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_depth=  50;

    //Camera
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let cam = Camera::new(look_from, look_at, v_up, 20.0, aspect_ratio, aperture, dist_to_focus);
   
    //Shared data
    let num_threads = (num_cpus::get() - 4) as i32;
    let samples = ((samples_per_pixel as f64) / (num_threads as f64)).ceil() as i32;
    
    //Package data
    let input_data = InputData { image_width, image_height, samples_per_pixel, max_depth, run: true };
    let static_data = Arc::new(StaticData { world, background, cam });

    //Threading
    let main_thread_samples = samples_per_pixel - samples * (num_threads - 1);
    let (thread_to_gui_tx, thread_to_gui_rx): (Sender<ImageData>, Receiver<ImageData>) = channel();
    let gui_to_thread_tx = initialise_threads(input_data, Arc::clone(&static_data), samples, main_thread_samples, thread_to_gui_tx, num_threads);

    let app = Gui::new(thread_to_gui_rx, gui_to_thread_tx, input_data);
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(image_width as f32 + 216f32, image_height as f32 + 36f32));
    eframe::run_native(Box::new(app), native_options);
}

pub fn ray_color<T>(r: &Ray, background: Color, world: &T, depth: i32) -> Color where T: Hit {

    //If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0{
        return Color::new(0.0,0.0,0.0)
    }

    let result = world.trace(r, 0.001, INFINITY);
    match result{
        TraceResult::Scattered((attenuation, scattered)) => attenuation.elementwise_mult(&ray_color(&scattered, background, world, depth-1)),
        TraceResult::Absorbed(emitted) => emitted,
        TraceResult::Missed => background      
    }
}

pub fn initialise_file(path: &str, image_width: usize, image_height: usize) -> File{
    let mut file = OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .open(path)
                                    .unwrap();
    write!(file, "P3\n{} {} \n255\n", image_width, image_height).unwrap();
    //println!("{}",image_width*image_height);
    file
}

pub fn initialise_threads<H>(input_data: InputData, scene_data: Arc<StaticData<H>>, samples: i32, remaining_samples: i32, thread_to_gui_tx: Sender<ImageData>, num_threads: i32) -> Vec<Sender<InputData>>
where H: Hit + 'static {
    let mut senders = vec![];
    let barrier = Arc::new(Barrier::new((num_threads) as usize));
    for i in 0..num_threads {
        let barrier_clone = Arc::clone(&barrier);
        let thread_to_gui_tx = thread_to_gui_tx.clone();
        let (gui_to_thread_tx, gui_to_thread_rx): (Sender<InputData>, Receiver<InputData>) = channel();
        let static_data = Arc::clone(&scene_data);
        if i == num_threads{
            thread::spawn(move || run_thread(input_data, static_data, remaining_samples, thread_to_gui_tx,  gui_to_thread_rx, barrier_clone));

        } else {
            thread::spawn(move || run_thread(input_data, static_data, samples, thread_to_gui_tx, gui_to_thread_rx,  barrier_clone));
        }

        senders.push(gui_to_thread_tx);
    }
    senders
}

 pub fn run_thread<H>(mut input_data: InputData, static_data: Arc<StaticData<H>>, samples: i32, thread_to_gui_tx: Sender<ImageData>, gui_to_thread_rx: Receiver<InputData>, barrier: Arc<Barrier>)
 where H: Hit + 'static {

    while input_data.run {
        barrier.wait();
        input_data = iterate_image(input_data, Arc::clone(&static_data), samples, thread_to_gui_tx.clone() , &gui_to_thread_rx);
    }
 }

pub fn iterate_image<H>(mut input_data: InputData, static_data: Arc<StaticData<H>>, samples: i32, thread_to_gui_tx: Sender<ImageData>, gui_to_thread_rx: &Receiver<InputData>)
 -> InputData where H: Hit + 'static {

    let image_height = input_data.image_height;
    let image_width = input_data.image_width;
    for _ in 0..samples{
        let mut pixel_colors = vec![Color::new(0.0,0.0,0.0); image_height * image_width];
        for j in 0..image_height{
            for i in 0..image_width{
                    let u = (rand_double(0.0, 1.0) + i as f64)/(image_width as f64 - 1.0);
                    let v = (rand_double(0.0, 1.0) + (image_width - j) as f64)/((image_width - 1) as f64);
                    let r = static_data.cam.get_ray(u,v);
                    let pixel_index = (j*image_width + i) as usize;
                    pixel_colors[pixel_index] = pixel_colors[pixel_index] + ray_color(&r, static_data.background, &static_data.world, input_data.max_depth);
                    let message = gui_to_thread_rx.try_recv();
                    match message {
                        Ok(input_data) => return input_data,
                        Err(err) => {
                            match err {
                                TryRecvError::Empty => {}
                                TryRecvError::Disconnected => {
                                    input_data.run = false;
                                    return input_data;
                                }
                            }
                            
                        }
                    }
                }
        }
        let output = ImageData{pixel_colors, image_width, image_height, samples: 1};
        thread_to_gui_tx.send(output).unwrap();
    }
    input_data 
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use super::*;

    #[test]
    fn test_bound(){
        let x = 10.0;
        let max_x = bound(x, 5.0, 7.0);
        let min_x = bound(x, 11.0, 14.0);
        assert_eq!(max_x, 7.0);
        assert_eq!(min_x, 11.0);
    }

    #[test]
    fn test_deg_2_rad(){
        let deg = 180.0;
        let rad = deg_to_rad(deg);
        assert_eq!(PI, rad);
    }

}
