use crate::core::component::Component;
use eframe::egui::{CentralPanel, Context, ScrollArea};

#[derive(Default)]
pub struct Messages;

impl Component for Messages {
    fn show(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::horizontal().show(ui, |ui| {
                ui.label("HELLO JOPA");
                ui.label("NU ZDAROVA LOH");
            });
            
            ui.text_edit_multiline(&mut "")
        });
    }
}
