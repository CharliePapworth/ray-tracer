use eframe::egui::{Image, Context, Ui, Response, self};

pub trait Label {
    fn LabelName(&self) -> String;
    fn LabelValue(&self) -> String;
    /// Validates the user inputted value. If it's fine,
    /// updates the settings.
    fn ValidateLabelValue(&mut self);
    fn AddLabel(&self, ctx: &Context, ui: &mut Ui) -> Response 
    {
        ui.label(self.LabelValue());
        let response = ui.add(egui::TextEdit::singleline(&mut self.LabelValue()));
        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
            self.ValidateLabelValue();
        }
        response
    }
}

pub struct ImageHeight {
    pub label_name: String,
    pub label_value: String,
    pub height: usize,
}

pub struct ImageWidth {
    pub label_name: String,
    pub label_value: String,
    pub width: usize,
}

pub struct CameraSpeed {
    pub label_name: String,
    pub label_value: String,
    pub speed: usize,
}

pub struct SamplesPerPixel {
    pub label_name: String,
    pub label_value: String,
    pub samples: i32,
}

impl Label for ImageHeight {
    fn LabelName(&self) -> String {
        self.label_name
    }

    fn LabelValue(&self) -> String {
        self.label_value
    }

    fn ValidateLabelValue(&mut self) {
        match self.label_value.parse::<usize>() {
            Ok(num) => {
                self.height = num;
            }
            Err(_) => {
                self.label_value = self.height.to_string();
            }
        }
    }
}

impl Label for ImageWidth {
    fn LabelName(&self) -> String {
        self.label_name
    }

    fn LabelValue(&self) -> String {
        self.label_value
    }

    fn ValidateLabelValue(&mut self) {
        match self.label_value.parse::<usize>() {
            Ok(num) => {
                self.width = num;
            }
            Err(_) => {
                self.label_value = self.width.to_string();
            }
        }
    }
}

impl Label for CameraSpeed {
    fn LabelName(&self) -> String {
        self.label_name
    }

    fn LabelValue(&self) -> String {
        self.label_value
    }

    fn ValidateLabelValue(&mut self) {
        match self.label_value.parse::<usize>() {
            Ok(num) => {
                self.speed = num;
            }
            Err(_) => {
                self.label_value = self.speed.to_string();
            }
        }
    }
}

impl Label for SamplesPerPixel {
    fn LabelName(&self) -> String {
        self.label_name
    }

    fn LabelValue(&self) -> String {
        self.label_value
    }

    fn ValidateLabelValue(&mut self) {
        match self.label_value.parse::<i32>() {
            Ok(num) => {
                self.samples = num;
            }
            Err(_) => {
                self.label_value = self.samples.to_string();
            }
        }
    }
}