#![windows_subsystem = "windows"]
#![allow(dead_code,unused_imports)]
#[macro_use] 

extern crate impl_ops;
extern crate fastrand;
extern crate tobj;
extern crate num_cpus;
extern crate enum_dispatch;
extern crate eframe;
extern crate line_drawing;
extern crate delegate;
extern crate nalgebra;

pub mod camera;
pub mod material;
pub mod util;
pub mod gui;
pub mod threads;
pub mod geometry;
pub mod image;
pub mod rasterizing;
pub mod primitives;
pub mod scenes;
pub mod raytracing;
pub mod spectrum;
pub mod sampler;
pub mod vec;

use eframe::egui::Vec2;

use camera::*;
use gui::*;
use spectrum::constant_spectra::ConstantSpectra;
use threads::*;
use geometry::*;
use primitives::*;
use scenes::*;
use eframe::egui::*;
use nalgebra::{Vector3};

use std::f64::INFINITY;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::*;
use std::thread::Thread;

fn main() {
    //Constants
    let constant_spectra = ConstantSpectra::new();

    //Scene
    let (geometric_primitives, background, look_from, look_at) = scenes::sphere_world(&constant_spectra);
    let bvh = Primitive::new_bvh(geometric_primitives.clone().to_bvh());
    let mut primitives = Primitives::new();
    primitives.add(bvh);

    //Image
    let aspect_ratio = 3.0/2.0;
    let image_width = 800;
    let image_height=  ((image_width as f64)/aspect_ratio) as usize;
    let samples_per_pixel = 1000;
    let max_depth=  50;

    //Camera
    let v_up: Vector3<f64> = Vector3::<f64>::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.0;
    let camera_settings = CameraSettings { look_from, look_at, v_up, v_fov: 20.0, aspect_ratio, aperture, focus_dist, image_height, image_width};
    let camera = Camera::new(camera_settings);
    
    //Package data
    let image_settings = ImageSettings { image_width, image_height };
    let raytrace_settings = RayTraceSettings { max_depth, samples_per_pixel };
    let scene = SceneData { raytracing_primitives: primitives, rasterization_primitives: geometric_primitives, background };
    let settings = GlobalSettings { raytrace_settings, image_settings, camera, scene, id: 1 };

    //Threading
    let mut thread_coordinator = ThreadCoordinator::new(settings.clone());
    thread_coordinator.spin_up(3);

    //Gui
    let app = Gui::new(settings.clone(), thread_coordinator);
    let initial_window_size = Some(Vec2::new(image_width as f32, image_height as f32));
    let native_options = eframe::NativeOptions { initial_window_size, decorated: false,..Default::default() };
    eframe::run_native("Raytracer", native_options, Box::new(|_cc| Box::new(app)));
}
