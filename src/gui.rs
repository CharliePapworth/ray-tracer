use eframe::{egui::{self, TextureId, Sense, Vec2}, epi};
use image::Pixel;

use crate::vec::*;
use crate::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    pub thread_output: Arc<Mutex<OutputData>>,
    pub size: [usize; 2],
}


impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let input = self.thread_output.lock().unwrap();
        let scale = _ctx.pixels_per_point();
        //_frame.set_window_size(Vec2::new(scale * (self.size[0] + 200) as f32, scale * self.size[1] as f32));

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self {thread_output, size} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");
            
            let mut label = "hello world".to_owned();
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut label);
            });

            let mut value = 2f32;
            ui.add(egui::Slider::new(&mut value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                value += 1.0;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            let input = thread_output.lock().unwrap();
            let rgbas = colors_to_rgba(&input.pixel_colors, input.completed_samples);
            let image = epi::Image::from_rgba_unmultiplied(*size, &rgbas);
            // The central panel the region left after adding TopPanel's and SidePanel's
            let texture_id = frame.alloc_texture(image);
            ui.image(texture_id, [size[0] as f32, size[1] as f32]);
            let scale = ctx.pixels_per_point();
            //ui.set_min_width(scale * size[0] as f32);
            //ui.set_min_height(scale * size[1] as f32);
            //(Vec2::new(scale * size[0] as f32, scale * size[1] as f32));
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}

pub fn colors_to_rgba(colors: &Vec<Color>, samples: i64) -> Vec<u8>{
    let mut rgbas = Vec::<u8>::with_capacity(colors.len() * 4);
    for color in colors{
     let rgb = color.scale_colors(samples);
        for i in 0..3{
            rgbas.push(rgb[i]);
        }
        rgbas.push(255);
    }
    rgbas
}

