use crate::core::component::main::Main;
use crate::core::component::Component;
use eframe::egui::Context;
use eframe::Frame;

#[derive(Default)]
pub struct FcApp {
    main: Main,
}

impl eframe::App for FcApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.main.show(ctx);
    }
}
