use eframe::egui::{Context, Ui, TextEdit, Response, InnerResponse, Window};
use eframe::{
    egui::{self, panel::TopBottomSide, style::Margin, Sense,},
    epaint::{Color32, ColorImage},
};

use super::labels::{Label, ImageHeight, ImageWidth, SamplesPerPixel, CameraSpeed};

pub struct SettingsWindows {
    pub open: bool,
    pub image_height: ImageHeight,
    pub image_width: ImageWidth,
    pub samples_per_pixel: SamplesPerPixel,
    pub camera_speed: CameraSpeed,
}

impl SettingsWindows {
    pub fn AddWindow(&mut self, ctx: &Context, ui: &mut Ui) -> Window {
        Window::new("🔧 Settings")
            .open(&mut self.open)
            .collapsible(false)
            .fixed_size(egui::Vec2::new(100.0, 100.0))
            .show(ctx, |ui| self.AddLabels(ctx, ui))
    }

    fn AddLabels(&self, ctx: &Context, ui: &mut Ui) -> InnerResponse<()> {
        ui.vertical(|ui| {
            self.image_height.AddLabel(ctx, ui);
            self.image_width.AddLabel(ctx, ui);
            self.samples_per_pixel.AddLabel(ctx, ui);
            self.camera_speed.AddLabel(ctx, ui);
        })
    }
}