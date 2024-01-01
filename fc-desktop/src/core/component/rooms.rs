use crate::core::component::Component;
use crate::core::widget::card::Card;
use crate::http;
use crate::http::model::Room;
use eframe::egui::{Context, Id, ScrollArea, SidePanel};
use std::path::PathBuf;

pub struct Rooms {
    rooms: Vec<Room>,
}

impl Default for Rooms {
    fn default() -> Self {
        Self {
            rooms: get_rooms_request(),
        }
    }
}

impl Component for Rooms {
    fn show(&mut self, ctx: &Context) {
        SidePanel::left("rooms_list")
            .min_width(300.0)
            .max_width(600.0)
            .resizable(false)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    for r in &self.rooms {
                        ui.add(Card::new(
                            Id::new(r.id),
                            r.name.clone(),
                            r.last_message.clone(),
                            PathBuf::default(),
                        ));
                    }
                });
            });
    }
}

fn get_rooms_request() -> Vec<Room> {
    match http::service::get_rooms() {
        Ok(response) => response.rooms,
        Err(err) => {
            log::error!("{}", err);
            vec![]
        }
    }
}
