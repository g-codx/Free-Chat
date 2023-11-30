use crate::core::component::Component;
use crate::core::widget::card::Card;
use eframe::egui;
use eframe::egui::{
    Color32, Context, Id, RichText, Rounding, ScrollArea, Sense, SidePanel, Stroke, Ui,
};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use std::path::PathBuf;

#[derive(Default)]
pub struct Rooms;

impl Component for Rooms {
    fn show(&mut self, ctx: &Context) {
        SidePanel::left("rooms_list")
            .min_width(300.0)
            .max_width(600.0)
            .resizable(false)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    for i in 0..25 {
                        ui.add(Card::new(Id::new(i), name(), content(), PathBuf::default()));
                    }
                });
            });
    }
}

fn name() -> String {
    "TEXT TEXT TEXT TEXT text text text text".to_string()
}

fn content() -> String {
    "TEXT TEXT TEXT TEXT text text text text TEXT TEXT TEXT TEXT text text text text TEXT TEXT TEXT TEXT text text text text TEXT TEXT TEXT TEXT text text text text".to_string()
}
