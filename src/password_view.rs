use magic_crypt::{MagicCrypt256, MagicCryptTrait};

use crate::{PasswordData, PasswordDataVec};

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
        password_data_vec: &mut PasswordDataVec,
        filename: &String,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Show or hide raw password.
            if ui.button("👁").clicked() {
                self.show_password = !self.show_password;

                if self.show_password {
                    self.refresh_raw_password(&mc, &password_data_vec.passwords)
                }
            }

            let mut indices_to_remove = Vec::new();

            for (index, pass) in password_data_vec.passwords.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(&pass.name);

                    let mut password: String;

                    ui.add(
                        egui::TextEdit::singleline(if self.show_password {
                            &mut self.raw_passwords[index]
                        } else {
                            password = pass.password.clone();
                            &mut password
                        })
                        .password(!self.show_password),
                    );
                    if ui.button("delete").clicked() {
                        indices_to_remove.push(index);
                    }
                });
            }

            if indices_to_remove.len() > 0 {
                // Remove from larger indices to smaller ones.
                indices_to_remove.reverse();

                for index in indices_to_remove {
                    password_data_vec.passwords.remove(index);
                }

                crate::write_to_file(filename, &password_data_vec);
                self.refresh_raw_password(mc, &password_data_vec.passwords);
            }
        });
    }

    pub fn refresh_raw_password(&mut self, mc: &MagicCrypt256, passwords: &[PasswordData]) {
        self.raw_passwords.clear();

        // Decrypt all password in one go
        for pass in passwords {
            self.raw_passwords.push(
                mc.decrypt_base64_to_string(&pass.password)
                    .unwrap_or_default(),
            );
        }
    }

    pub fn hide_password(&mut self) {
        self.show_password = false;
    }
}
