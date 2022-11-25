#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

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
    is_free: [bool; 512],
    interface_path: String,
    dmx: Option<DMXSerial>,
    connection_error: bool,
    status: egui::RichText,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut is_free = [true; 512];
        Self {
            channels: vec![ChannelComponent::create_next(&mut is_free, 0).unwrap(); 1],
            is_free,
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
                if ui.add(egui::Button::new("New Channel")).clicked() {
                    if let Ok(channel) = ChannelComponent::create_next(&mut self.is_free, 0) {
                        self.channels.push(channel);
                    } else {
                        println!("No more channels left!");
                    }
                }
                if ui.add(egui::Button::new("Reset")).clicked() {
                    self.is_free = [true; 512];
                    self.channels = vec![ChannelComponent::new(&mut self.is_free, 1, 0).unwrap(); 1];
                    self.dmx.as_mut().unwrap().reset_channels();
                }
            });
            ui.group(|ui| {
                ui.style_mut().spacing.slider_width = ui.available_height() - 100.0;
                egui::ScrollArea::horizontal().always_show_scroll(true).auto_shrink([false, true]).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let free_channels = &mut self.is_free;
                        for channel in self.channels.iter_mut() {    
                            channel.update(free_channels, ui);
                            match dmx_serial::check_valid_channel(channel.channel) {
                                Ok(_) => {
                                    self.dmx.as_mut().unwrap().set_channel(channel.channel, channel.value).unwrap();
                                },
                                Err(DMXError::NotValid(cause)) => {
                                    match cause {
                                        dmx_serial::DMXErrorValidity::TooHigh => {
                                            channel.channel = 512;
                                        },
                                        dmx_serial::DMXErrorValidity::TooLow => {
                                            channel.channel = 1;
                                        },
                                    }
                                },
                                Err(e) => {
                                    println!("Error: {}", e);
                                    channel.channel = 1;
                                }
                            } 
                        }
                        //Delete channels with Delete flag
                        self.channels.retain(|channel| !channel.flag);
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
    flag: bool,
}

impl ChannelComponent {
    fn new(free_channels: &mut[bool; 512], channel:usize, value: u8) -> Result<Self, DMXError> {
        match dmx_serial::check_valid_channel(channel) {
            Ok(_) => {
                if free_channels[channel -1] {
                    Ok(Self {
                        channel,
                        value,
                        flag: false,
                    })
                } else {
                    Err(DMXError::AlreadyInUse)
                }
            },
            Err(e) => Err(e),
        }
    }

    fn create_next(free_channels: &mut[bool; 512], value: u8) -> Result<Self, DMXError> {
        if let Some(index) = free_channels.iter().position(|&x| x) {
            free_channels[index] = false;
            Ok(Self {
                channel: index + 1,
                value,
                flag: false,
            })
        } else {
            Err(DMXError::NoChannels)
        }
    }

    fn update(&mut self, free_channels: &mut [bool; 512], ui: &mut egui::Ui) {
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
                if ui.add(egui::Button::new("X")).on_hover_text("Delete Channel").clicked() {
                    self.unregister(free_channels);
                    self.flag = true;
                }
            });
        });
    }

    fn unregister(&mut self, free_channels: &mut [bool; 512]) {
        println!("Unregistering channel {}", self.channel);
        free_channels[self.channel - 1] = true;
    }
}