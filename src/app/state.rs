//! Holds the state of the entire application. This separates the state from the
//! user interface crate as well as routing/message passing, as well as application business logic.

/// Any application state variables would be added here.
#[derive(Debug, Default, Clone, Copy)]
pub struct State {
    /// The only state we have in this simple app.
    counter: i32,
}

impl State {
    /// Get the current count
    #[inline]
    pub const fn counter(&self) -> &i32 {
        &self.counter
    }

    /// Update the state of the current count.
    ///
    /// # Arguments
    /// * `rhs` the amount to increment or decrements by.
    #[inline]
    pub fn update_count(&mut self, rhs: i32) {
        self.counter += rhs;
    }
}
