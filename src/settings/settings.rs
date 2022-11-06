use super::SettingsPatch;

pub struct Settings {
    camera_speed: f32,
    samples_per_pixel: u32,
    image_height: usize,
    image_width: usize,
    settings_patch: SettingsPatch,
}

impl Settings {
    pub fn get_patch(&mut self) -> SettingsPatch {
        let returned_patch = self.settings_patch.clone();
        self.settings_patch = SettingsPatch::default();
        returned_patch
    }

    pub fn update_camera_speed(&mut self, new_camera_speed: f32) {
        if new_camera_speed > 0.0 {
            self.settings_patch.camera_speed = Some(new_camera_speed);
            self.camera_speed = new_camera_speed;
        }
    }

    pub fn get_camera_speed(&self) -> f32 {
        return self.camera_speed;
    }

    pub fn update_samples_per_pixel(&mut self, new_samples_per_pixel: u32) {
        self.settings_patch.samples_per_pixel = Some(new_samples_per_pixel);
        self.samples_per_pixel = new_samples_per_pixel;
    }

    pub fn get_samples_per_pixel(&self) -> u32 {
        self.samples_per_pixel
    }

    pub fn update_image_height(&mut self, new_image_height: usize) {
        self.image_height = new_image_height;
    }

    pub fn get_image_height(&self) -> usize {
        self.image_height
    }

    pub fn update_image_width(&mut self, new_image_width: usize) {
        self.image_height = new_image_width;
    }

    pub fn get_image_width(&self) -> usize {
        self.image_width
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            camera_speed: 1.0,
            samples_per_pixel: 10,
            image_height: 10,
            image_width: 10,
            settings_patch: Default::default(),
        }
    }
}
