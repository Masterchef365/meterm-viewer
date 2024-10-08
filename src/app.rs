use meterm_client::ServerWidget;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    server: String,
    debug_mode: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            server: "ws://127.0.0.1:5000".to_string(),
            debug_mode: false,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>, server: String) -> Self {
        /*
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        */

        Self {
            server,
            ..Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        //eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.server);
                ui.checkbox(&mut self.debug_mode, "Debug mode")
                    .on_hover_text("Show the updates to the server only. Flashing lights warning!");
            });
            ui.add(ServerWidget::new(&self.server).debug_packets(self.debug_mode));
        });
    }
}
