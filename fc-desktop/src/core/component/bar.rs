use eframe::egui::{Context, SidePanel, TopBottomPanel};

#[derive(Default)]
pub struct Bar;

impl super::Component for Bar {
    fn show(&mut self, ctx: &Context) {
        TopBottomPanel::top("bar").show(ctx, |ui| {
            if ui.button("THIS IS BUTTON").clicked() {
                dbg!("CHPONK");
            }
        });
    }
}
