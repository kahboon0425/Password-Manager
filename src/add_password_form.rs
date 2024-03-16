use magic_crypt::{MagicCrypt256, MagicCryptTrait};

use crate::PasswordData;

#[derive(Default)]
pub struct AddPasswordForm {
    name: String,
    password: String,
    show_password: bool,
    // password_view: PasswordView,
}

impl AddPasswordForm {
    pub fn show_window(
        &mut self,
        ctx: &egui::Context,
        open: &mut bool,
        mc: &MagicCrypt256,
    ) -> Option<PasswordData> {
        let mut password_data = None;

        egui::Window::new("Add Password")
            .max_size(egui::Vec2::new(200.0, 100.0))
            .vscroll(false)
            .resizable(false)
            .collapsible(false)
            .open(open)
            .show(ctx, |ui| password_data = self.ui(ctx, ui, mc));

        password_data
    }

    fn ui(
        &mut self,
        _ctx: &egui::Context,
        ui: &mut egui::Ui,
        mc: &MagicCrypt256,
    ) -> Option<PasswordData> {
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

        if ui.button("Add").clicked() {
            if self.name.is_empty() == false && encrypted_password.is_empty() == false {
                return Some(PasswordData {
                    name: self.name.clone(),
                    password: encrypted_password.clone(),
                });
            }
        }

        None
    }
}
