use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct PasswordDataVec {
    pub passwords: Vec<PasswordData>,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordData {
    pub name: String,
    pub password: String,
}

#[derive(Default)]
pub struct AddPasswordForm {
    pub name: String,
    pub password: String,
    pub show_password: bool,
}

impl AddPasswordForm {
    pub fn show_window(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Add Password")
            .max_size(egui::Vec2::new(200.0, 100.0))
            .vscroll(false)
            .resizable(false)
            .collapsible(false)
            .open(open)
            .show(ctx, |ui| self.ui(ui));
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.name);
        });

        ui.horizontal(|ui| {
            ui.label("Password:");
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::singleline(&mut self.password).password(!self.show_password),
            );

            if ui.button("👁").clicked() {
                self.show_password = !self.show_password;
            }
        });

        if ui.button("Add").clicked() {}
    }
}
