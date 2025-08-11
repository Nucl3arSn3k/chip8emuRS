#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]
mod emustatus;
mod opcodeparse;
use eframe::egui;
use tinyfiledialogs::open_file_dialog;

use crate::opcodeparse::dump_rom;
fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let mut emulator = emustatus::Chip8Emu::new(); //init
    eframe::run_simple_native("Chip8Emu", options, move |ctx, _frame| {
    // Menu bar in its own top panel
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    let choice = open_file_dialog("Open File", "", Some((&[".ch8"], "CHIP-8 ROM Files")));
                    match choice{
                        Some(o) => {

                            println!("File string: {}",o);
                            let vec = dump_rom(o);

                            match vec{
                                Ok(o) => {
                                    println!("{:?}",o); //mem is valid in here think we cheat by just 
                                    emulator.mapmem(o);//but it IS Here actually!
                                    emulator.dumpmemory();
                                    opcodeparse::parser_gen(&mut emulator);
                                },
                                Err(_) => todo!(),
                            }
                            
                        },
                        None => {println!("No file selected")},
                    }
                }
                if ui.button("Display test").clicked() { //Fire the displaytest,it actually works now
                    emulator.displaytest();
                }
                if ui.button("Load file").clicked() {

                    //emulator.openself();
                    emulator.dumpmemory();
                }
            });
        });
    });
    
    // Display in the central panel
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::Frame::none()
            .fill(egui::Color32::BLACK) // Black background for the screen
            .show(ui, |ui| {
                let loc = emulator.display;
                for (x, row) in loc.iter().enumerate() {
                    for (y, element) in row.iter().enumerate() {
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
    });
})
}