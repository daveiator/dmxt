pub mod main_window;


use eframe::egui::Context;

trait SubWindow {
    fn ui(&mut self, ctx: &Context);
}