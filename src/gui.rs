use eframe::{egui::{self, Vec2, Visuals, Sense, panel::TopBottomSide, style::Margin}, epi, epaint::ColorImage};

use crate::vec::*;
use crate::*;



pub struct Gui {
    pub thread_coordinator: ThreadCoordinator,
    pub settings: Settings,
    pub labels: Labels,
    pub count: i32,
    pub camera_speed: f64,
    pub expecting_data: bool,
    pub windows: Windows,
    pub renderers: Renderers
}

pub struct Windows {
    pub image_settings: bool,
    pub render_settings: bool,
    pub camera_settings: bool
}

pub struct Renderers{
    pub raytracer: bool,
    pub rasterizer: bool
}

impl Gui{
    pub fn new(settings: Settings, thread_coordinator: ThreadCoordinator) -> Gui {
        let camera_speed = 1.0;

        let image_width = settings.image_settings.image_width;
        let image_height = settings.image_settings.image_height;
        let samples_per_pixel = settings.raytrace_settings.samples_per_pixel;

        let labels = Labels{width: image_width.to_string(), height: image_height.to_string(), samples: samples_per_pixel.to_string(), camera_speed: camera_speed.to_string()};
        let windows = Windows { image_settings: false, render_settings: false, camera_settings: false };
        let renderers = Renderers {raytracer: false, rasterizer: false};
        let count = 0;
        let expecting_data = true;

        Gui { thread_coordinator, settings, labels, count, camera_speed, expecting_data, windows, renderers}
    }



    pub fn capture_user_input(&mut self, ctx: &egui::Context) {
        let user_input = ctx.input();
        let mut up = 0.0;
        let mut right = 0.0;

        if user_input.key_down(egui::Key::W) {
            up -= 1.0;
        }

        if user_input.key_down(egui::Key::A) {
            right -= 1.0;
        }

        if user_input.key_down(egui::Key::S) {
            up += 1.0;
        }

        if user_input.key_down(egui::Key::D) {
            right += 1.0;
        }

       
        if up != 0.0 || right != 0.0 {
            let settings = self.settings.camera_settings;
            let w = (settings.look_from - settings.look_at).unit_vector();
            let u = Vec3::cross(settings.v_up, w);
            self.settings.camera_settings.look_from = self.settings.camera_settings.look_from + up * w * self.camera_speed;
            self.settings.camera_settings.look_at = self.settings.camera_settings.look_at + up * w * self.camera_speed;
            self.settings.camera_settings.look_from = self.settings.camera_settings.look_from + right * u * self.camera_speed;
            self.settings.camera_settings.look_at = self.settings.camera_settings.look_at + right * u * self.camera_speed;


            self.thread_coordinator.update_camera(settings, Priority::Next);
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


impl epi::App for Gui {
    fn name(&self) -> &str {
        "Ray Tracer"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        //let Self {thread_output_rx, thread_input_tx, input_data, image_data, labels, count} = self;
        self.capture_user_input(ctx);        
        let response = egui::TopBottomPanel::new(TopBottomSide::Top, "top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                    
                    if ui.button("Save").clicked() {
                        let path = "results.ppm";
                        self.thread_coordinator.image.save(path);
                    }
                });

                ui.menu_button("View", |ui| {
                    ui.checkbox( &mut self.windows.image_settings, "  Image Settings");
                    ui.checkbox( &mut self.windows.render_settings, "  Render Settings");
                    ui.checkbox( &mut self.windows.camera_settings, "  Camera Settings");
                });

                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    if ui.button("🗙").clicked() {
                        frame.quit();
                    }
                });
                if self.windows.render_settings || self.windows.image_settings || self.windows.camera_settings {
                    
                    egui::Window::new("Window").show(ctx, |ui| {
                        
                        //Image Settings
                        if self.windows.image_settings {
                            ui.horizontal(|ui| {
                                ui.label("Width:");
                                let width_response =  ui.add_sized(Vec2::new(30f32, 20f32), egui::TextEdit::singleline(&mut self.labels.width));
                                if width_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                    match self.labels.width.parse::<usize>(){
                                        Ok(num) => {
                                            self.settings.image_settings.image_width = num;
                                            self.thread_coordinator.update_settings(self.settings.clone(), Priority::Now);
                                        }
                                        Err(_) => {
                                            self.labels.width = self.settings.image_settings.image_width.to_string();
                                        }
                                    }
                                }
                                ui.label("Height:");
                                let height_response =  ui.add_sized(Vec2::new(30f32, 20f32), egui::TextEdit::singleline(&mut self.labels.height));
                                if height_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                    match self.labels.height.parse::<usize>(){
                                        Ok(num) => {
                                            self.settings.image_settings.image_height = num;
                                            self.thread_coordinator.update_settings(self.settings.clone(), Priority::Now);
                                        }
                                        Err(_) => {
                                            self.labels.height = self.settings.image_settings.image_height.to_string();
                                        }
                                    }
                                }
                            });
                
                        }

                        if self.windows.render_settings {
                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.checkbox( &mut self.renderers.raytracer, "Raytracer");
                                ui.checkbox( &mut self.renderers.rasterizer, "Rasterizer");
                            });
                            ui.horizontal(|ui| {
                                ui.label("Samples:");
                                let samples_response =  ui.add_sized(Vec2::new(30f32, 20f32), egui::TextEdit::singleline(&mut self.labels.samples));
                                if samples_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                    match self.labels.samples.parse::<usize>(){
                                        Ok(num) => {
                                            self.settings.raytrace_settings.samples_per_pixel = num;
                                            if num < self.settings.raytrace_settings.samples_per_pixel {
                                                self.thread_coordinator.update_settings(self.settings.clone(), Priority::Now)
                                            }
                                        }
                                        Err(_) => {
                                        self.labels.samples = self.settings.raytrace_settings.samples_per_pixel.to_string();
                                        }
                                    }
                                }
                            });
                        }

                        if self.windows.camera_settings {
                            ui.separator();
                            
                        }
                    });
                }
            });
        });
            
        // if response.response.interact(Sense::drag()).dragged() {
        //     frame.drag_window();
        // };



        egui::CentralPanel::default().frame(egui::Frame{ margin: Margin::same(0.0),..Default::default() }).show(ctx, |ui| {
            let rgbas = self.thread_coordinator.image.output_rgba();
            let image = ColorImage::from_rgba_unmultiplied([self.thread_coordinator.image.image_width, self.thread_coordinator.image.image_height], &rgbas);
            let texture_handle = egui::Context::load_texture(&ctx, "output_image", image);
            ui.image(texture_handle.id(), [self.thread_coordinator.image.image_width as f32, self.thread_coordinator.image.image_height as f32]);
        });

        if !self.thread_coordinator.is_done() {
            ctx.request_repaint();
            self.thread_coordinator.update_image();
        } 
    }
}

