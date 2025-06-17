#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

mod emustatus;
use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let  mut emulator = emustatus::Chip8Emu::new(); //init

    eframe::run_simple_native("Chip8Emu", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.heading("CHIP-8 Emulator");

            //Emu UI
            ui.label("Display:");

            if ui.button("Load ROM").clicked() {
                // Load ROM logic
                emulator.memorymap();
            }
        });
    })
}
