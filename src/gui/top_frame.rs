use eframe::egui::{Context, Frame, InnerResponse, Response, TextEdit, Ui, Window};
use eframe::epaint::Stroke;
use eframe::{
    egui::{self, panel::TopBottomSide, style::Margin, Sense},
    epaint::{Color32, ColorImage},
};

use crate::concurrency::concurrent_integrator::ConcurrentIntegrator;

use super::file_drop_down::{self, FileDropDown};
use super::settings_window::SettingsWindow;

pub struct TopFrame {
    frame: Frame,
    file_drop_down: FileDropDown,
}

impl TopFrame {
    pub fn new(&mut self, ctx: &Context, ui: &mut Ui) -> TopFrame {
        let frame = egui::Frame {
            inner_margin: Margin::symmetric(5.0, 5.0),
            fill: Color32::WHITE,
            stroke: Stroke::new(1.0, Color32::GRAY),
            ..Default::default()
        };

        let file_drop_down = FileDropDown::new();

        Self { frame, file_drop_down }
    }

    pub fn show(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        settings_window: &mut SettingsWindow,
        concurrent_integrator: &ConcurrentIntegrator,
    ) -> InnerResponse<()> {
        egui::TopBottomPanel::new(TopBottomSide::Top, "top_panel")
            .frame(self.frame)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    self.file_drop_down.add(ctx, ui,  settings_window);
                });
            })
    }
}
