use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use eframe::Error;
use egui::TextBuffer;
use magic_crypt::{MagicCrypt256, MagicCryptTrait};

use crate::{PasswordData, PasswordDataVec};
// use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct AddPasswordForm {
    name: String,
    password: String,
    show_password: bool,
}

impl AddPasswordForm {
    pub fn show_window(
        &mut self,
        ctx: &egui::Context,
        open: &mut bool,
        mc: &MagicCrypt256,
        check_mc: Option<String>,
        existing_password_data: &mut Option<PasswordDataVec>,
    ) {
        egui::Window::new("Add Password")
            .max_size(egui::Vec2::new(200.0, 100.0))
            .vscroll(false)
            .resizable(false)
            .collapsible(false)
            .open(open)
            .show(ctx, |ui| self.ui(ui, mc, check_mc, existing_password_data));
    }

    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        mc: &MagicCrypt256,
        check_magic_crypt: Option<String>,
        existing_password_data: &mut Option<PasswordDataVec>,
    ) {
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

        if ui.button("Add").clicked() {
            let password_data = crate::PasswordData {
                name: self.name.clone(),
                password: encrypted_password.clone(),
            };
            self.write_password(password_data, mc, check_magic_crypt, existing_password_data)
        }
    }

    pub fn write_password(
        &mut self,
        password_data: PasswordData,
        mc: &MagicCrypt256,
        check_magic_crypt: Option<String>,
        existing_password_data: &mut Option<PasswordDataVec>,
    ) {
        if let Some(check_magic_crypt) = check_magic_crypt {
            println!("Targeted file {:?}", check_magic_crypt);

            if let Some(data) = existing_password_data {
                data.passwords.push(password_data);
            } else {
                let mut password_data_vec = crate::PasswordDataVec {
                    passwords: Vec::new(),
                };

                password_data_vec.passwords.push(password_data);
            }
        } else {
            let mut password_data_vec = crate::PasswordDataVec {
                passwords: Vec::new(),
            };

            password_data_vec.passwords.push(password_data);
            *existing_password_data = Some(password_data_vec);
        }

        // Serialize the password data vector to JSON
        let json_data = serde_json::to_string(&existing_password_data).unwrap();

        println!("Json Data: {:?}", json_data);

        let file_name: String = mc.encrypt_str_to_base64(crate::FILENAME_ENCRYPT);

        println!("File Name: {}", file_name);

        let password_dir_path = "./password-folder";

        let file_path = format!("{}/{}", password_dir_path, file_name);

        // Open the file
        let mut save_password = OpenOptions::new()
            .write(true)
            .create(true) // create new file or open it if already exists
            .open(file_path)
            .unwrap();

        // Write JSON data to the file
        save_password.write_all(json_data.as_bytes()).unwrap();

        self.name.clear();
        self.password.clear();
    }
}
