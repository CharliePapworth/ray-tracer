use eframe::{egui::{self, Vec2}, epi};

use crate::vec::*;
use crate::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct Gui {
    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    pub thread_output_rx: Receiver<ImageData>,
    pub thread_input_tx: Vec<Sender<InputData>>,
    pub input_data: InputData,
    pub image_data: ImageData,
    pub labels: Labels,
    pub count: i32
}

impl Gui{
    pub fn new(thread_output_rx: Receiver<ImageData>,  thread_input_tx: Vec<Sender<InputData>>, input_data: InputData) -> Gui {
        let labels = Labels{width: input_data.image_width.to_string(), height: input_data.image_height.to_string()};
        let image_data = ImageData{pixel_colors: vec![Color::new(0.0,0.0,0.0); input_data.image_height * input_data.image_width], image_width: input_data.image_width, image_height: input_data.image_height, samples: 0};
        let count = 0;
        Gui{thread_output_rx, thread_input_tx, input_data, image_data, labels, count}
    }
}

#[derive(Default)]
pub struct Labels{
    width: String,
    height: String
}


impl epi::App for Gui {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }


    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self {thread_output_rx, thread_input_tx, input_data, image_data, labels, count} = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                    if ui.button("Save").clicked() {
                        let path = "results.ppm";
                        let mut file = initialise_file(path, image_data.image_width, image_data.image_height);
                        for pixel in image_data.pixel_colors.iter() {
                            pixel.write_color(&mut file, image_data.samples);
                         }
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Width:");
                let width_response =  ui.add_sized(Vec2::new(30f32, 20f32), egui::TextEdit::singleline(&mut labels.width));
                if width_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    match labels.width.parse::<usize>(){
                        Ok(num) => {
                            input_data.image_width = num;
                            input_data.done = false;
                            transmit(thread_input_tx, *input_data);
                        }
                        Err(_) => {
                            labels.width = input_data.image_width.to_string();
                        }
                    }
                }
                ui.label("Height:");
                let height_response =  ui.add_sized(Vec2::new(30f32, 20f32), egui::TextEdit::singleline(&mut labels.height));
                if height_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    match labels.height.parse::<usize>(){
                        Ok(num) => {
                            input_data.image_height = num;
                            input_data.done = false;
                            transmit(thread_input_tx, *input_data);
                        }
                        Err(_) => {
                            labels.height = input_data.image_height.to_string();
                        }
                    }
                }
            });

            let mut value = 2f32;
            ui.add(egui::Slider::new(&mut value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                value += 1.0;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let message_result = thread_output_rx.try_recv();
            if let Ok(message) = message_result {
                if message.image_height != image_data.image_height || message.image_width != image_data.image_width {     
                    *image_data = message;
                }
                else {
                    for i in 0..image_data.pixel_colors.len(){
                        image_data.pixel_colors[i] = image_data.pixel_colors[i] + message.pixel_colors[i];
                    }
                    image_data.samples += message.samples;
                    if image_data.samples == input_data.samples_per_pixel {
                        input_data.done = true;
                        transmit(thread_input_tx, *input_data);
                    }
                }
            }

            let rgbas = colors_to_rgba(&image_data.pixel_colors, image_data.samples.max(1));
            let image = epi::Image::from_rgba_unmultiplied([image_data.image_width, image_data.image_height], &rgbas);
            // The central panel the region left after adding TopPanel's and SidePanel's
            let texture_id = frame.alloc_texture(image);
            ui.image(texture_id, [image_data.image_width as f32, image_data.image_height as f32]);
        });

        if !input_data.done {
            ctx.request_repaint();
        }
        *count += 1;
        println!{"{}", count};

    }
}

pub fn transmit(thread_input_tx: &Vec<Sender<InputData>>, input_data: InputData){
    for transmitter in thread_input_tx{
        transmitter.send(input_data);
    }
}

pub fn colors_to_rgba(colors: &Vec<Color>, samples: usize) -> Vec<u8>{
    let mut rgbas = Vec::<u8>::with_capacity(colors.len() * 4);
    for color in colors{
     let rgb = color.scale_colors(samples);
        for i in 0..3{
            rgbas.push(rgb[i]);
        }
        rgbas.push(255);
    }
    rgbas
}

