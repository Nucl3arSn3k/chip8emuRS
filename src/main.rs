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
    let mut emulator = emustatus::Chip8Emu::new(); //init
    eframe::run_simple_native("Chip8Emu", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.heading("CHIP-8 Emulator");
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {}
                    if ui.button("Display test").clicked() {
                        emulator.displaytest();
                    }
                });
            });

            let loc = emulator.display;
            for (x, row) in loc.iter().enumerate() {
                for (y, element) in row.iter().enumerate() {
                    println!("Element at [{x}][{y}]: {element}");
                    if element == &1u8 {
                        //draw a white square
                        let rect = egui::Rect::from_min_size(
                            egui::pos2(y as f32 * 5.0, x as f32 * 5.0),
                            egui::vec2(5.0, 5.0),
                        );
                        ui.painter().rect_filled(rect, 0.0, egui::Color32::WHITE);
                    } else if element == &0u8 {
                        //draw a black square
                        let rect = egui::Rect::from_min_size(
                            egui::pos2(y as f32 * 5.0, x as f32 * 5.0),
                            egui::vec2(5.0, 5.0),
                        );
                        ui.painter().rect_filled(rect, 0.0, egui::Color32::BLACK);
                    }
                }
            }
        });
    })
}
