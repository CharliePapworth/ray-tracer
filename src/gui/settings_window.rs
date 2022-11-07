use eframe::egui::{Context, InnerResponse, Response, TextEdit, Ui, Window};
use eframe::{
    egui::{self, panel::TopBottomSide, style::Margin, Sense},
    epaint::{Color32, ColorImage},
};

use crate::settings::Settings;

use super::validated_text_input::{self, ValidatedTextInput};

pub struct SettingsWindow {
    pub open: bool,
    pub validated_text_inputs: SettingsWindowValidatedTextInputs,
}

pub struct SettingsWindowValidatedTextInputs {
    pub image_height: ValidatedTextInput<usize>,
    pub image_width: ValidatedTextInput<usize>,
    pub samples_per_pixel: ValidatedTextInput<u32>,
    pub camera_speed: ValidatedTextInput<f32>,
}

impl SettingsWindow {
    pub fn new(settings: &mut Settings) -> Self {
        let open = false;
        let image_height = ValidatedTextInput::new(
            String::from("Image Height"),
            settings,
            |height, settings| settings.update_image_height(height),
            |settings| settings.get_image_height(),
        );
        let image_width = ValidatedTextInput::new(
            String::from("Image Width"),
            settings,
            |width, settings| settings.update_image_width(width),
            |settings| settings.get_image_width(),
        );
        let samples_per_pixel = ValidatedTextInput::new(
            String::from("Samples Per Pixel"),
            settings,
            |samples, settings| settings.update_samples_per_pixel(samples),
            |settings| settings.get_samples_per_pixel(),
        );
        let camera_speed = ValidatedTextInput::new(
            String::from("Camera Speed"),
            settings,
            |speed, settings| settings.update_camera_speed(speed),
            |settings| settings.get_camera_speed(),
        );

        let validated_text_inputs = SettingsWindowValidatedTextInputs {
            image_height,
            image_width,
            samples_per_pixel,
            camera_speed,
        };

        Self {
            open,
            validated_text_inputs,
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

    fn add_labels(
        validated_text_inputs: &mut SettingsWindowValidatedTextInputs,
        ui: &mut Ui,
        settings: &mut Settings,
    ) -> InnerResponse<()> {
        ui.vertical(|ui| {
            validated_text_inputs.image_height.add(settings, ui);
            validated_text_inputs.image_width.add(settings, ui);
            validated_text_inputs.samples_per_pixel.add(settings, ui);
            validated_text_inputs.camera_speed.add(settings, ui);
        })
    }
}
