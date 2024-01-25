use crate::http::model::Room;
use eframe::egui::{Color32, Frame, Id, Response, RichText, Sense, Stroke, Style, Ui, Widget};
use egui_extras::{Size, StripBuilder};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Card {
    id: Id,
    name: String,
    content: String,
    logo: PathBuf,
    state: CardState,
}

#[derive(Clone)]
pub enum CardState {
    UnHovered,
    Hovered,
}

impl CardState {
    pub fn get_stroke(&self) -> Stroke {
        match self {
            CardState::UnHovered => Stroke::new(1.0, Color32::LIGHT_BLUE),
            CardState::Hovered => Stroke::new(1.0, Color32::LIGHT_YELLOW),
        }
    }
}

impl From<Room> for Card {
    fn from(value: Room) -> Self {
        Self {
            id: Id::new(value.id),
            name: value.name,
            content: value.last_message,
            logo: PathBuf::default(),
            state: CardState::UnHovered,
        }
    }
}

impl Card {
    pub fn new(id: Id, name: String, content: String, logo: PathBuf) -> Self {
        Self {
            id,
            name,
            content,
            logo,
            state: CardState::UnHovered,
        }
    }

    pub fn save_state(&self, ui: &mut Ui) {
        ui.data_mut(|d| d.insert_temp(self.id, self.state.clone()))
    }

    pub fn load_state(&self, ui: &mut Ui) -> Option<CardState> {
        ui.data(|d| d.get_temp(self.id))
    }

    fn card_name(&self) -> RichText {
        let text = if self.name.len() > 20 {
            format!("{}..", &self.name[0..23])
        } else {
            self.name.clone()
        };

        RichText::new(text).size(15.0)
    }

    fn card_content(&self) -> RichText {
        let text = if self.content.len() > 80 {
            format!("{}..", &self.content[0..80])
        } else {
            self.content.clone()
        };

        RichText::new(text).size(10.0)
    }
}

impl Widget for Card {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let stroke = self
            .load_state(ui)
            .unwrap_or(CardState::UnHovered)
            .get_stroke();

        let response = Frame::window(&Style::default())
            .stroke(stroke)
            .show(ui, |ui| {
                let builder =
                    StripBuilder::new(ui)
                        .size(Size::exact(60.0))
                        .vertical(|mut strip| {
                            strip.strip(|builder| {
                                builder
                                    .size(Size::exact(50.0))
                                    .size(Size::exact(250.0))
                                    .horizontal(|mut strip| {
                                        strip.cell(|ui| {
                                            ui.vertical_centered(|ui| {
                                                ui.add_space(10.0);
                                                ui.label(RichText::new("").size(50.0));
                                            });
                                        });
                                        strip.strip(|builder| {
                                            builder
                                                .size(Size::exact(20.0))
                                                .size(Size::exact(10.0))
                                                .vertical(|mut strip| {
                                                    strip.cell(|ui| {
                                                        ui.add_space(10.0);
                                                        ui.label(self.card_name());
                                                    });
                                                    strip.cell(|ui| {
                                                        ui.add_space(10.0);
                                                        ui.label(self.card_content());
                                                    });
                                                });
                                        });
                                    });
                            });
                        });

                let response = ui.interact(builder.rect, Id::new(self.id), Sense::click());

                if response.hovered() {
                    self.state = CardState::Hovered;
                } else {
                    self.state = CardState::UnHovered;
                }

                self.save_state(ui);

                response
            });

        response.inner
    }
}
