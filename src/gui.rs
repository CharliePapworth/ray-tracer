use eframe::{egui::{self, Vec2}, epi};

use crate::vec::*;
use crate::*;

pub struct Gui {
    pub thread_output_rx: Receiver<ImageData>,
    pub thread_input_tx: Vec<Sender<InputData>>,
    pub input_data: InputData,
    pub image_data: ImageData,
    pub labels: Labels,
    pub count: i32,
    pub camera_speed: f64
}

impl Gui{
    pub fn new(thread_output_rx: Receiver<ImageData>,  thread_input_tx: Vec<Sender<InputData>>, input_data: InputData) -> Gui {
        let camera_speed = 1.0;
        let labels = Labels{width: input_data.image_width.to_string(), height: input_data.image_height.to_string(), samples: input_data.samples_per_pixel.to_string(), camera_speed: camera_speed.to_string()};
        let image_data = ImageData{pixel_colors: vec![Color::new(0.0,0.0,0.0); input_data.image_height * input_data.image_width], image_width: input_data.image_width, image_height: input_data.image_height, samples: 0};
        let count = 0;
        Gui{thread_output_rx, thread_input_tx, input_data, image_data, labels, count, camera_speed}
    }

    pub fn transmit_input_data(&mut self){
        let mut threads_to_remove = vec!();
        for (index, transmitter, ) in self.thread_input_tx.iter().enumerate() {
            if transmitter.send(self.input_data).is_err() {
                threads_to_remove.push(index);
            }
        }
        for index in threads_to_remove {
            self.thread_input_tx.remove(index);
        }
    }

    pub fn refresh_image(&mut self) {
        self.image_data.pixel_colors =  vec![Color::new(0.0,0.0,0.0); self.input_data.image_height * self.input_data.image_width];
        self.image_data.samples = 0;
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
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        //let Self {thread_output_rx, thread_input_tx, input_data, image_data, labels, count} = self;
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
            let settings = self.input_data.camera_settings;
            let w = (settings.look_from - settings.look_at).unit_vector();
            let u = Vec3::cross(settings.v_up, w);
            self.input_data.camera_settings.look_from = self.input_data.camera_settings.look_from + up * w * self.camera_speed;
            self.input_data.camera_settings.look_at = self.input_data.camera_settings.look_at + up * w * self.camera_speed;
            self.input_data.camera_settings.look_from = self.input_data.camera_settings.look_from + right * u * self.camera_speed;
            self.input_data.camera_settings.look_at = self.input_data.camera_settings.look_at + right * u * self.camera_speed;



            self.input_data.done = false;
            self.transmit_input_data();
            self.refresh_image();
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                    
                    if ui.button("Save").clicked() {
                        let path = "results.ppm";
                        let mut file = initialise_file(path, self.image_data.image_width, self.image_data.image_height);
                        for pixel in self.image_data.pixel_colors.iter() {
                            pixel.write_color(&mut file, self.image_data.samples);
                         }
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Width:");
                let width_response =  ui.add_sized(Vec2::new(30f32, 20f32), egui::TextEdit::singleline(&mut self.labels.width));
                if width_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    match self.labels.width.parse::<usize>(){
                        Ok(num) => {
                            self.input_data.image_width = num;
                            self.input_data.done = false;
                            self.transmit_input_data();
                        }
                        Err(_) => {
                            self.labels.width = self.input_data.image_width.to_string();
                        }
                    }
                }
                ui.label("Height:");
                let height_response =  ui.add_sized(Vec2::new(30f32, 20f32), egui::TextEdit::singleline(&mut self.labels.height));
                if height_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    match self.labels.height.parse::<usize>(){
                        Ok(num) => {
                            self.input_data.image_height = num;
                            self.input_data.done = false;
                            self.transmit_input_data();
                        }
                        Err(_) => {
                            self.labels.height = self.input_data.image_height.to_string();
                        }
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Samples:");
                let samples_response =  ui.add_sized(Vec2::new(30f32, 20f32), egui::TextEdit::singleline(&mut self.labels.samples));
                if samples_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    match self.labels.samples.parse::<usize>(){
                        Ok(num) => {
                            if num < self.input_data.samples_per_pixel {
                                self.image_data.pixel_colors =  vec![Color::new(0.0,0.0,0.0); self.input_data.image_height * self.input_data.image_width];
                                self.image_data.samples = 0;
                            }
                            self.input_data.samples_per_pixel = num;
                            self.input_data.done = false;
                            self.transmit_input_data();
                        }
                        Err(_) => {
                            self.labels.samples = self.input_data.samples_per_pixel.to_string();
                        }
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Camera Speed:");
                let camera_speed_response =  ui.add_sized(Vec2::new(30f32, 20f32), egui::TextEdit::singleline(&mut self.labels.camera_speed));
                if camera_speed_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    match self.labels.camera_speed.parse::<f64>(){
                        Ok(new_speed) => {
                            if new_speed > 0.0 {
                                self.camera_speed = new_speed;
                            }
                            else {
                                self.labels.camera_speed = self.camera_speed.to_string();
                            }
                        }
                        Err(_) => {
                            self.labels.camera_speed= self.camera_speed.to_string();
                        }
                    }
                }
            })

            // let mut value = 2f32;
            // ui.add(egui::Slider::new(&mut value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     image_data.samples += 1.0;
            // }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let message_result = self.thread_output_rx.try_recv();
            if let Ok(message) = message_result {
                if message.image_height != self.image_data.image_height || message.image_width != self.image_data.image_width {     
                    self.image_data = message;
                }
                else if self.image_data.samples < self.input_data.samples_per_pixel {
                    for i in 0..self.image_data.pixel_colors.len(){
                        self.image_data.pixel_colors[i] = self.image_data.pixel_colors[i] + message.pixel_colors[i];
                    }
                    self.image_data.samples += message.samples;
                }
                if self.image_data.samples >= self.input_data.samples_per_pixel {
                    self.input_data.done = true;
                    self.transmit_input_data();
                }
            }

            let rgbas = colors_to_rgba(&self.image_data.pixel_colors, self.image_data.samples.max(1));
            let image = epi::Image::from_rgba_unmultiplied([self.image_data.image_width, self.image_data.image_height], &rgbas);
            // The central panel the region left after adding TopPanel's and SidePanel's
            let texture_id = frame.alloc_texture(image);
            ui.image(texture_id, [self.image_data.image_width as f32, self.image_data.image_height as f32]);
        });

        if !self.input_data.done {
            ctx.request_repaint();
        }
        self.count += 1;
        println!{"{}", self.image_data.samples};

    }
}





pub fn colors_to_rgba(colors: &[Color], samples: usize) -> Vec<u8>{
    let mut rgbas = Vec::<u8>::with_capacity(colors.len() * 4);
    for color in colors{
     let rgb = color.scale_colors(samples);
        for color in &rgb{
            rgbas.push(*color);
        }
        rgbas.push(255);
    }
    rgbas
}

