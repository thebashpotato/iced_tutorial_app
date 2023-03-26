//! All application logic is here. Following the code in this file is the best
//! way to learn how this counter application works.

mod state;
use iced::{Element, Sandbox};
use state::State;

use crate::ui::{AppMessage, CounterPage, MainPage, Navigation};

/// The final application, it holds the state, page routing, and the actual
/// pages the load into the view.
#[derive(Default)]
pub struct App {
    /// Internal state of the application.
    state: State,
    /// Trigger to route to a different page.
    current_route: Navigation,
    /// The main page view.
    main_page: MainPage,
    /// The counter page view.
    counter_page: CounterPage,
}

impl Sandbox for App {
    /// This is a type alias to the message type we are going to use in our
    /// application.
    type Message = AppMessage;

    /// Initializes the state of our application.
    fn new() -> Self {
        Self::default()
    }

    /// Sets the title of the application.
    fn title(&self) -> String {
        String::from("Counter App")
    }

    /// Will receive messages of the type we defined earlier and mutate the
    /// state our application..
    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::Increment => {
                self.state.update_count(1);
            }
            AppMessage::Decrement => {
                self.state.update_count(-1);
            }
            AppMessage::Router(view) => {
                self.current_route = view;
            }
        }
    }

    /// This is where we draw our GUI. Should return an iced element that will
    /// occupy our view.
    fn view(&self) -> Element<'_, Self::Message> {
        match self.current_route {
            Navigation::MainPage => self.main_page.view(),
            Navigation::CounterPage => self.counter_page.view(*self.state.counter()),
        }
    }
}
