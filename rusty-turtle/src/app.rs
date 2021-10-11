use eframe::{egui, epi};
use eframe::egui::{Color32, Frame, Painter, Pos2, Rgba};
use eframe::epi::egui::{emath, Rect, Ui};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct LogoApp {
    label: String,
    zoom: f32,
    line_width: f32
}

impl Default for LogoApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            zoom: 1.0,
            line_width: 2.5
        }
    }
}

impl epi::App for LogoApp {
    fn name(&self) -> &str {
        "Rusty Turtle"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().frame(Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| self.draw_turtle_canvas(ui));

        egui::TopBottomPanel::bottom("code_area").show(ctx, |bottom| {
            bottom.heading("Enter commands:");
            bottom.label(">");
            bottom.text_edit_singleline(&mut self.label);
            egui::warn_if_debug_build(bottom);
        });
    }
}

impl LogoApp {
    pub fn draw_turtle_canvas(&mut self, ui: &mut Ui) {
        ui.ctx().request_repaint();

        let painter = Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap()
        );
        self.paint(&painter);
        ui.expand_to_include_rect(painter.clip_rect());
    }

    fn paint(&mut self, painter: &Painter) {
        let rect = painter.clip_rect();
        let to_screen = emath::RectTransform::from_to(
            Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
            rect
        );

    }
}