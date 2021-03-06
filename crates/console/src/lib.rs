pub trait Console {
    fn new() -> Self;
}

pub mod ui {
    /// The visual state the console is in.
    pub enum ConsoleUIState {
        /// The console is open.
        Open,
        /// The console is closed.
        Closed,
    }

    pub trait ConsoleUI {
        /// Check if the console is currently being displayed.
        fn is_open(&self) -> bool;

        /// Toggle visibility of the console.
        fn toggle(&mut self);

        /// Called to update the console's position when showing/hiding.
        fn update_toggle_animation(&self);
    }
}
