/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TerminalApp {
    // Example stuff:
    terminalBuffer: String,
    screenContent: String
}

impl Default for TerminalApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            terminalBuffer: "".to_owned(),
            screenContent: "".to_owned()
        }
    }
}

impl TerminalApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TerminalApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::SidePanel::left("left panel").show(ctx, |ui| {

        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {


            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {

                egui::widgets::global_theme_preference_switch(ui);

                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(160.0);
                }
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {

            let term_text_edit = ui.text_edit_singleline(&mut self.terminalBuffer);
            let term_text_id = term_text_edit.id;
            
            if term_text_edit.lost_focus() {
                //usage of and clearing of term content
                self.screenContent = self.terminalBuffer.clone();
                self.terminalBuffer.clear();

                //returning focus
                ui.ctx().memory_mut(|mem| mem.request_focus(term_text_id));
            }
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.label(self.screenContent.clone());
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
