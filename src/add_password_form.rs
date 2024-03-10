use std::{
    fs::{self, File, OpenOptions},
    io::Write,
};

use eframe::Error;
use egui::TextBuffer;
use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use serde::{Deserialize, Serialize};

const FILENAME_ENCRYPT: &str = "filename";

/// A vector of password data entries.
#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordDataVec {
    pub passwords: Vec<PasswordData>,
}

/// A single entry of password data.
#[derive(Serialize, Deserialize, Debug)]
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
    pub fn show_window(
        &mut self,
        ctx: &egui::Context,
        open: &mut bool,
        mc: &MagicCrypt256,
        check_mc: Option<String>,
    ) {
        egui::Window::new("Add Password")
            .max_size(egui::Vec2::new(200.0, 100.0))
            .vscroll(false)
            .resizable(false)
            .collapsible(false)
            .open(open)
            .show(ctx, |ui| self.ui(ui, mc, check_mc));
    }

    fn ui(&mut self, ui: &mut egui::Ui, mc: &MagicCrypt256, check_magic_crypt: Option<String>) {
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
        if ui.button("Add").clicked() {
            let password_data = PasswordData {
                name: self.name.clone(),
                password: encrypted_password.clone(),
            };

            println!("Targeted file {:?}", check_magic_crypt);
            if let Some(check_magic_crypt) = check_magic_crypt {
                println!("Append Here");
                println!("Targeted file {:?}", check_magic_crypt);
                let password_dir_path = "./password-folder";

                let file_path = format!("{}/{}", password_dir_path, check_magic_crypt);

                let new_password_data = serde_json::to_string(&password_data).unwrap();

                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(file_path)
                    .unwrap();

                writeln!(file, "{}", new_password_data).unwrap();
            } else {
                let mut password_data_vec = PasswordDataVec {
                    passwords: Vec::new(),
                };

                println!("Password Data: {:?}", password_data);

                password_data_vec.passwords.push(password_data);

                println!("Password Data List: {:?}", password_data_vec.passwords);

                println!("Password Data Vec: {:?}", password_data_vec);

                let json_data = serde_json::to_string(&password_data_vec).unwrap();

                println!("Json Data:{:?}", json_data);

                let file_name: String = mc.encrypt_str_to_base64(FILENAME_ENCRYPT);
                // let magic_crypt_str = format!("{:?}", mc);
                // println!("{}", magic_crypt_str);

                // let remove_non_digit_character = magic_crypt_str
                //     .chars()
                //     .filter(|c| c.is_ascii_digit())
                //     .collect::<String>();

                // let file_name = format!("{}.json", remove_non_digit_character);

                println!("File Name{}", file_name);

                let password_dir_path = "./password-folder";

                let file_path = format!("{}/{}", password_dir_path, file_name);

                let mut save_password = OpenOptions::new()
                    .write(true)
                    .create(true) // create new file or open it if already exists
                    .open(file_path)
                    .unwrap();

                save_password.write_all(json_data.as_bytes()).unwrap();
                self.name.clear();
                self.password.clear();
            }
        }
    }

    pub fn check_magic_crypt(magic_crypt: &MagicCrypt256) -> Option<String> {
        let mut file_found = None;

        let password_dir_path = "./password-folder";

        let target_file_name: String = magic_crypt.encrypt_str_to_base64(FILENAME_ENCRYPT);

        if let Ok(files) = fs::read_dir(password_dir_path) {
            for file in files {
                if let Ok(file) = file {
                    let file_name = file.file_name();

                    println!("---------------");
                    println!("{}", target_file_name);
                    println!("{}", file_name.to_str().unwrap());
                    println!("---------------");

                    // println!("Get password file name{:?}: ", file_name);
                    if let Some(password_file) = file_name.to_str() {
                        // println!("Get password string file name{:?}: ", password_file);
                        if target_file_name.as_str() == password_file {
                            // println!("MMMMMMMMMagic Crypt: {:?}", password_file);
                            // println!("FFFFFFFFFile Name: {:?}", target_file_name);
                            file_found = Some(password_file.to_string());
                        }
                    }
                }
            }
        }
        // let content = match fs::read_to_string(file_path) {
        //     Ok(content) => content,
        //     Err(_) => return false,
        // };
        // println!("{}", content);

        // let password_data_vec: PasswordDataVec =
        //     match serde_json::from_str::<PasswordDataVec>(&content) {
        //         Ok(password_data_vec) => password_data_vec,
        //         Err(_) => return false,
        //     };

        // for password_data in password_data_vec.passwords {
        //     println!(
        //         "Name: {}, Password: {}",
        //         password_data.name, password_data.password
        //     );
        // }

        file_found
    }
}
