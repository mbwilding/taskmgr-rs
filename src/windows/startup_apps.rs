use crate::TaskManagerApp;
use egui::*;

pub fn show(_app: &mut TaskManagerApp, ui: &mut Ui) {
    ui.vertical_centered_justified(|ui| {
        ui.heading("Startup apps");
        ui.label("Not Implemented");
    });
}
