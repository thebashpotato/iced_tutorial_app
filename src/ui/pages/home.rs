//! The main page view is rendered here. This structure is used in the main
//! application structure.

use iced::widget::{Button, Column, Container, Text};
use iced::{Element, Length};

use crate::ui::{AppMessage, Navigation};

/// Represent the main or home page.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct HomePage {
    /// Text for label.
    label_text: String,
    /// Padding between widgets in column.
    padding: u16,
}

impl Default for HomePage {
    fn default() -> Self {
        Self {
            label_text: String::from("Welcome to the Counter App"),
            padding: 15,
        }
    }
}

impl HomePage {
    /// This is where we draw our GUI for the main page. Should return an iced
    /// element that will occupy our view. It doesn't do much, but provide a
    /// button to go to the counter page.
    ///
    /// # Returns
    ///   An [`iced::Element`] which contains the [`crate::ui::AppMessage`]. The
    ///   [`crate::app::App`] structures view method will use this view to render it into iced.
    pub fn view(&self) -> Element<'_, AppMessage> {
        Container::new(
            Column::new()
                .push(Text::new(&self.label_text))
                .spacing(self.padding)
                .push(
                    Button::new(Text::new("Start Counting! It's fun!"))
                        .on_press(AppMessage::Router(Navigation::CounterPage)),
                ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
