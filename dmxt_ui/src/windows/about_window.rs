use std::thread;

use eframe::egui::{Context, Window, Vec2, Align2, Sense, Response};


pub fn about_window(ctx: &Context) -> Response {
    let mut res: Option<Response> = None;
    Window::new("About")
    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
    .fixed_size(Vec2::new(400.0, 200.0))
    .title_bar(false)
    .show(ctx, |ui| {
        let (_, response) = ui.allocate_exact_size(ui.available_size(), Sense::hover());
        ui.label("DMXT");
        ui.label("Version: 0.0.0");
        res = Some(response);
    });
    res.unwrap()
}