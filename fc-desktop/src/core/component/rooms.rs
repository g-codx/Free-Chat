use crate::core::component::dialog::CreateRoomDialog;
use crate::core::component::main::Event;
use crate::core::component::{Component, Dialog};
use crate::core::widget::card::Card;
use crate::http::model::Room;
use crate::{http, WsMessage};
use eframe::egui::{Context, Id, ScrollArea, Sense, SidePanel};
use fc_command::{encode, Command};
use std::path::PathBuf;
use std::sync::mpsc::Sender;

pub struct Rooms {
    app_sender: Sender<WsMessage>,
    event_sender: Sender<Event>,
    rooms: Vec<Room>,
    modal: CreateRoomDialog,
}

impl Rooms {
    pub fn new(app_sender: Sender<WsMessage>, event_sender: Sender<Event>) -> Self {
        Self {
            app_sender,
            event_sender,
            rooms: get_rooms_request(),
            modal: CreateRoomDialog::new("Enter the room name".to_string()),
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
                if self.modal.show(ctx) {
                    let message = encode(Command::CreateRoom(self.modal.buffer())).unwrap();

                    match self.app_sender.send(WsMessage::binary(message)) {
                        Ok(_) => {}
                        Err(err) => log::error!("{}", err),
                    }
                }

                ScrollArea::vertical().show(ui, |ui| {
                    for r in &self.rooms {
                        let response = ui.add(Card::new(
                            Id::new(r.id),
                            r.name.clone(),
                            r.last_message.clone(),
                            PathBuf::default(),
                        ));

                        if response.clicked() {
                            self.event_sender.send(Event::NewTarget(r.clone())).unwrap();
                        }

                        response.context_menu(|ui| {
                            if ui.button("Leave room").clicked() {
                                dbg!("");
                            }

                            dbg!("");
                        });
                    }
                });

                let rect = ui.available_rect_before_wrap();
                let response = ui.interact(rect, Id::new("left_panel_space"), Sense::click());

                response.context_menu(|ui| {
                    if ui.button("Add room").clicked() {
                        self.modal.open();
                        ui.close_menu();
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
