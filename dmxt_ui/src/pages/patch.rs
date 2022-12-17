use eframe::egui::Ui;
use crate::pages::PageUI;

pub struct PatchPage {

}

impl Default for PatchPage {
    fn default() -> Self {
        Self {}
    }
}

impl PageUI for PatchPage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.label("Patch Page");
    }
}