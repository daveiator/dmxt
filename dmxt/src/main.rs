use eframe::{self, egui, App};
use dmxt_ui::pages::*;
use dmxt_ui::windows::about_window::about_window;


#[derive(Debug, Default)]
struct  DMXTApp {
    open_page: Page,
    // file: String,
    about_window: bool,
    // universes: Vec<Universe>,
    // interfaces: Vec<Interface>,
    
    // fixtures: Vec<Fixture>,
    // scene_groups: Vec<SceneGroup>,

    // mixer: Mixer,
}

impl App for DMXTApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.about_window {
            if !about_window(ctx).hovered() && ctx.input().pointer.any_pressed() {
                self.about_window = false;
            };
        }

        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            egui::trace!(ui); 
            egui::menu::bar(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.menu_button("⚡", |ui| {
                            if ui.button("About").clicked() {
                                self.about_window = true;
                            };
                            if ui.button("❤️ Donate").clicked() {
                                let _ = open::that("https://github.com/sponsors/daveiator");
                            };
                            ui.separator();
                            if ui.button("Quit").clicked() {
                                frame.close();
                            }
                        });
                        ui.menu_button("File", |ui| {
                            let _ = ui.button("New");
                            let _ = ui.button("Open...");
                            let _ = ui.menu_button("Open recent", |ui| {
                                let _ = ui.button("File 1");
                                let _ = ui.button("File 2");
                                let _ = ui.button("File 3");
                            });
                            let _ = ui.button("Import Patch...");
                            let _ = ui.button("Import Scenes...");
                            let _ = ui.separator();
                            let _ = ui.button("Check for missing files");
                            let _ = ui.separator();
                            let _ = ui.button("Save");
                            let _ = ui.button("Save As...");
                        });
                        ui.menu_button("Edit", |ui| {
                            let _ = ui.button("Undo");
                            let _ = ui.button("Redo");
                            let _ = ui.separator();
                            let _ = ui.button("Preferences");
                        });
                        ui.menu_button("Tools", |ui| {
                            let _ = ui.button("Interface Manager");
                            let _ = ui.button("DMX Monitor");
                        });
                        ui.menu_button("Mappings" , |ui| {
                            let _ = ui.button("Map Keyboard");
                            let _ = ui.button("Map OSC");
                        });
                        ui.menu_button("Help", |ui| {
                            let _ = ui.button("Help");
                            let _ = ui.button("About");
                        });
                    });
                    ui.add_space(20.0);
                });

                ui.add_space(40.0);

                ui.style_mut().spacing.button_padding = egui::vec2(10.0, 10.0);

                ui.selectable_value(&mut self.open_page, Page::Patch, "Patch");
                ui.selectable_value(&mut self.open_page, Page::Scenes, "Scenes");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.open_page {
                Page::Patch => {
                    PatchPage::default().ui(ui);
                }
                Page::Scenes => {
                    ScenePage::default().ui(ui);
                }
            }
        });
    }
}

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.maximized = true;
    let app = DMXTApp::default();
    eframe::run_native(
        "DMXT",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}