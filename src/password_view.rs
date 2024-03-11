use magic_crypt::{MagicCrypt256, MagicCryptTrait};

use crate::PasswordData;

#[derive(Default)]
pub struct PasswordView {
    show_password: bool,
    raw_passwords: Vec<String>,
}

impl PasswordView {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        mc: &MagicCrypt256,
        password_data: &[PasswordData],
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Show or hide raw password.
            if ui.button("👁").clicked() {
                self.show_password = !self.show_password;

                if self.show_password {
                    self.raw_passwords.clear();

                    // Decrypt all password in one go
                    for pass in password_data {
                        self.raw_passwords.push(
                            mc.decrypt_base64_to_string(&pass.password)
                                .unwrap_or_default(),
                        );
                    }
                }
            }

            for (index, pass) in password_data.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(&pass.name);

                    let mut password: String;

                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(if self.show_password {
                            &mut self.raw_passwords[index]
                        } else {
                            password = pass.password.clone();
                            &mut password
                        })
                        .password(!self.show_password)
                        // Password field cannot be edited
                        .interactive(false),
                    );
                });
            }
        });
    }

    /// Call this function whenever something changes.
    pub fn hide_password(&mut self) {
        self.show_password = false;
    }
}
