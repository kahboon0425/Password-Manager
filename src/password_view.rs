use std::{fs::OpenOptions, io::Write};

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
        password_data: &mut Vec<PasswordData>,
        password_file: &Option<String>,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Show or hide raw password.
            if ui.button("👁").clicked() {
                self.show_password = !self.show_password;

                if self.show_password {
                    self.raw_passwords.clear();

                    // Decrypt all password in one go
                    for pass in password_data.iter_mut() {
                        self.raw_passwords.push(
                            mc.decrypt_base64_to_string(&pass.password)
                                .unwrap_or_default(),
                        );
                    }
                }
            }

            let mut indices_to_remove = Vec::new();

            for (index, pass) in password_data.iter_mut().enumerate() {
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
                        // password_data.remove(index);
                    }
                });
            }
            for index in indices_to_remove {
                println!("Old Password Data>>>>>>>>> {:?}: ", password_data);
                password_data.remove(index);
                println!("New Password Data<<<<<<<<<< {:?}: ", password_data);
            }

            // self.update_file(password_file, password_data);
        });
    }

    pub fn update_file(&mut self, password_file: &Option<String>, password_data: &[PasswordData]) {
        // Serialize the password data vector to JSON
        let json_data = serde_json::to_string(&password_data).unwrap();

        let password_dir_path = "./password-folder";

        if let Some(password_file) = password_file {
            let file_path = format!("{}/{}", password_dir_path, password_file);

            // Open the file
            let mut save_password = OpenOptions::new()
                .write(true)
                .create(true) // create new file or open it if already exists
                .open(file_path)
                .unwrap();

            // Write JSON data to the file
            save_password.write_all(json_data.as_bytes()).unwrap();
        }
    }

    pub fn refresh_raw_password(&mut self, mc: &MagicCrypt256, password_data: &[PasswordData]) {
        self.raw_passwords.clear();
        for pass in password_data {
            self.raw_passwords.push(
                mc.decrypt_base64_to_string(&pass.password)
                    .unwrap_or_default(),
            );
        }
    }

    /// Call this function whenever something changes.
    pub fn hide_password(&mut self) {
        self.show_password = false;
    }
}
