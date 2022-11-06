use std::{str::FromStr, fmt::Display};

use eframe::egui::{Image, Context, Ui, Response, self};

use crate::settings::Settings;

pub struct ValidatedTextInput {
    pub name: String,
    pub text: String,
}

impl ValidatedTextInput {

    pub fn new<V>(name: String, initial_value: V) -> Self
    where V: FromStr + Display {
        Self {
            name,
            text: initial_value.to_string()
        }
    }
    /// Validates the user inputted value. If it's fine,
    /// updates the settings. If not, reverts to the previous state.
    fn try_update<V>(&mut self, settings: &mut Settings, update_settings: impl FnOnce(V, &mut Settings), get_settings: impl Fn(&mut Settings) -> V) 
    where V: FromStr + Display {
        if let Ok(parsed_input) = self.text.parse::<V>() {
            update_settings(parsed_input, settings);
        }
        self.text = get_settings(settings).to_string();
    }

    pub fn add<V>(&mut self, settings: &mut Settings, ui: &mut Ui, update_settings: impl FnOnce(V, &mut Settings), get_settings: impl Fn(&mut Settings) -> V) -> Response
    where V: FromStr + Display 
    {
        ui.label(&self.text);
        let response = ui.add(egui::TextEdit::singleline(&mut self.text));
        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
           self.try_update(settings, update_settings, get_settings);
        }

        response
    }
}