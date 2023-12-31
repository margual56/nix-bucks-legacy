use eframe::egui;

use nix_bucks::App;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 800.0)),
        resizable: false,
        follow_system_theme: true,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..Default::default()
    };

    eframe::run_native("NixBucks", options, Box::new(|cc| Box::new(App::new(cc)))).unwrap();
}
