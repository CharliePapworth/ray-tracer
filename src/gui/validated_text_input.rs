use std::{fmt::Display, str::FromStr};

use eframe::egui::{self, Context, Image, Response, Ui};

use crate::settings::Settings;

pub struct ValidatedTextInput<V>
where
    V: FromStr + Display,
{
    pub name: String,
    pub text: String,
    update_settings: Box<dyn FnMut(V, &mut Settings)>,
    get_settings: Box<dyn Fn(&Settings) -> V>,
}

impl<V> ValidatedTextInput<V>
where
    V: FromStr + Display,
{
    pub fn new(
        name: String,
        settings: &Settings,
        update_settings: impl FnMut(V, &mut Settings) + 'static,
        get_settings: impl Fn(&Settings) -> V + 'static,
    ) -> Self
    where
        V: FromStr + Display,
    {
        Self {
            name,
            text: get_settings(settings).to_string(),
            update_settings: Box::new(update_settings),
            get_settings: Box::new(get_settings),
        }
    }
    /// Validates the user inputted value. If it's fine,
    /// updates the settings. If not, reverts to the previous state.
    fn try_update(&mut self, settings: &mut Settings)
    where
        V: FromStr + Display,
    {
        if let Ok(parsed_input) = self.text.parse::<V>() {
            (self.update_settings)(parsed_input, settings);
        }
        self.text = (self.get_settings)(settings).to_string();
    }

    pub fn add(&mut self, settings: &mut Settings, ui: &mut Ui) -> Response
    where
        V: FromStr + Display,
    {
        ui.label(&self.name);
        let response = ui.add(egui::TextEdit::singleline(&mut self.text));
        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
            self.try_update(settings);
        }

        response
    }
}
