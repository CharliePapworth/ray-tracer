use eframe::egui::{Context, Ui, TextEdit, Response, InnerResponse, Window, Frame};
use eframe::epaint::Stroke;
use eframe::{
    egui::{self, panel::TopBottomSide, style::Margin, Sense,},
    epaint::{Color32, ColorImage},
};

use crate::concurrency::concurrent_integrator::ConcurrentIntegrator;

pub struct TopFrame<'a> {
    frame: Frame,
    concurrent_integrator: &'a ConcurrentIntegrator
}



impl<'a> TopFrame<'a> {
    pub fn new(&mut self, ctx: &Context, ui: &mut Ui) -> Frame {
            egui::Frame {
                inner_margin: Margin::symmetric(5.0, 5.0),
                fill: Color32::WHITE,
                stroke: Stroke::new(1.0, Color32::GRAY),
                ..Default::default()
        }
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) -> Response {
        let top_frame_response = egui::TopBottomPanel::new(TopBottomSide::Top, "top_panel")
        .frame(self.frame)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save Image").clicked() {
                        let path = "results.ppm";
                    }
                });

                if ui.button("Settings").clicked() {
                    self.windows.settings = !self.windows.settings;
                };

                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    if ui.button("🗙").clicked() {
                        self.frame.quit();
                    }
                });
                self.windows.settings.AddWindow(ctx, ui)
            });
        });
    }
}