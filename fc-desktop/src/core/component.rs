use eframe::egui;

pub mod bar;
pub mod main;
mod messages;
mod rooms;

pub trait Component {
    fn show(&mut self, ctx: &egui::Context);
}
