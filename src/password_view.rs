use magic_crypt::MagicCrypt256;

use crate::add_password_form::PasswordData;

pub fn show(ctx: &egui::Context, mc: &MagicCrypt256, password_data: &[PasswordData]) {
    egui::CentralPanel::default().show(ctx, |ui| {
        for pass in password_data {
            ui.label(&pass.name);
        }
    });
}
