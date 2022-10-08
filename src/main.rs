#![windows_subsystem = "windows"]
#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate impl_ops;
extern crate crossbeam;
extern crate delegate;
extern crate eframe;
extern crate enum_dispatch;
extern crate fastrand;
extern crate line_drawing;
extern crate nalgebra;
extern crate num_cpus;
extern crate tobj;

pub mod camera;
pub mod film;
pub mod filter;
pub mod geometry;
pub mod gui;
pub mod image;
pub mod integrator;
pub mod light;
pub mod material;
pub mod primitives;
pub mod rasterizing;
pub mod raytracing;
pub mod sampler;
pub mod scenes;
pub mod spectrum;
pub mod threader;
pub mod util;
pub mod vec;

use eframe::egui::Vec2;

use camera::*;
use eframe::egui::*;
use geometry::*;
use gui::*;
use nalgebra::Vector3;
use primitives::*;
use scenes::*;
use spectrum::constant_spectra::ConstantSpectra;
use threader::*;

use std::f32::INFINITY;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc::*;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread::Thread;

fn main() {
    //Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 800;
    let image_height = ((image_width as f32) / aspect_ratio) as usize;

    //Scene
    let scene = scenes::point_light_test(image_width, aspect_ratio);

    //Integrator
    let samples_per_pixel = 1000;
    let max_depth = 50;

    //Camera

    //Package data
    let raytrace_settings = Settings {
        max_depth,
        samples_per_pixel,
    };
    let settings = ThreadData {
        settings: raytrace_settings,
        scene,
        id: 1,
    };

    //Threading
    let mut thread_coordinator = Multithreader::new(settings.clone());
    thread_coordinator.spin_up(3);

    //Gui
    let app = Gui::new(settings.clone(), thread_coordinator);
    let initial_window_size = Some(Vec2::new(image_width as f32, image_height as f32));
    let native_options = eframe::NativeOptions {
        initial_window_size,
        decorated: false,
        ..Default::default()
    };
    eframe::run_native("Raytracer", native_options, Box::new(|_cc| Box::new(app)));
}
