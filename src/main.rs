#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, epaint::{Shape, FontId, Color32, Pos2}, emath::Align2};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    text_cache: Option<Shape>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text_cache: None,
        }
    }
}

const CACHE_ENABLED: bool = true;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if CACHE_ENABLED {
                if self.text_cache.is_none() {
                    self.text_cache = Some(Shape::text(&ctx.fonts(), Pos2::new(100.0, 100.0), Align2::LEFT_TOP, "A", FontId::proportional(60.0), Color32::BLACK))
                }
            } else {
                self.text_cache = Some(Shape::text(&ctx.fonts(), Pos2::new(100.0, 100.0), Align2::LEFT_TOP, "A", FontId::proportional(60.0), Color32::BLACK))
            }
            ctx.set_pixels_per_point(1.5);
            ui.painter().add(self.text_cache.as_ref().unwrap().clone());
        });
    }
}
