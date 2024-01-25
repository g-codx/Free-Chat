use eframe::egui;
use eframe::egui::{Context, Layout, RichText, SidePanel, TopBottomPanel};
use eframe::epaint::FontId;

#[derive(Default)]
pub struct Bar {
    room_name_buffer: String,
}

impl super::Component for Bar {
    fn show(&mut self, ctx: &Context) {
        TopBottomPanel::top("bar").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(10.0);

                ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.add_space(5.0);
                    ui.menu_button(RichText::new("⛭").size(20.0), |ui| {
                        if ui.button("Theme").clicked() {
                            dbg!("Theme button");
                        }

                        if ui.button("Settings").clicked() {
                            dbg!("Settings button");
                        }
                    });
                    ui.add_space(5.0);

                    if ui.button(RichText::new("🔍").size(20.0)).clicked() {
                        dbg!();
                    }

                    ui.text_edit_singleline(&mut self.room_name_buffer);
                });

                ui.add_space(10.0);
            });
        });
    }
}
