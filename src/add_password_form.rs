use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use serde::{Deserialize, Serialize};

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

        let encrypted_password = mc.encrypt_str_to_base64(&mut self.password);
        println!("{}", encrypted_password);
        println!(
            "{}",
            mc.decrypt_base64_to_string(&encrypted_password).unwrap()
        );
        if ui.button("Add").clicked() {}
    }
}
