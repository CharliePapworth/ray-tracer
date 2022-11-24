use eframe::egui::{Context, Ui};

use crate::concurrency::{ConcurrentIntegrator, RunConcurrently};

use super::settings_window::SettingsWindow;

pub struct FileDropDown {}

impl FileDropDown {
    pub fn new() -> FileDropDown {
        Self {}
    }

    pub fn add(&mut self, ctx: &Context, ui: &mut Ui, settings_window: &mut SettingsWindow) {
        ui.menu_button("File", |ui| {
            if ui.button("Save Image").clicked() {
                let path = "results.ppm";
                //integrator.output_image().save(path);
            }
            if ui.button("Settings").clicked() {
                settings_window.open = !settings_window.open;
            };
        });
    }
}
