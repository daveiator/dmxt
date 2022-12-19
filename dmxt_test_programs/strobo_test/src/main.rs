use open_dmx::DMXSerial;
use eframe::egui;
use std::{thread, sync::mpsc};

struct StrobeApp {
    strobe: StrobeTime,
    intensity: u8,
    tx: mpsc::SyncSender<(StrobeTime, u8)>,
}

impl StrobeApp {
    pub fn new() -> Self {

        let (tx, rx) = mpsc::sync_channel(1);
        let strobe = StrobeTime::Off;
        let intensity =  255;

        thread::spawn(move || {
            let mut dmx = DMXSerial::open("COM4").unwrap();
            let mut s = strobe;
            let mut i = intensity;
            loop {
                if let Ok((strobe, intensity)) = rx.try_recv() {
                    println!("Received: {:?}, {:?}", strobe, intensity);
                    s = strobe;
                    i = intensity;
                }
                match s {
                    StrobeTime::Time(t) => {
                        dmx.set_channels([i; 512]);
                        dmx.update();
                        thread::sleep(t);
                        dmx.set_channels([0; 512]);
                        dmx.update();
                        thread::sleep(t);
                    }
                    StrobeTime::Instant => {
                        dmx.set_channels([i; 512]);
                        dmx.update();
                        dmx.set_channels([0; 512]);
                        dmx.update();
                    }
                    StrobeTime::Off => {
                        dmx.set_channels([0; 512]);
                        dmx.update();
                    }
                    
                }
            }
        });
        Self {
            strobe: StrobeTime::Off,
            intensity: 255,
            tx,
        }
    }
}
impl eframe::App for StrobeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui|{
                ui.heading("Strobe");
                ui.horizontal(|ui| {
                    ui.style_mut().spacing.button_padding = egui::vec2(40.0, 20.0);
                    if ui.button("   0   ").on_hover_text("Strobe off").hovered() { self.strobe = StrobeTime::Off; }
                    if ui.button("   1   ").on_hover_text("Strobe Intensity").hovered() { self.strobe = StrobeTime::Time(std::time::Duration::from_millis(100)); }
                    if ui.button("   2   ").on_hover_text("Strobe Intensity").hovered() { self.strobe = StrobeTime::Time(std::time::Duration::from_millis(50)); }
                    if ui.button("   3   ").on_hover_text("Strobe Intensity").hovered() { self.strobe = StrobeTime::Instant; }
                });
                ui.separator();
                ui.add(egui::Slider::new(&mut self.intensity, 0..=255));
            });
        });
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {
        if let Err(mpsc::TrySendError::Disconnected(_)) = self.tx.try_send((self.strobe, self.intensity)) {
            panic!("DMX-Thread disconnected");
        }
        println!("Updating DMX: {:?}, {:?}" , self.strobe, self.intensity);
    }
}

#[derive(Debug, Clone, Copy)]
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
