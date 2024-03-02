use crate::View;

#[derive(Default)]
pub struct AddPasswordForm {
    pub name: String,
    pub password: String,
    pub show_password: bool,
}

impl AddPasswordForm {
    pub fn show_window(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Test")
            .vscroll(false)
            .resizable(false)
            .open(open)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl crate::View for AddPasswordForm {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.name);
        });

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

        if ui.button("Add").clicked() {}
    }
}