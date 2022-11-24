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
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            channels: vec![ChannelComponent::new(1, 0).unwrap(); 1],
            interface_path: String::new(),
            dmx: Option::None,
            connection_error: false,
            status: egui::RichText::new(""),
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
                            Ok(dmx) => {
                                self.connection_error = false;
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
                        println!("Disconnecting...");
                        self.dmx = Option::None;
                        self.status = egui::RichText::new("");
                    }
                }
                    ui.add(egui::Label::new(self.status.clone()).wrap(true));
            });
            if self.dmx.is_none() {
                return;
            }
            ui.separator();
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add(egui::Button::new("New Channel"));
                if ui.add(egui::Button::new("Reset")).clicked() {
                    self.channels = vec![ChannelComponent::new(1, 0).unwrap(); 1];
                    self.dmx.as_mut().unwrap().reset_channels();
                }
            });
            ui.group(|ui| {
                ui.style_mut().spacing.slider_width = ui.available_height() - 100.0;
                egui::ScrollArea::horizontal().always_show_scroll(true).auto_shrink([false, true]).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for channel in self.channels.iter_mut() {     
                            channel.update(ui);
                            match dmx_serial::check_valid_channel(channel.channel) {
                                Ok(_) => {
                                    self.dmx.as_mut().unwrap().set_channel(channel.channel, channel.value).unwrap();
                                },
                                Err(e) => {
                                    println!("Error: {}", e);
                                    channel.channel = 1;
                                }
                            } 
                        }
                    });
                });
            });
        });
    }
}


#[derive(Debug, Clone)]
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

    fn update(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_max_width(25.0);
            ui.vertical_centered_justified(|ui| {
                ui.add(egui::Slider::new(&mut self.value, 0..=255).vertical().show_value(false));
                ui.add(
                    egui::DragValue::new(&mut self.value)
                    .fixed_decimals(0)
                ).on_hover_text("Value");
                ui.add(
                    egui::DragValue::new(&mut self.channel)
                    .fixed_decimals(0)
                ).on_hover_text("Channel");
                ui.separator();
                ui.add_space(10.0);
                ui.add(egui::Button::new("X")).on_hover_text("Delete Channel");
            });
        });
    }
}