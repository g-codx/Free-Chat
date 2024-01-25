use eframe::egui;

pub mod bar;
mod dialog;
pub mod main;
mod messages;
mod rooms;

pub trait Component {
    fn show(&mut self, ctx: &egui::Context);
}

pub trait Dialog {
    fn show(&mut self, ctx: &egui::Context) -> bool;

    fn buffer(&mut self) -> String;
}
