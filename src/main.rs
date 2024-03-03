#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use login_page::LoginPage;
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use top_panel::TopPanel;

mod add_password_form;
mod login_page;
mod password_view;
mod top_panel;

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(MyApp::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {
    magic_crypt: Option<MagicCrypt256>,
    login_page: LoginPage,
    top_panel: TopPanel,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &mut self.magic_crypt {
            Some(mc) => {
                self.top_panel.show(ctx);
            }
            None => {
                if let Some(secret_key) = self.login_page.show(ctx) {
                    self.magic_crypt = Some(new_magic_crypt!(secret_key, 256));
                }
            }
        }
    }
}
