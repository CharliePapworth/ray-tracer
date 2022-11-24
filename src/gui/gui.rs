use crate::integrator::integrator::Integrator;
use crate::*;
use crate::{
    nalgebra::{Point2, Point3, Rotation3, Unit, Vector3},
};

use crate::gui::settings_window::SettingsWindow;
use crate::settings::Settings;
use eframe::egui::{Response, Style, Visuals};
use eframe::epaint::Stroke;
use eframe::*;
use eframe::{
    egui::{self, panel::TopBottomSide, style::Margin, Context, Sense, Ui},
    epaint::{Color32, ColorImage},
};

use super::file_drop_down::FileDropDown;
use super::progress_bar::CustomProgressBar;

pub struct Gui {
    //pub concurrent_integrator: ConcurrentIntegrator,
    pub settings_window: SettingsWindow,
    pub file_drop_down: FileDropDown,
    pub settings: Settings,
}

impl Gui {
    pub fn new(mut settings: Settings) -> Gui {
        let settings_window = SettingsWindow::new(&mut settings);
        let file_drop_down = FileDropDown::new();
        Gui { settings_window, file_drop_down, settings}
    }

    // pub fn show_image(&self, ctx: &Context, ui: &mut Ui) {
    //     let image = self.concurrent_integrator.output_image();
    //     let rgbas = image.output(self.image_output, self.outline).output_rgba();
    //     let raw_image = ColorImage::from_rgba_unmultiplied([image.image_width, image.image_height], &rgbas);
    //     let texture_handle = egui::Context::load_texture(&ctx, "output_image", raw_image);
    //     ui.image(texture_handle.id(), [image.image_width as f32, image.image_height as f32]);
    // }

    

    // pub fn capture_mouse_input(&mut self, ctx: &egui::Context, response: Response) {
    //     if response.interact(Sense::drag()).dragged() {
    //         let cam = self.settings.camera;
    //         let egui_pointer_pos = ctx.pointer_interact_pos().unwrap();
    //         let pointer_position: Point2<f32> = Point2::<f32>::new(
    //             egui_pointer_pos[0] as f32,
    //             (self.settings.image_settings.image_width as f32) - (egui_pointer_pos[1] as f32),
    //         );
    //         let pointer_position_3d: Point3<f32> = cam.lower_left_corner
    //             + pointer_position[0] * cam.horizontal.norm() * cam.orientation.u.into_inner()
    //                 / (self.settings.image_settings.image_width as f32)
    //             + pointer_position[1] * cam.orientation.v.into_inner() * cam.vertical.norm()
    //                 / (self.settings.image_settings.image_height as f32);
    //         let click_vector = pointer_position_3d - cam.origin;
    //         if self.dragging && click_vector != self.click_vector {
    //             let rotation_axis = Unit::new_normalize(click_vector.cross(&self.click_vector));
    //             let angle = click_vector.angle(&self.click_vector);
    //             let rotation = Rotation3::from_axis_angle(&rotation_axis, angle);
    //             self.settings.camera.rotate(rotation_axis, angle);
    //             self.integrator.update_settings(self.settings.clone());
    //             self.click_vector = rotation.transform_vector(&click_vector);
    //         } else {
    //             self.dragging = true;
    //             self.click_vector = click_vector
    //         }
    //     } else {
    //         self.dragging = false;
    //     }
    // }

    // pub fn capture_keyboard_input(&mut self, ctx: &egui::Context) {
    //     let user_input = ctx.input();
    //     let mut up = 0.0;
    //     let mut forward = 0.0;
    //     let mut right = 0.0;

    //     if user_input.key_down(egui::Key::W) {
    //         forward += self.camera_speed;
    //     }

    //     if user_input.key_down(egui::Key::S) {
    //         forward -= self.camera_speed;
    //     }

    //     if user_input.key_down(egui::Key::A) {
    //         right -= self.camera_speed;
    //     }

    //     if user_input.key_down(egui::Key::D) {
    //         right += self.camera_speed;
    //     }

    //     if user_input.key_down(egui::Key::Space) {
    //         up += self.camera_speed;
    //     }

    //     if user_input.key_down(egui::Key::F) {
    //         up -= self.camera_speed;
    //     }

    //     if up != 0.0 || right != 0.0 || forward != 0.0 {
    //         self.settings.camera.translate(forward, right, up);
    //         self.integrator.update_settings(self.settings.clone());
    //     }
    // }
}

impl eframe::App for Gui {
    // Called each time the UI needs repainting.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_style(Style{visuals: Visuals {code_bg_color: Color32::from_rgb(200,
        200, 200), ..Default::default()}, ..Default::default()});
        //self.capture_keyboard_input(ctx);

        let top_frame = egui::Frame {
            inner_margin: Margin::symmetric(5.0, 5.0),
            fill: Color32::WHITE,
            stroke: Stroke::new(1.0, Color32::GRAY),
            ..Default::default()
        };

        let top_frame_response = egui::TopBottomPanel::new(TopBottomSide::Top, "top_panel")
            .frame(top_frame)
            .show(ctx, |ui| {
                self.file_drop_down.add(ctx, ui, &mut self.settings_window);
            });

        // if top_frame_response.response.interact(Sense::drag()).dragged() {
        //     frame.drag_window();
        // };

        self.settings_window.add_window(ctx, &mut self.settings);

        // let central_panel = egui::CentralPanel::default().frame(egui::Frame {
        //     outer_margin: Margin::same(0.0),
        //     stroke: Stroke::new(1.0, Color32::GRAY),
        //     ..Default::default()
        // });
        // let central_panel_response = central_panel.show(ctx, |ui| self.show_image(ctx, ui)).response;
        // self.capture_mouse_input(ctx, central_panel_response);

        // let completed_samples = self.integrator.get_progress() as f32;
        // let requested_samples = self.settings.settings.samples_per_pixel as f32;
        // let progress = completed_samples / requested_samples;
        let bottom_frame = egui::Frame {
            inner_margin: Margin::symmetric(5.0, 5.0),
            stroke: Stroke::new(1.0, Color32::GRAY),
            fill: Color32::WHITE,
            ..Default::default()
        };
        // egui::TopBottomPanel::new(TopBottomSide::Bottom, "bottom_panel")
        //     .frame(bottom_frame)
        //     .show(ctx, |ui| {
        //         ui.add(
        //             CustomProgressBar::new(progress)
        //                 .desired_width(200.0)
        //                 .text(completed_samples.to_string() + "/" + &requested_samples.to_string() + " samples")
        //                 .animate(true),
        //         );
        //     });

        // if !self.integrator.is_done() {
        //     ctx.request_repaint();
        // }
    }
}
