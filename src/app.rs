use metacontrols_client::ServerWidget;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        /*
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        */

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        //eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let server = "ws://127.0.0.1:5000".to_string();

        // Get the server from the URL instead!
        #[cfg(target_arch = "wasm32")]
        let server: String = web_sys::window()
            .unwrap()
            .location()
            .href()
            .unwrap()
            .split("?srv=")
            .skip(1)
            .next()
            .map(|s| s.to_string())
            .unwrap_or(server);


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&server);
            ui.add(ServerWidget::new(&server));
        });
    }
}
