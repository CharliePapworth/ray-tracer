#[macro_use] 

extern crate impl_ops;
extern crate fastrand;
extern crate tobj;
extern crate num_cpus;
extern crate enum_dispatch;
extern crate eframe;
extern crate image;


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

use crate::vec::*;
use crate::ray::*;
use crate::traceable::*;
use crate::camera::*;
use crate::util::*;
use crate::material::*;
use crate::bounding_box::*;
use crate::enum_dispatch::*;
use crate::gui::*;
use crate::image::*;

use std::f64::INFINITY;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

#[derive (Copy, Clone)]
pub struct InputData {
    pub image_width: i32,
    pub image_height:i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32
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
    let image_width = 1200;
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
    let shared_data = Arc::new(Mutex::new(OutputData {pixel_colors, completed_samples, current_calculations, total_calculations, progress }));
    let image_data = InputData { image_width, image_height, samples_per_pixel, max_depth };
    let scene_data = Arc::new(StaticData { world, background, cam });

    //Threading
    let main_thread_samples = samples_per_pixel - samples * (num_threads - 1);
    initialise_threads(image_data.clone(), Arc::clone(&scene_data), samples, Arc::clone(&shared_data), num_threads);
    initialise_threads(image_data.clone(), Arc::clone(&scene_data), main_thread_samples, Arc::clone(&shared_data), 1);
    //iterate_image(image_data.clone(), Arc::clone(&scene_data), main_thread_samples, Arc::clone(&shared_data));
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    //Write to file
    //let unlocked_data = shared_data.lock().unwrap();
    // for pixel in unlocked_data.pixel_colors.iter() {
    //     pixel.write_color(&mut file, samples_per_pixel);
    // }

    let mut app = TemplateApp{thread_output: Arc::clone(&shared_data), size: [image_width as usize, image_height as usize]};
    let native_options = eframe::NativeOptions::default();
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
    println!("{}",image_width*image_height);
    file
}

pub fn initialise_threads<H>(image_data: InputData, scene_data: Arc<StaticData<H>>, samples: i32, shared_data: Arc<Mutex<OutputData>>, num_threads: i32) -> Vec<JoinHandle<()>>
where H: Hit + 'static {
    let mut handles = vec![];
    for _ in 0..num_threads - 1 {
        let shared_data = Arc::clone(&shared_data);
        let scene_data = Arc::clone(&scene_data);
        let handle = thread::spawn(move || iterate_image(image_data, scene_data, samples, shared_data));
        handles.push(handle);
    }
    handles
}

pub fn report_data(shared_data: Arc<Mutex<OutputData>>, pixel_colors: Vec<Color>) {

     //Acquire lock
     let mut unlocked_data  = shared_data.lock().unwrap();
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
         println!("{}", new_progress);
         unlocked_data.progress += 1;
     }
 }

pub fn iterate_image<H>(image_data: InputData, scene_data: Arc<StaticData<H>>, samples: i32, shared_data: Arc<Mutex<OutputData>>)
where H: Hit + 'static {

    let image_height = image_data.image_height as i64;
    let image_width = image_data.image_width as i64;
    for _ in 0..samples{
        let mut pixel_colors = vec![Color::new(0.0,0.0,0.0); (image_height*image_width) as usize];
        for j in 0..image_height{
            for i in 0..image_width{
                    let u = (rand_double(0.0, 1.0) + i as f64)/(image_width as f64 - 1.0);
                    let v = (rand_double(0.0, 1.0) + (image_width - j) as f64)/((image_width - 1) as f64);
                    let r = scene_data.cam.get_ray(u,v);
                    let pixel_index = (j*image_width + i) as usize;
                    pixel_colors[pixel_index] = pixel_colors[pixel_index] + ray_color(&r, scene_data.background, &scene_data.world, image_data.max_depth);
                }
        }
        report_data(Arc::clone(&shared_data), pixel_colors);  
    }   
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
