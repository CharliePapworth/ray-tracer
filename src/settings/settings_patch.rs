#[derive(Default, Clone)]
pub struct SettingsPatch {
    pub(super) camera_speed: Option<f32>,
    pub(super) samples_per_pixel: Option<u32>,
}

impl SettingsPatch {}
