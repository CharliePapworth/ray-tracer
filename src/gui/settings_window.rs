use eframe::egui::{Context, Ui, TextEdit, Response, InnerResponse, Window};
use eframe::{
    egui::{self, panel::TopBottomSide, style::Margin, Sense,},
    epaint::{Color32, ColorImage},
};

use crate::settings::Settings;

use super::validated_text_input::{ValidatedTextInput, self};

pub struct SettingsWindow {
    pub open: bool,
    pub validated_text_inputs: ValidatedTextInputs
}

pub struct ValidatedTextInputs {
    pub image_height: ValidatedTextInput,
    pub image_width: ValidatedTextInput,
    pub samples_per_pixel: ValidatedTextInput,
    pub camera_speed: ValidatedTextInput,
}

impl SettingsWindow {

    pub fn new(settings: &mut Settings) -> Self {
        let open = false;
        let image_height = ValidatedTextInput::new(String::from("Image Height"), settings.get_image_height());
        let image_width = ValidatedTextInput::new(String::from("Image Width"), settings.get_image_width());
        let samples_per_pixel = ValidatedTextInput::new(String::from("Samples Per Pixel"), settings.get_samples_per_pixel());
        let camera_speed = ValidatedTextInput::new(String::from("Camera Speed"), settings.get_camera_speed());
        
        let validated_text_inputs = ValidatedTextInputs {
            image_height,
            image_width,
            samples_per_pixel,
            camera_speed,
        };
        
        Self {
            open,
            validated_text_inputs
            
        }
    }

    pub fn add_window(&mut self, ctx: &Context, settings: &mut Settings) -> Option<InnerResponse<Option<InnerResponse<()>>>> {
        let validated_text_inputs = &mut self.validated_text_inputs;
        Window::new("🔧 Settings")
            .open(&mut self.open)
            .collapsible(false)
            .fixed_size(egui::Vec2::new(100.0, 100.0))
            .show(ctx, |ui| Self::add_labels(validated_text_inputs, ui, settings))
    }

    fn add_labels(validated_text_inputs: &mut ValidatedTextInputs, ui: &mut Ui, settings: &mut Settings) -> InnerResponse<()> {
        ui.vertical(|ui| {
            validated_text_inputs.image_height.add(settings, ui, |height, settings| settings.update_image_height(height), |settings| settings.get_image_height());
            validated_text_inputs.image_width.add(settings, ui, |width, settings| settings.update_image_width(width), |settings| settings.get_image_width());
            validated_text_inputs.samples_per_pixel.add(settings, ui, |samples, settings| settings.update_samples_per_pixel(samples), |settings| settings.get_samples_per_pixel());
            validated_text_inputs.camera_speed.add(settings, ui, |speed, settings| settings.update_camera_speed(speed), |settings| settings.get_camera_speed());
        })
    }
}