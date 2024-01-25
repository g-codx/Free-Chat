use crate::core::component::main::Event;
use crate::core::component::Component;
use crate::{http, WsMessage};
use crate::http::model::{Message, Room};
use eframe::egui;
use eframe::egui::{
    CentralPanel, Color32, Context, FontId, Key, RichText, Rounding, ScrollArea, Stroke, TextEdit,
    Ui,
};
use egui_extras::{Size, StripBuilder};
use fc_command::{encode, Command};
use std::sync::mpsc::{Receiver, SendError, Sender, TryRecvError};
use std::thread;



pub struct Messages {
    app_sender: Sender<WsMessage>,
    event_receiver: Receiver<Event>,
    messages: Vec<Message>,
    message_buffer: String,
    target: Option<Room>,
}

impl Messages {
    pub fn new(app_sender: Sender<WsMessage>, event_receiver: Receiver<Event>) -> Self {
        Self {
            app_sender,
            event_receiver,
            messages: vec![],
            message_buffer: "".to_string(),
            target: None,
        }
    }
}

impl Component for Messages {
    fn show(&mut self, ctx: &Context) {
        if let Ok(event) = self.event_receiver.try_recv() {
            match event {
                Event::NewTarget(room) => {
                    self.messages = get_messages_request(room.id);
                    self.target = Some(room);
                }
            }
        }

        CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::remainder())
                .size(Size::exact(70.0))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ScrollArea::vertical().show(ui, |ui| {
                            for s in &self.messages {
                                ui.label(
                                    RichText::new(s.get_content()).font(FontId::proportional(20.0)),
                                );
                            }
                        });
                    });
                    strip.cell(|ui| {
                        let response = ui.add(
                            TextEdit::multiline(&mut self.message_buffer)
                                .desired_width(f32::INFINITY),
                        );

                        if response.has_focus()
                            && ui.input(|i| i.key_down(Key::Enter))
                            && !self.message_buffer.is_empty()
                        {
                            if let Some(target_room) = &self.target {
                                let text = self.message_buffer.trim().to_string();

                                let message =
                                    encode(Command::SendMessage(target_room.id, text.into_bytes()))
                                        .unwrap();

                                match self.app_sender.send(WsMessage::binary(message)) {
                                    Ok(_) => {}
                                    Err(err) => log::error!("{}", err),
                                }
                            }

                            self.message_buffer.clear();
                        }
                    })
                });
        });
    }
}

fn get_messages_request(room_id: i64) -> Vec<Message> {
    match http::service::get_messages(room_id) {
        Ok(response) => response.messages,
        Err(err) => {
            log::error!("{}", err);
            vec![]
        }
    }
}

fn rect(ui: &mut Ui, color: Color32) {
    ui.painter().rect(
        ui.available_rect_before_wrap(),
        Rounding::default(),
        color,
        Stroke::default(),
    );
}
