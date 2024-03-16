use crate::{add_password_form::AddPasswordForm, password_view::PasswordView, PasswordDataVec};
use magic_crypt::MagicCrypt256;

#[derive(Default)]
pub struct TopPanel {
    /// Password list view
    password_view: PasswordView,
    /// Form for adding password.
    add_password_form: AddPasswordForm,
    /// Boolean to show or hide AddPasswordForm.
    add_password_form_open: bool,
}

impl TopPanel {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        mc: &MagicCrypt256,
        filename: &String,
        password_data_vec: &mut PasswordDataVec,
    ) -> bool {
        let mut logout = false;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Add password button
                if ui.button("Add").clicked() {
                    // Open password form
                    self.add_password_form_open = true;
                };

                // Logout button
                if ui.button("Logout").clicked() {
                    self.password_view.hide_password();
                    logout = true;
                };
            });
        });

        let password_data =
            self.add_password_form
                .show_window(ctx, &mut self.add_password_form_open, mc);

        // Add password data if user add a new one from the form above.
        if let Some(password_data) = password_data {
            // Update password on memory
            password_data_vec.passwords.push(password_data);
            self.password_view
                .refresh_raw_password(&mc, &password_data_vec.passwords);

            // Update file on disk
            crate::write_to_file(filename, &password_data_vec);
        }

        self.password_view
            .show(ctx, mc, password_data_vec, &filename);

        logout
    }
}
