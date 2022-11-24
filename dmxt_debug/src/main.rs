#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, ScrollArea};

use dmxt_lib::dmx_serial::{self, DMXSerial, DMXError};


fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "DXM-Debugger",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    channels: Vec<ChannelComponent>,
    interface_path: String,
    dmx: Option<DMXSerial>,
    connection_error: bool,
    status: egui::RichText,
    temp: u8
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            channels: vec![ChannelComponent::new(1, 0).unwrap()],
            interface_path: String::new(),
            dmx: Option::None,
            connection_error: false,
            status: egui::RichText::new(""),
            temp: 0
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DMX-Debugger 📊");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Interface:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.interface_path)
                    .hint_text("COM")
                    .desired_width(50.0)
                    .interactive(self.dmx.is_none())
                );
                if self.dmx.is_none() {
                    if ui.button("Connect").clicked() {
                        match DMXSerial::open(&self.interface_path) {
                            Ok(mut dmx) => {
                                self.connection_error = false;
                                dmx.set_max_channels(512).unwrap();
                                self.dmx = Option::Some(dmx);
                                self.status = egui::RichText::new("Connected!").color(egui::Color32::GREEN);
                            }
                            Err(e) => {
                                self.connection_error = true;
                                self.status = egui::RichText::new(e.to_string()).color(egui::Color32::RED);
                                println!("Error: {}", e);
                            }
                        }
                    }
                } else {
                    if ui.button("Disconnect").clicked() {
                        self.dmx = Option::None;
                        self.status = egui::RichText::new("");
                    }
                }
                    ui.add(egui::Label::new(self.status.clone()).wrap(true));
            });
            if self.dmx.is_none() { return; }
            ui.separator();
            ui.add_space(10.0);
            ui.style_mut().spacing.slider_width = 350.0;
            egui::ScrollArea::horizontal()
                .always_show_scroll(true)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(&mut self.temp, 0..=255).text(format!("")).vertical());
                    });
                });
            });
    }
}

struct ChannelComponent {
    channel: usize,
    value: u8,
}

impl ChannelComponent {
    fn new(channel:usize, value: u8) -> Result<Self, DMXError> {
        match dmx_serial::check_valid_channel(channel) {
            Ok(_) => {
                Ok(Self {
                    channel,
                    value
                })
            },
            Err(e) => Err(e),
        }
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add(egui::Label::new(format!("Channel {}:", self.channel)));
            ui.add(egui::Slider::new(&mut self.value, 0..=255).text(format!("")).vertical());
        });
    }
}