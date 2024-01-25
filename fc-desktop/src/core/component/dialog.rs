use crate::core::component::{Component, Dialog};
use crate::WsMessage;
use eframe::egui;
use eframe::egui::{Align2, Context, InnerResponse, Key, Vec2};
use fc_command::{encode, Command};

pub struct CreateRoomDialog {
    action_label: String,
    buffer: String,
    open: bool,
    enter: bool,
}

impl CreateRoomDialog {
    pub fn new(action_label: String) -> Self {
        Self {
            action_label,
            buffer: String::default(),
            open: false,
            enter: false,
        }
    }

    pub fn open(&mut self) {
        self.open = true;
    }
}

impl Dialog for CreateRoomDialog {
    fn show(&mut self, ctx: &Context) -> bool {
        let response = egui::Window::new(self.action_label.clone())
            .open(&mut self.open)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                let response = ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.buffer);
                    ui.input(|i| i.key_down(Key::Enter)) && !self.buffer.is_empty()
                });

                response.inner
            });

        if let Some(inner) = response {
            let result = inner.inner.unwrap_or(false);

            if result {
                self.open = false;
            }

            result
        } else {
            false
        }
    }

    fn buffer(&mut self) -> String {
        let buffer = self.buffer.clone();
        self.buffer.clear();
        buffer
    }
}
