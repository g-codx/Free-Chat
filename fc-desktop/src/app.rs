use crate::core::component::main::Main;
use crate::core::component::Component;
use eframe::egui::Context;
use eframe::Frame;
use std::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::tungstenite::Message;

pub struct FcApp {
    main: Main,
}

impl FcApp {
    pub fn new(app_sender: Sender<Message>, app_receiver: Receiver<Message>) -> Self {
        Self {
            main: Main::new(app_sender, app_receiver),
        }
    }

    pub fn run(self) -> eframe::Result<()> {
        let native_options = eframe::NativeOptions {
            initial_window_size: Some([1300.0, 900.0].into()),
            ..Default::default()
        };

        eframe::run_native("fc-desktop", native_options, Box::new(|_| Box::new(self)))
    }
}

impl eframe::App for FcApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.main.show(ctx);
    }
}
