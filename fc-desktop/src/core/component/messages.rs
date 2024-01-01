use crate::core::component::Component;
use eframe::egui;
use eframe::egui::{
    CentralPanel, Color32, Context, FontId, RichText, Rounding, ScrollArea, Stroke, TextEdit, Ui,
};
use egui_extras::{Size, StripBuilder};

#[derive(Default)]
pub struct Messages {
    messages: Vec<String>,
    message_buffer: String,
}

impl Component for Messages {
    fn show(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::remainder())
                .size(Size::exact(70.0))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ScrollArea::vertical().show(ui, |ui| {
                            for s in mock() {
                                ui.label(RichText::new(s).font(FontId::proportional(20.0)));
                            }
                        });
                    });
                    strip.cell(|ui| {
                        ui.add(
                            TextEdit::multiline(&mut self.message_buffer)
                                .desired_width(f32::INFINITY),
                        );
                    })
                });
        });
    }
}

fn mock() -> Vec<&'static str> {
    vec![
        "mock", "mock", "mock", "mock", "mock", "mock", "mock", "mock", "mock", "mock", "mock",
        "mock",
    ]
}

fn rect(ui: &mut Ui, color: Color32) {
    ui.painter().rect(
        ui.available_rect_before_wrap(),
        Rounding::default(),
        color,
        Stroke::default(),
    );
}
