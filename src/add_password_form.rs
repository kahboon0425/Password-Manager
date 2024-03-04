use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use serde::{Deserialize, Serialize};

/// A vector of password data entries.
#[derive(Serialize, Deserialize)]
pub struct PasswordDataVec {
    pub passwords: Vec<PasswordData>,
}

/// A single entry of password data.
#[derive(Serialize, Deserialize)]
pub struct PasswordData {
    pub name: String,
    pub password: String,
}

#[derive(Default)]
pub struct AddPasswordForm {
    name: String,
    password: String,
    show_password: bool,
}

impl AddPasswordForm {
    pub fn show_window(&mut self, ctx: &egui::Context, open: &mut bool, mc: &MagicCrypt256) {
        egui::Window::new("Add Password")
            .max_size(egui::Vec2::new(200.0, 100.0))
            .vscroll(false)
            .resizable(false)
            .collapsible(false)
            .open(open)
            .show(ctx, |ui| self.ui(ui, mc));
    }

    fn ui(&mut self, ui: &mut egui::Ui, mc: &MagicCrypt256) {
        // Name field
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.name);
        });

        // Password field
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

        let encrypted_password = mc.encrypt_str_to_base64(&mut self.password);
        println!("{}", encrypted_password);
        println!(
            "{}",
            mc.decrypt_base64_to_string(&encrypted_password).unwrap()
        );

        // Save encrypted password to disk.
        if ui.button("Add").clicked() {}
    }
}
