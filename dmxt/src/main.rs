use eframe::{self, egui, App};
use dmxt_ui::pages::*;


#[derive(Debug, Default)]
struct  DMXTApp {
    open_page: Page,
    file: String,
    // universes: Vec<Universe>,
    // interfaces: Vec<Interface>,
    
    // fixtures: Vec<Fixture>,
    // scene_groups: Vec<SceneGroup>,

    // mixer: Mixer,
}

impl App for DMXTApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            egui::trace!(ui); 
            egui::menu::bar(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.menu_button("⚡", |ui| {
                            ui.button("About");
                            ui.button("❤️ Donate");
                            ui.separator();
                            if ui.button("Quit").clicked() {
                                frame.close();
                            }
                        });
                        ui.menu_button("File", |ui| {
                            ui.button("New");
                            ui.button("Open...");
                            ui.menu_button("Open recent", |ui| {
                                ui.button("File 1");
                                ui.button("File 2");
                                ui.button("File 3");
                            });
                            ui.button("Import Patch...");
                            ui.button("Import Scenes...");
                            ui.separator();
                            ui.button("Check for missing files");
                            ui.separator();
                            ui.button("Save");
                            ui.button("Save As...");
                        });
                        ui.menu_button("Edit", |ui| {
                            ui.button("Undo");
                            ui.button("Redo");
                            ui.separator();
                            ui.button("Preferences");
                        });
                        ui.menu_button("Tools", |ui| {
                            ui.button("Interface Manager");
                            ui.button("DMX Monitor");
                        });
                        ui.menu_button("Mappings" , |ui| {
                            ui.button("Map Keyboard");
                            ui.button("Map OSC");
                        });
                        ui.menu_button("Help", |ui| {
                            ui.button("Help");
                            ui.button("About");
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