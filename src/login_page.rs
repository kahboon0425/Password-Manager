struct LoginPage {
    secret_key: String,
    show_secret: bool,
}

impl Default for LoginPage {
    fn default() -> Self {
        Self {
            secret_key: String::new(),
            show_secret: false,
        }
    }
}

impl LoginPage {
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = egui::Rect::from_min_max(
                egui::Pos2::new(0.0, 0.0),
                ui.available_size().to_pos2() / 2.0 - egui::Vec2::new(30.0, 30.0),
            );

            ui.allocate_rect(rect, egui::Sense::hover());

            ui.vertical_centered(|ui| {
                ui.set_width(200.0);
                ui.heading("Password Manager");
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.label("Secret Key: ");

                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut self.secret_key)
                            .password(!self.show_secret),
                    );

                    if ui.button("👁").clicked() {
                        self.show_secret = !self.show_secret;
                    }
                });
                ui.add_space(10.0);
                if ui.button("Login").clicked() {}
            });

            // println!("Secret Key: {}", self.secret_key);
        });
    }
}
