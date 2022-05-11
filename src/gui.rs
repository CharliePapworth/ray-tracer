use eframe::{egui::{self, Sense, panel::TopBottomSide, style::Margin, Ui, Context}, epaint::{ColorImage, Color32}};

use crate::{vec::{Vec2, Vec3}, image::PrimaryImageType, threads::{ThreadCoordinator, GlobalSettings}};
use crate::*;

pub struct Gui {
    pub thread_coordinator: ThreadCoordinator,
    pub settings: GlobalSettings,
    pub labels: Labels,
    pub camera_speed: f64,
    pub expecting_data: bool,
    pub windows: Windows,
    pub renderers: Renderers,
    pub image_output: PrimaryImageType,
    pub outline: bool,
    pub click_vector: Vec3,
    pub dragging: bool
}

pub struct Windows {
    pub settings: bool
}

pub struct Renderers{
    pub raytracer: bool,
    pub rasterizer: bool
}

impl Gui{
    pub fn new(settings: GlobalSettings, thread_coordinator: ThreadCoordinator) -> Gui {
        let camera_speed = 0.2;

        let image_width = settings.image_settings.image_width;
        let image_height = settings.image_settings.image_height;
        let samples_per_pixel = settings.raytrace_settings.samples_per_pixel;

        let labels = Labels{width: image_width.to_string(), height: image_height.to_string(), samples: samples_per_pixel.to_string(), camera_speed: camera_speed.to_string()};
        let windows = Windows { settings: false };
        let renderers = Renderers {raytracer: false, rasterizer: false};
        let expecting_data = true;
        let image_output = PrimaryImageType::Raytrace;
        let outline = false;
        let click_vector = Vec3::default();
        let dragging = false;

        Gui { thread_coordinator, settings, labels, camera_speed, expecting_data, windows, renderers, image_output, outline, click_vector, dragging }
    }

    pub fn show_image(&self, ctx: &Context, ui: &mut Ui) {
        let image = self.thread_coordinator.output_image();
        let rgbas = image.output(self.image_output, self.outline).output_rgba();
        let raw_image = ColorImage::from_rgba_unmultiplied([image.image_width, image.image_height], &rgbas);
        let texture_handle = egui::Context::load_texture(&ctx, "output_image", raw_image);
        ui.image(texture_handle.id(), [image.image_width as f32, image.image_height as f32]);
    }

