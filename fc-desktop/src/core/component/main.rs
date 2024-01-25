use crate::core::component::bar::Bar;
use crate::core::component::messages::Messages;
use crate::core::component::rooms::Rooms;
use crate::http::model::Room;
use eframe::egui::Context;
use futures_channel::mpsc::UnboundedReceiver;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use tokio_tungstenite::tungstenite::Message;

pub struct Main {
    bar: Bar,
    rooms: Rooms,
    messages: Messages,
    app_receiver: Receiver<Message>,
}

impl Main {
    pub fn new(app_sender: Sender<Message>, app_receiver: Receiver<Message>) -> Self {
        let component_channel = mpsc::channel();
        Self {
            bar: Default::default(),
            rooms: Rooms::new(app_sender.clone(), component_channel.0.clone()),
            messages: Messages::new(app_sender.clone(), component_channel.1),
            app_receiver,
        }
    }
}

impl super::Component for Main {
    fn show(&mut self, ctx: &Context) {
        if let Ok(message) = self.app_receiver.try_recv() {
            dbg!("MAIN GOT A MESSAGE: {}", message);
        }

        self.bar.show(ctx);
        self.rooms.show(ctx);
        self.messages.show(ctx);
    }
}

#[derive(Debug)]
pub enum Event {
    NewTarget(Room),
}
