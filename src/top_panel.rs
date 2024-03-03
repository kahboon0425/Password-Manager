use crate::add_password_form::AddPasswordForm;

#[derive(Default)]
pub struct TopPanel {
    pub add_password_form: AddPasswordForm,
    pub add_password_form_open: bool,
}

impl TopPanel {
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Add").clicked() {
                    self.add_password_form_open = true;
                };
            });
        });

        self.add_password_form
            .show_window(ctx, &mut self.add_password_form_open);
    }
}
