pub mod console_egui {
    use console::ui;
    use console::Console;

    pub struct EguiConsole {
        is_console_open: bool,
    }

    impl Console for EguiConsole {
        fn new() -> Self {
            EguiConsole {
                is_console_open: false,
            }
        }
    }

    impl ui::ConsoleUI for EguiConsole {
        fn is_open(&self) -> bool {
            self.is_console_open
        }

        fn toggle(&mut self) {
            self.is_console_open = !self.is_console_open
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

    #[test]
    fn check_console_is_shown_when_toggled() {
        let mut console = EguiConsole::new();
        assert_eq!(console.is_open(), false);
        console.toggle();
        assert_eq!(console.is_open(), true)
    }
}
