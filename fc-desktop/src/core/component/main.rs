use crate::core::component::bar::Bar;
use crate::core::component::messages::Messages;
use crate::core::component::rooms::Rooms;
use eframe::egui::Context;

#[derive(Default)]
pub struct Main {
    bar: Bar,
    rooms: Rooms,
    messages: Messages,
}

impl super::Component for Main {
    fn show(&mut self, ctx: &Context) {
        self.bar.show(ctx);
        self.rooms.show(ctx);
        self.messages.show(ctx);
    }
}
