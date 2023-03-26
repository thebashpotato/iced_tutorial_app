//! The counter page view is rendered here. This structure is used in the main
//! application structure.

use iced::widget::{Button, Column, Container, Text};
use iced::Element;

use crate::ui::{AppMessage, Navigation};

/// Represent the counter page where the user can manipulate the applications
/// state.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct CounterPage {
    /// Page text label.
    label: String,
    /// Padding between widgets in column.
    padding: u16,
}

impl Default for CounterPage {
    fn default() -> Self {
        Self {
            label: String::from("Current count"),
            padding: 10,
        }
    }
}

impl CounterPage {
    /// This is where we draw our GUI for the counter page. Should return an
    /// iced element that will occupy our view.
    ///
    /// # Arguments
    /// * `counter` the state variable is passed through from the main app
    ///    that implements the Sandbox trait. This is a nice way to keep our state
    ///    separate, but pass it around when needed.
    ///
    /// # Returns
    ///   An [`iced::Element`] which contains the [`crate::ui::AppMessage`]. The
    ///   [`crate::app::App`] structures view will use this view to render it into iced.
    pub fn view(&self, counter: i32) -> Element<'_, AppMessage> {
        Container::new(
            Column::new()
                .push(Text::new(&self.label))
                .push(Text::new(format!("{counter}")))
                .spacing(self.padding)
                .push(Button::new("Increment").on_press(AppMessage::Increment))
                .spacing(self.padding)
                .push(Button::new("Decrement").on_press(AppMessage::Decrement))
                .spacing(self.padding)
                .push(
                    Button::new(Text::new("Back to the main page"))
                        .on_press(AppMessage::Router(Navigation::MainPage)),
                )
                .spacing(self.padding),
        )
        .center_x()
        .center_y()
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
    }
}
