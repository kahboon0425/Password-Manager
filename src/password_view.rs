use magic_crypt::{MagicCrypt256, MagicCryptTrait};

use crate::PasswordData;

#[derive(Default)]
pub struct PasswordView {
    show_password: bool,
}

impl PasswordView {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        mc: &MagicCrypt256,
        password_data: &[PasswordData],
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("👁").clicked() {
                self.show_password = !self.show_password;
            }

            for pass in password_data {
                ui.horizontal(|ui| {
                    ui.label(&pass.name);

                    ui.set_enabled(false);
                    let mut password = pass.password.clone();
                    if self.show_password {
                        password = mc.decrypt_base64_to_string(password).unwrap_or_default();
                    }
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut password).password(!self.show_password),
                    );
                    ui.set_enabled(true);
                });
            }
        });
    }
}
