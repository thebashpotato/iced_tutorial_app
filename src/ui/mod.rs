//! Application routing and messaging enumerations.

mod pages;
pub use pages::{CounterPage, HomePage};

/// Used for navigation or 'routing' between pages in the application.
#[derive(Debug, Clone, Copy)]
pub enum Navigation {
    /// Applications home page.
    HomePage,
    /// Displays the count.
    CounterPage,
}

impl Default for Navigation {
    fn default() -> Self {
        Self::HomePage
    }
}

/// The types of message's the counter application can handle.
#[derive(Debug, Clone, Copy)]
pub enum AppMessage {
    /// Used to send a signal to increment the state counter.
    Increment,
    /// Used to send a signal to decrements the state counter.
    Decrement,
    /// Route to a different page of the application.
    Router(Navigation),
}