    pub fn show_settings_window(&mut self, ctx: &Context, ui: &mut Ui) {
        //Image Settings
        if self.windows.settings {
            ui.horizontal(|ui| {
                ui.label("Width:");
                let width_response =  ui.add_sized(egui::Vec2::new(30f32, 20f32), egui::TextEdit::singleline(&mut self.labels.width));
                if width_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    match self.labels.width.parse::<usize>(){
                        Ok(num) => {
                            self.settings.image_settings.image_width = num;
                            self.thread_coordinator.update_settings(self.settings.clone());
                        }
                        Err(_) => {
                            self.labels.width = self.settings.image_settings.image_width.to_string();
                        }
                    }
                }
                ui.label("Height:");
                let height_response =  ui.add_sized(egui::Vec2::new(30f32, 20f32), egui::TextEdit::singleline(&mut self.labels.height));
                if height_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    match self.labels.height.parse::<usize>(){
                        Ok(num) => {
                            self.settings.image_settings.image_height = num;
                            self.thread_coordinator.update_settings(self.settings.clone());
                        }
                        Err(_) => {
                            self.labels.height = self.settings.image_settings.image_height.to_string();
                        }
                    }
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.checkbox( &mut self.renderers.raytracer, "Raytracer");
                ui.checkbox( &mut self.renderers.rasterizer, "Rasterizer");
            });
            ui.checkbox(&mut self.outline, "Outline");
            ui.horizontal(|ui| {
                ui.label("Samples:");
                let samples_response =  ui.add_sized(egui::Vec2::new(40f32, 20f32), egui::TextEdit::singleline(&mut self.labels.samples));
                if samples_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    match self.labels.samples.parse::<usize>(){
                        Ok(num) => {
                            self.thread_coordinator.update_samples(num);
                        }
                        Err(_) => {
                        self.labels.samples = self.settings.raytrace_settings.samples_per_pixel.to_string();
                        }
                    }
                }
            });
        
        }
    }

    pub fn capture_mouse_input(&mut self, ctx: &egui::Context, response: Response) {
        if response.interact(Sense::drag()).dragged() {
            let cam = self.settings.camera;
            let egui_pointer_pos = ctx.pointer_interact_pos().unwrap();
            let pointer_position = Vec2::new(egui_pointer_pos[0] as f64, (self.settings.image_settings.image_width as f64) - (egui_pointer_pos[1] as f64));
            let pointer_position_3d = pointer_position[0] * cam.horizontal.length() * cam.orientation.u / (self.settings.image_settings.image_width as f64) + pointer_position[1] * cam.orientation.v * cam.vertical.length() / (self.settings.image_settings.image_height as f64) + cam.lower_left_corner;
            let click_vector = pointer_position_3d - cam.origin;
            if self.dragging && click_vector != self.click_vector{
                let rotation_axis = click_vector.perpendicular(self.click_vector).unit_vector();
                let angle =  click_vector.angle(self.click_vector);
                self.settings.camera.rotate(rotation_axis, angle);
                self.thread_coordinator.update_settings(self.settings.clone());
                self.click_vector = click_vector.rotate(rotation_axis, angle);
            } else {
                self.dragging = true;
                self.click_vector = click_vector
            }
        } else {
            self.dragging = false;
        }
    }


    pub fn capture_keyboard_input(&mut self, ctx: &egui::Context) {
        let user_input = ctx.input();
        let mut up = 0.0;
        let mut forward = 0.0;
        let mut right = 0.0;

        if user_input.key_down(egui::Key::W) {
            forward += self.camera_speed;
        }

        if user_input.key_down(egui::Key::S) {
            forward -= self.camera_speed;
        }

        if user_input.key_down(egui::Key::A) {
            right -= self.camera_speed;
        }

        if user_input.key_down(egui::Key::D) {
            right += self.camera_speed;
        }

        if user_input.key_down(egui::Key::Space) {
            up += self.camera_speed;
        }

        if user_input.key_down(egui::Key::F) {
            up -= self.camera_speed;
        }

        if up != 0.0 || right != 0.0 || forward != 0.0 {
            self.settings.camera.translate(forward, right, up);
            self.thread_coordinator.update_settings(self.settings.clone());
        }
    }
}

#[derive(Default)]
pub struct Labels{
    width: String,
    height: String,
    samples: String,
    camera_speed: String
}


impl eframe::App for Gui {

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        //let Self {thread_output_rx, thread_input_tx, input_data, image_data, labels, count} = self;
        self.capture_keyboard_input(ctx);  
        let top_frame = egui::Frame{inner_margin: Margin::symmetric(5.0, 5.0), fill: Color32::WHITE, ..Default::default()};
        let response = egui::TopBottomPanel::new(TopBottomSide::Top, "top_panel").frame(top_frame).show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {                    
                    if ui.button("Save Image").clicked() {
                        let path = "results.ppm";
                        self.thread_coordinator.output_image().output(PrimaryImageType::Raytrace, true).save(path);
                    }
                });

                if ui.button("Settings").clicked() {
                   self.windows.settings = !self.windows.settings;
                };

                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    if ui.button("🗙").clicked() {
                        frame.quit();
                    }
                });
                if self.windows.settings {
                    let mut settings_clone = self.windows.settings;
                    egui::Window::new("🔧 Settings").open(&mut settings_clone)
                                                           .collapsible(false)
                                                           .fixed_size(egui::Vec2::new(100.0, 100.0))
                                                           .show(ctx, |ui| { self.show_settings_window(ctx, ui) });

                    self.windows.settings = settings_clone;
                }
            });
        });
            
        // if response.response.interact(Sense::drag()).dragged() {
        //     frame.drag_window();
        // };


        let central_panel = egui::CentralPanel::default().frame(egui::Frame{ outer_margin: Margin::same(0.0),..Default::default() });
        let response = central_panel.show(ctx, |ui| {self.show_image(ctx, ui)}).response;
        self.capture_mouse_input(ctx, response);  
        
        if !self.thread_coordinator.is_done() {
            ctx.request_repaint();
        } 
    }
}

