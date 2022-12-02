use dmxt_ui::windows::main_window;
use eframe::{self, egui};

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.maximized = true;
    let app = MainApp::new();
    eframe::run_native(
        "DMXT v0.0.0",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}

struct MainApp {
    main_window: main_window::MainWindow,
}

impl MainApp {
    pub fn new() -> Self {
        Self {
            main_window: main_window::MainWindow::new(),
        }
    }
}
impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.main_window.ui(ctx);
    }
}