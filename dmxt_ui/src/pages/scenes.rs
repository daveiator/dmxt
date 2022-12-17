use eframe::egui::Ui;
use crate::pages::PageUI;

pub struct ScenePage {

}

impl Default for ScenePage {
    fn default() -> Self {
        Self {}
    }
}

impl PageUI for ScenePage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.label("Scene Page");
    }
}