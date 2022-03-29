#![allow(dead_code,unused_imports)]
#[macro_use] 

extern crate impl_ops;
extern crate fastrand;
extern crate tobj;
extern crate num_cpus;
extern crate enum_dispatch;
extern crate eframe;
extern crate line_drawing;

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
mod gui;
mod threads;
mod geometry;

use eframe::egui::Vec2;
use primitive::Primitive;

use crate::vec::*;
use crate::ray::*;
use crate::traceable::*;
use crate::camera::*;
use crate::util::*;
use crate::gui::*;
use crate::threads::*;
use crate::geometry::*;

use std::f64::INFINITY;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::*;

#[derive (Clone)]
pub struct StaticData<H, W> where H: Hit, W: Outline{
    pub world: H,
    pub primitives: W,
    pub background: Color,
    pub cam: Camera,    
}

fn main(){

    //Scene
    let (primitives, background, look_from, look_at) = scenes::obj_test();
    let world = primitives.clone().to_bvh();

    //Image
    let aspect_ratio = 3.0/2.0;
    let image_width = 800;
    let image_height=  ((image_width as f64)/aspect_ratio) as usize;
    let samples_per_pixel = 1;
    let max_depth=  50;

    //Camera
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.0;
    let camera_settings = CameraSettings { look_from, look_at, v_up, v_fov: 20.0, aspect_ratio, aperture, focus_dist, image_height, image_width};
    let cam = Camera::new(camera_settings);
    
    //Package data
    let image_settings = ImageSettings{ image_width, image_height };
    let raytrace_settings = RayTraceSettings { max_depth, samples_per_pixel };
    let settings = Settings { raytrace_settings, image_settings, camera_settings, draw_mode: DrawMode::Raytrace, id: 1 };
    let static_data = Arc::new(StaticData { world, primitives, background, cam });

    //Threading
    let num_threads = 1 as i32;//(num_cpus::get() - 4) as i32;
    let settings_lock = Arc::new(RwLock::new(settings));
    let (thread_to_gui_tx, thread_to_gui_rx): (Sender<ImageData>, Receiver<ImageData>) = channel();
    let gui_to_thread_tx = initialise_threads(Arc::clone(&settings_lock), Arc::clone(&static_data), thread_to_gui_tx, num_threads);

    //Gui
    let app = Gui::new(thread_to_gui_rx, gui_to_thread_tx, Arc::clone(&settings_lock));
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
