use eframe::egui::{Context, Modifiers, ScrollArea, Ui};
use eframe::egui;
use eframe::egui::Color32;

pub struct MainWindow {
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
        }
    }
    pub fn ui(&mut self, ctx: &Context) {

        let my_frame = egui::containers::Frame {
            inner_margin: egui::style::Margin { left: 10., right: 10., top: 10., bottom: 10. },
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            fill: Color32::from_rgb(27, 27, 27),
            ..Default::default()
        };
        egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {});


        egui::SidePanel::left("groups")
        .resizable(false)
        .default_width(300.0)
        .show(ctx, |ui| {
            ui.label("Groups");
        });


        egui::SidePanel::right("egui_demo_panel")
        .resizable(false)
        .default_width(150.0)
        .show(ctx, |ui| {
            egui::trace!(ui);
            ui.vertical_centered(|ui| {
                ui.heading("✒ egui demos");
            });
            
            ui.separator();
            
            use egui::special_emojis::{GITHUB, TWITTER};
            ui.hyperlink_to(
                format!("{} egui on GitHub", GITHUB),
                "https://github.com/emilk/egui",
            );
            ui.hyperlink_to(
                format!("{} @ernerfeldt", TWITTER),
                "https://twitter.com/ernerfeldt",
            );
            
            ui.separator();
            
        });
        
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
            });
        });
        
    }
}