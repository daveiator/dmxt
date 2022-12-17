mod patch;
pub use patch::PatchPage;
mod scenes;
pub use scenes::ScenePage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    Patch,
    Scenes,
}

impl Default for Page {
    fn default() -> Self {
        Self::Patch
    }
}

use eframe::egui::Ui;
pub trait PageUI {
    fn ui(&mut self, ui: &mut Ui);
}