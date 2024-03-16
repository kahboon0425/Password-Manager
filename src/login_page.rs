use std::fs;

use magic_crypt::{MagicCrypt256, MagicCryptTrait};

pub struct LoginPage {
    secret_key: String,
    show_secret: bool,
}

impl Default for LoginPage {
    fn default() -> Self {
        Self {
            secret_key: String::new(),
            show_secret: false,
        }
    }
}

impl LoginPage {
    pub fn show(&mut self, ctx: &egui::Context) -> Option<String> {
        // Initialize the logged in secret key to None
        // Will only be populated if the login button is clicked
        let mut logged_in_key = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            // Spacing before the login page UI
            let rect = egui::Rect::from_min_max(
                egui::Pos2::new(0.0, 0.0),
                ui.available_size().to_pos2() / 2.0 - egui::Vec2::new(30.0, 30.0),
            );

            ui.allocate_rect(rect, egui::Sense::hover());

            // Login page UI
            ui.vertical_centered(|ui| {
                ui.set_width(200.0);
                // Heading
                ui.heading("Password Manager");
                ui.add_space(10.0);

                // Secret key field
                ui.horizontal(|ui| {
                    ui.label("Secret Key: ");

                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut self.secret_key)
                            .password(!self.show_secret),
                    );

                    if ui.button("👁").clicked() {
                        self.show_secret = !self.show_secret;
                    }
                });
                ui.add_space(10.0);

                // Login button
                if ui.button("Login").clicked() {
                    // Populate the logged in secret key if the secret key field is not empty
                    if self.secret_key.is_empty() == false {
                        logged_in_key = Some(self.secret_key.to_string());
                    }
                }
            });
        });

        // Return the logged in key (move semantics)
        logged_in_key
    }

    pub fn check_magic_crypt(&mut self, magic_crypt: &MagicCrypt256) -> Option<String> {
        let mut file_found = None;

        let password_dir_path = "./password-folder";

        let target_file_name: String = magic_crypt.encrypt_str_to_base64(crate::FILENAME_ENCRYPT);

        if let Ok(files) = fs::read_dir(password_dir_path) {
            for file in files {
                if let Ok(file) = file {
                    let file_name = file.file_name();

                    println!("---------------");
                    println!("{}", target_file_name);
                    println!("{}", file_name.to_str().unwrap());
                    println!("---------------");

                    if let Some(password_file) = file_name.to_str() {
                        if target_file_name.as_str() == password_file {
                            file_found = Some(password_file.to_string());
                        }
                    }
                }
            }
        }

        file_found
    }

    pub fn read_password(&mut self, file_found: &Option<String>) -> Option<crate::PasswordDataVec> {
        println!("{:?}", file_found);

        let password_dir_path = "./password-folder";

        if let Ok(files) = fs::read_dir(password_dir_path) {
            for file in files {
                if let Ok(file_entry) = file {
                    let file_path = file_entry.path();

                    if let Some(file_name) = file_found {
                        if file_path.file_name() == Some(file_name.as_ref()) {
                            println!(">>>>>>>>>>{}", file_name);
                            if let Ok(content) = fs::read_to_string(&file_path) {
                                println!("...............{}", content);
                                if let Ok(password_data_vec) = serde_json::from_str(&content) {
                                    println!("{:?}********************", password_data_vec);

                                    // for password_data in password_data_vec.passwords {
                                    //     println!(
                                    //         "Name: {}, Password: {}",
                                    //         password_data.name, password_data.password
                                    //     );
                                    // }
                                    return Some(password_data_vec);
                                }
                            } else {
                                eprintln!("Error reading file content for {}", file_name);
                            }
                        }
                    }
                }
            }
        } else {
            println!("Error reading directory: {}", password_dir_path);
        }

        None
    }
}
