pub struct LoginPage {
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
    pub fn show(&mut self, ctx: &egui::Context) -> Option<String> {
        // Initialize the logged in secret key to None
        // Will only be populated if the login button is clicked
        let mut logged_in_key = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            // Spacing before the login page UI
            let rect = egui::Rect::from_min_max(
                egui::Pos2::new(0.0, 0.0),
                ui.available_size().to_pos2() / 2.0 - egui::Vec2::new(30.0, 30.0),
            );

            ui.allocate_rect(rect, egui::Sense::hover());

            // Login page UI
            ui.vertical_centered(|ui| {
                ui.set_width(200.0);
                // Heading
                ui.heading("Password Manager");
                ui.add_space(10.0);

                // Secret key field
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

                // Login button
                if ui.button("Login").clicked() {
                    // Populate the logged in secret key if the secret key field is not empty
                    if self.secret_key.is_empty() == false {
                        logged_in_key = Some(self.secret_key.to_string());
                    }
                }
            });
        });

        // Return the logged in key (move semantics)
        logged_in_key
    }
}
