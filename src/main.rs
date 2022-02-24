#![allow(dead_code,unused_imports)]
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
mod threads;

use eframe::egui::Vec2;

use crate::vec::*;
use crate::ray::*;
use crate::traceable::*;
use crate::camera::*;
use crate::util::*;
use crate::gui::*;
use crate::threads::*;

use std::f64::INFINITY;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::sync::mpsc::*;

#[derive (Clone)]
pub struct StaticData<H> where H: Hit{
    pub world: H,
    pub background: Color,
    pub cam: Camera,    
}

fn main(){

    //Scene
    let (world, background, look_from, look_at) = scenes::sphere_world();
    let world = world.to_bvh();

    //Image
    let aspect_ratio = 3.0/2.0;
    let image_width = 800;
    let image_height=  ((image_width as f64)/aspect_ratio) as usize;
    let samples_per_pixel = 1;
    let max_depth=  50;

    //Camera
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let cam = Camera::new(look_from, look_at, v_up, 20.0, aspect_ratio, aperture, dist_to_focus);
    
    //Package data
    let input_data = InputData { image_width, image_height, samples_per_pixel, max_depth, run: true, done: false};
    let static_data = Arc::new(StaticData { world, background, cam });

    //Threading
    let num_threads = (num_cpus::get() - 4) as i32;
    let (thread_to_gui_tx, thread_to_gui_rx): (Sender<ImageData>, Receiver<ImageData>) = channel();
    let gui_to_thread_tx = initialise_threads(input_data, Arc::clone(&static_data), thread_to_gui_tx, num_threads);

    //Gui
    let app = Gui::new(thread_to_gui_rx, gui_to_thread_tx, input_data);
    let initial_window_size = Some(Vec2::new(image_width as f32 + 216f32, image_height as f32 + 36f32));
    let native_options = eframe::NativeOptions {initial_window_size, ..Default::default()};
    eframe::run_native(Box::new(app), native_options);
}

pub fn ray_color<T>(r: &Ray, background: Color, world: &T, depth: i32) -> Color where T: Hit {

    //If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0{
        return Color::new(0.0,0.0,0.0)
    }

    let result = world.trace(r, 0.001, INFINITY);
    match result {
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
    file
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
