use crate::http::model::Room;
use eframe::egui::{Id, Response, RichText, Sense, Ui, Widget};
use egui_extras::{Size, StripBuilder};
use std::path::PathBuf;

pub struct Card {
    id: Id,
    name: String,
    content: String,
    logo: PathBuf,
}

impl From<Room> for Card {
    fn from(value: Room) -> Self {
        Self {
            id: Id::new(value.id),
            name: value.name,
            content: value.last_message,
            logo: PathBuf::default(),
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
        }
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
    fn ui(self, ui: &mut Ui) -> Response {
        let builder = StripBuilder::new(ui)
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

        ui.interact(builder.rect, Id::new(self.id), Sense::click())
    }
}
