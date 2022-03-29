use eframe::{egui::{self, Vec2}, epi};

use crate::vec::*;
use crate::*;

pub struct Gui {
    pub thread_output_rx: Receiver<ImageData>,
    pub thread_input_tx: Vec<Sender<Message>>,
    pub settings_lock: Arc<RwLock<Settings>>,
    pub settings: Settings,
    pub image_data: ImageData,
    pub labels: Labels,
    pub count: i32,
    pub camera_speed: f64,
    pub expecting_data: bool,
    pub recieved_id: i32
}

impl Gui{
    pub fn new(thread_output_rx: Receiver<ImageData>,  thread_input_tx: Vec<Sender<Message>>, settings_lock: Arc<RwLock<Settings>>) -> Gui {
        let camera_speed = 1.0;
        let settings = *settings_lock.read().unwrap();

        let image_width = settings.image_settings.image_width;
        let image_height = settings.image_settings.image_height;
        let max_depth = settings.raytrace_settings.max_depth;
        let samples_per_pixel = settings.raytrace_settings.samples_per_pixel;

        let labels = Labels{width: image_width.to_string(), height: image_height.to_string(), samples: samples_per_pixel.to_string(), camera_speed: camera_speed.to_string()};
        let image_data = ImageData{pixel_colors: vec![Color::new(0.0,0.0,0.0); image_height * image_width], image_width: image_width, image_height: image_height, samples: 0, id: 0 };
        let count = 0;
        let expecting_data = true;
        let recieved_id = 0;

        Gui{ thread_output_rx, thread_input_tx, settings_lock, settings, image_data, labels, count, camera_speed, expecting_data, recieved_id }
    }

    pub fn transmit_settings(&mut self){
        self.settings.id += 1;
        let mut settings = self.settings_lock.write().unwrap();
        *settings = self.settings;
       
    }

    pub fn transmit_message(&mut self, message: Message){
        let mut threads_to_remove = vec!();
        for (index, transmitter, ) in self.thread_input_tx.iter().enumerate() {
            if transmitter.send(message).is_err() {
                threads_to_remove.push(index);
            }
        }
        for index in threads_to_remove {
            self.thread_input_tx.remove(index);
        }
    }

    pub fn refresh_image(&mut self) {
        let image_width = self.settings.image_settings.image_width;
        let image_height = self.settings.image_settings.image_height;
        self.image_data.pixel_colors =  vec![Color::new(0.0,0.0,0.0); image_height * image_width];
        self.image_data.samples = 0;
    }

    pub fn capture_user_input(&mut self, ctx: &egui::CtxRef) {
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


            self.transmit_settings();
            self.transmit_message(Message {instructions: Instructions::ChangeSettings, priority: Priority::Next});
            self.refresh_image();
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
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        //let Self {thread_output_rx, thread_input_tx, input_data, image_data, labels, count} = self;
        
        self.capture_user_input(ctx);
        
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
                            self.settings.image_settings.image_width = num;
                            self.transmit_settings();
                            self.transmit_message(Message {instructions: Instructions::ChangeSettings, priority: Priority::Now});
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
                            self.transmit_settings();
                            self.transmit_message(Message {instructions: Instructions::ChangeSettings, priority: Priority::Next});
                        }
                        Err(_) => {
                            self.labels.height = self.settings.image_settings.image_height.to_string();
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
                            if num < self.settings.raytrace_settings.samples_per_pixel {
                                self.image_data.pixel_colors =  vec![Color::new(0.0,0.0,0.0); self.settings.image_settings.image_height * self.settings.image_settings.image_width];
                                self.image_data.samples = 0;
                                self.transmit_message(Message {instructions: Instructions::ChangeSettings, priority: Priority::Now});

                            }
                            self.settings.raytrace_settings.samples_per_pixel = num;
                        }
                        Err(_) => {
                            self.labels.samples = self.settings.raytrace_settings.samples_per_pixel.to_string();
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
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let message_result = self.thread_output_rx.try_recv();
            if let Ok(message) = message_result {
                if message.id > self.recieved_id {  
                    self.recieved_id = message.id;
                    self.image_data = message;
                }
                else if self.image_data.samples < self.settings.raytrace_settings.samples_per_pixel {
                    for i in 0..self.image_data.pixel_colors.len(){
                        self.image_data.pixel_colors[i] = self.image_data.pixel_colors[i] + message.pixel_colors[i];
                    }
                    self.image_data.samples += message.samples;
                }
                if self.image_data.samples >= self.settings.raytrace_settings.samples_per_pixel {
                    self.transmit_message(Message {instructions: Instructions::Pause, priority: Priority::Now});
                }
            }

            let rgbas = colors_to_rgba(&self.image_data.pixel_colors, self.image_data.samples.max(1));
            let image = epi::Image::from_rgba_unmultiplied([self.image_data.image_width, self.image_data.image_height], &rgbas);
            // The central panel the region left after adding TopPanel's and SidePanel's
            let texture_id = frame.alloc_texture(image);
            ui.image(texture_id, [self.image_data.image_width as f32, self.image_data.image_height as f32]);
        });

        if self.expecting_data {
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

