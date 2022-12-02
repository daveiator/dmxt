use dmxt_lib::dmx::serial::DMXSerial;
use eframe::egui;
use std::thread;

struct StrobeApp {
    dmx: DMXSerial,
    strobe: StrobeTime,
    intensity: u8,
}

impl StrobeApp {
    pub fn new() -> Self {
        Self {
            dmx: DMXSerial::open("COM4").unwrap(),
            strobe: StrobeTime::Off,
            intensity: 255,
        }
    }
}
impl eframe::App for StrobeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui|{
                ui.heading("Strobe");
                ui.horizontal(|ui| {
                    ui.style_mut().spacing.button_padding = egui::vec2(40.0, 20.0);
                    if ui.button("   0   ").on_hover_text("Strobe off").hovered() { self.strobe = StrobeTime::Off; }
                    if ui.button("   1   ").on_hover_text("Strobe Intensity").hovered() { self.strobe = StrobeTime::Time(std::time::Duration::from_millis(100)); }
                    if ui.button("   2   ").on_hover_text("Strobe Intensity").hovered() { self.strobe = StrobeTime::Time(std::time::Duration::from_millis(50)); }
                    if ui.button("   3   ").on_hover_text("Strobe Intensity").hovered() { self.strobe = StrobeTime::Time(std::time::Duration::from_millis(0)); }
                });
                ui.separator();
                ui.add(egui::Slider::new(&mut self.intensity, 0..=255));
            });
        });
        ctx.request_repaint();
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {
        match self.strobe {
            StrobeTime::Time(t) => {
                self.dmx.set_channels([self.intensity; 512]);
                self.dmx.update();
                thread::sleep(t);
                self.dmx.set_channels([0; 512]);
                self.dmx.update();
                thread::sleep(t);
            }
            StrobeTime::Instant => {
                self.dmx.set_channels([self.intensity; 512]);
                self.dmx.update();
                self.dmx.set_channels([0; 512]);
                self.dmx.update();
            }
            StrobeTime::Off => {
                self.dmx.set_channels([0; 512]);
            }
            
        }
    }
}

enum StrobeTime {
    Off,
    Time(std::time::Duration),
    Instant,
}


fn main() {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::Vec2::new(500.0, 150.0));
    options.initial_window_pos = Some(egui::Pos2::new(960.0 - 100.0, 540.0-50.0-200.0));
    let app = StrobeApp::new();
    eframe::run_native(
        "Strobo-Test",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}
