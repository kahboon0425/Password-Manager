#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use add_password_form::AddPasswordForm;
use eframe::egui;
use login_page::LoginPage;
use magic_crypt::{new_magic_crypt, MagicCrypt256};
use serde::{Deserialize, Serialize};
use top_panel::TopPanel;

mod add_password_form;
mod login_page;
mod password_view;
mod top_panel;

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

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
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

    password_data: Option<PasswordDataVec>,
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Match to see if the magic_crypt Option enum is Some or None
        match &mut self.magic_crypt {
            // The applicatio is logged in
            Some(mc) => {
                // let check_mc: Option<String> = AddPasswordForm::check_magic_crypt(mc);

                let check_mc: Option<String> = LoginPage::check_magic_crypt(mc);
                // let read_password: Option<PasswordDataVec> = LoginPage::read_password(&check_mc);

                self.password_data = LoginPage::read_password(&check_mc);

                // if let Some(check_mc) = &check_mc {
                //     self.password_data = LoginPage::read_password(&check_mc);
                // }
                println!("{:?}", self.password_data);

                // Show top panel
                if self
                    .top_panel
                    .show(ctx, mc, check_mc, &mut self.password_data)
                {
                    // Set magic_crypt to None when user logs out
                    self.magic_crypt = None;
                }
            }
            // The application is not logged in
            None => {
                // Show login page
                if let Some(secret_key) = self.login_page.show(ctx) {
                    // Create magic_crypt when a secret key is being created from the login page
                    self.magic_crypt = Some(new_magic_crypt!(&secret_key, 256));
                }
            }
        }
    }
}
