pub mod console_egui {
    use console::ui;
    use console::Console;

    pub struct EguiConsole {}

    impl Console for EguiConsole {
        fn new() -> Self {
            EguiConsole {}
        }
    }

    impl ui::ConsoleUI for EguiConsole {
        fn is_open(&self) -> bool {
            false
        }

        fn toggle(&self) {
            todo!()
        }

        fn update_toggle_animation(&self) {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::console_egui::*;
    use console::{ui::ConsoleUI, Console};

    #[test]
    fn check_console_hidden_by_default() {
        let console = EguiConsole::new();
        assert_eq!(console.is_open(), false);
    }
}
