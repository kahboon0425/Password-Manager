#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{fs, io::Write};

use eframe::egui;
use login_page::LoginPage;
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use serde::{Deserialize, Serialize};
use top_panel::TopPanel;

mod add_password_form;
mod login_page;
mod password_view;
mod top_panel;

const FILENAME_ENCRYPT: &str = "filename";
const PASSWORD_DIR: &str = "./password_data/";

/// A vector of password data entries.
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct PasswordDataVec {
    pub passwords: Vec<PasswordData>,
}

/// A single entry of password data.
#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordData {
    pub name: String,
    pub password: String,
}

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    fs::create_dir_all(PASSWORD_DIR).unwrap();

    eframe::run_native(
        "Password Manager",
        options,
        Box::new(|_cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(MainApp::default())
        }),
    )
}

/// The main app that drives the update loop of the password manager.
#[derive(Default)]
struct MainApp {
    /// An Option enum that holds a MagicCrypt256 struct.
    /// It will be None when the application is not logged in, and Some when the application is logged in.
    magic_crypt: Option<MagicCrypt256>,
    /// The login page UI of the application.
    login_page: LoginPage,
    /// The top panel UI when the user logged into the application.
    top_panel: TopPanel,
    password_data: PasswordDataVec,
    filename: String,
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Match to see if the magic_crypt Option enum is Some or None
        match &mut self.magic_crypt {
            // The applicatio is logged in
            Some(mc) => {
                // Show top panel
                if self
                    .top_panel
                    .show(ctx, mc, &self.filename, &mut self.password_data)
                {
                    // Set magic_crypt to None when user logs out
                    self.magic_crypt = None;
                }
            }
            // The application is not logged in
            None => {
                // Show login page
                if let Some(secret_key) = self.login_page.show(ctx) {
                    let mc = new_magic_crypt!(&secret_key, 256);
                    // Evaluate file name using magic crypt
                    self.filename = format!(
                        "{}{}",
                        PASSWORD_DIR,
                        mc.encrypt_str_to_base64(crate::FILENAME_ENCRYPT)
                    );

                    // Create magic_crypt when a secret key is being created from the login page
                    self.magic_crypt = Some(mc);
                    // Read password data or create new one using magic crypt data
                    self.password_data = read_password_data(&self.filename);
                }
            }
        }
    }
}

pub fn read_password_data(filename: &String) -> PasswordDataVec {
    let content = fs::read_to_string(filename);

    if let Ok(content) = content {
        // Deserialize file into password data vector
        serde_json::from_str::<PasswordDataVec>(&content).unwrap()
    } else {
        // If no file is found, return an empty password data vector
        PasswordDataVec::default()
    }
}

pub fn write_to_file(filename: &String, password_data_vec: &PasswordDataVec) {
    // Serialize the password data vector to JSON
    let json_data = serde_json::to_string(&password_data_vec).unwrap();

    // Open the file
    let mut save_password = fs::OpenOptions::new()
        .write(true)
        .create(true) // create new file or open it if already exists
        .open(filename)
        .unwrap();

    // Write JSON data to the file
    save_password.write_all(json_data.as_bytes()).unwrap();
}
