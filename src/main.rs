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
use crate::material::*;
use crate::bounding_box::*;
use crate::enum_dispatch::*;
use crate::gui::*;

use std::f64::INFINITY;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::Mutex;
use std::sync::mpsc::*;
use std::thread;
use std::thread::JoinHandle;

#[derive (Copy, Clone)]
pub struct InputData {
    pub image_width: i32,
    pub image_height:i32,
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

#[derive (Clone)]
pub struct OutputData{
    pub pixel_colors: Vec<Color>,
    pub completed_samples: i64,
    pub current_calculations: i64,
    pub total_calculations: i64,
    pub progress: i64,
}

fn main(){

    //Scene
    let (world, background, look_from, look_at) = scenes::sphere_world();
    let world = world.to_Bvh();

    //Image
    let aspect_ratio = 3.0/2.0;
    let image_width = 800;
    let image_height=  ((image_width as f64)/aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth=  50;

    //Camera
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let cam = Camera::new(look_from, look_at, v_up, 20.0, aspect_ratio, aperture, dist_to_focus);

    //Render
    let path = "results.ppm";
    let mut file = initialise_file(path, image_width, image_height);
   
    //Shared data
    let num_threads = (num_cpus::get() - 4) as i32;
    let samples = ((samples_per_pixel as f64) / (num_threads as f64)).ceil() as i32;
    let pixel_colors = vec![Color::new(0.0,0.0,0.0); (image_width * image_height) as usize];
    let current_calculations = 0;
    let completed_samples = 0;
    let total_calculations = (image_height * image_width * samples_per_pixel) as i64;
    let progress = 0;
    
    //Package data
    let output_data = Arc::new(Mutex::new(OutputData {pixel_colors, completed_samples, current_calculations, total_calculations, progress }));
    let input_data = InputData { image_width, image_height, samples_per_pixel, max_depth, run: true };
    let static_data = Arc::new(StaticData { world, background, cam });

    //Threading
    let main_thread_samples = samples_per_pixel - samples * (num_threads - 1);
    let senders = initialise_threads(input_data, Arc::clone(&static_data), samples, main_thread_samples, Arc::clone(&output_data), num_threads);

    //Write to file
    //let unlocked_data = shared_data.lock().unwrap();
    // for pixel in unlocked_data.pixel_colors.iter() {
    //     pixel.write_color(&mut file, samples_per_pixel);
    // }

    let mut app = Gui::new(Arc::clone(&output_data), senders, input_data);
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

pub fn initialise_file(path: &str, image_width: i32, image_height: i32) -> File{
    let mut file = OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .open(path)
                                    .unwrap();
    write!(file, "P3\n{} {} \n255\n", image_width, image_height).unwrap();
    //println!("{}",image_width*image_height);
    file
}

pub fn initialise_threads<H>(input_data: InputData, scene_data: Arc<StaticData<H>>, samples: i32, remaining_samples: i32, shared_data: Arc<Mutex<OutputData>>, num_threads: i32) -> Vec<Sender<InputData>>
where H: Hit + 'static {
    let mut senders = vec![];
    let barrier = Arc::new(Barrier::new((num_threads) as usize));
    for i in 0..num_threads {
        let barrier_clone = Arc::clone(&barrier);
        let (tx, rx): (Sender<InputData>, Receiver<InputData>) = channel();
        let output_data = Arc::clone(&shared_data);
        let static_data = Arc::clone(&scene_data);
        if i == num_threads{
            thread::spawn(move || run_thread(input_data, static_data, remaining_samples, output_data, rx, barrier_clone));

        } else {
            thread::spawn(move || run_thread(input_data, static_data, samples, output_data, rx,  barrier_clone));
        }

        senders.push(tx);
    }
    senders
}

pub fn report_data(output_data: Arc<Mutex<OutputData>>, pixel_colors: Vec<Color>) {

     //Acquire lock
     let mut unlocked_data  = output_data.lock().unwrap();
     let current_calculations = unlocked_data.current_calculations;
     let total_calculations = unlocked_data.total_calculations;
     let progress = unlocked_data.progress;

     for i in 0..unlocked_data.pixel_colors.len() {
        unlocked_data.pixel_colors[i] = unlocked_data.pixel_colors[i] + pixel_colors[i];
     }

     //Write data and update progress
     unlocked_data.current_calculations += pixel_colors.len() as i64;
     unlocked_data.completed_samples += 1;
     let new_progress = ((current_calculations) * 100 /total_calculations) as i64;
     if new_progress - progress >= 1 {
         //println!("{}", new_progress);
         unlocked_data.progress += 1;
     }
 }

 pub fn run_thread<H>(mut input_data: InputData, static_data: Arc<StaticData<H>>, samples: i32, output_data: Arc<Mutex<OutputData>>, rx: Receiver<InputData>, barrier: Arc<Barrier>)
 where H: Hit + 'static {

    while input_data.run {
        let barrier_result = barrier.wait();
        if barrier_result.is_leader(){
            let mut unlocked_data  = output_data.lock().unwrap();
            unlocked_data.pixel_colors = vec![Color::new(0.0,0.0,0.0); (input_data.image_width * input_data.image_height) as usize];
            unlocked_data.current_calculations = 0;
            unlocked_data.completed_samples = 0;
            unlocked_data.total_calculations = (input_data.image_height * input_data.image_width * input_data.samples_per_pixel) as i64;
            unlocked_data.progress = 0;
        }
        barrier.wait();
        input_data = iterate_image(input_data, Arc::clone(&static_data), samples, Arc::clone(&output_data), &rx);
    }
 }

pub fn iterate_image<H>(mut input_data: InputData, static_data: Arc<StaticData<H>>, samples: i32, output_data: Arc<Mutex<OutputData>>, rx: &Receiver<InputData>)
 -> InputData where H: Hit + 'static {

    let image_height = input_data.image_height as i64;
    let image_width = input_data.image_width as i64;
    for _ in 0..samples{
        let mut pixel_colors = vec![Color::new(0.0,0.0,0.0); (image_height * image_width) as usize];
        for j in 0..image_height{
            for i in 0..image_width{
                    let u = (rand_double(0.0, 1.0) + i as f64)/(image_width as f64 - 1.0);
                    let v = (rand_double(0.0, 1.0) + (image_width - j) as f64)/((image_width - 1) as f64);
                    let r = static_data.cam.get_ray(u,v);
                    let pixel_index = (j*image_width + i) as usize;
                    pixel_colors[pixel_index] = pixel_colors[pixel_index] + ray_color(&r, static_data.background, &static_data.world, input_data.max_depth);
                    let message = rx.try_recv();
                    match message {
                        Ok(input_data) => return input_data,
                        Err(_) => {}
                    }
                }
        }
        report_data(Arc::clone(&output_data), pixel_colors);
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
