#![windows_subsystem = "windows"]

use eframe::{egui::{self, RichText, TextEdit, TextStyle, Ui}};
use regex::Regex;

#[derive(Default)]
struct App {
    title: RichText,
    version: RichText,
    text: String,
    regex: String,
    start: usize,
    end: usize,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            title: RichText::new("Rexus")
                .size(24.0)
                .color(egui::Color32::from_rgb(153, 204, 255)),
            version: RichText::new("Scr44gr@v0.1.0")
                .size(16.0)
                .color(egui::Color32::from_rgb(252, 117, 117)),
            text: "Hello, world!".to_string(),
            regex: "".to_string(),
            start: 0,
            end: 0,
        }
    }

}

impl eframe::App for App {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(self.title.to_owned());
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.vertical_centered(|ui| {
                ui.label("Tiny regular expression tester");
                ui.add_space(50.0);
                ui.text_edit_multiline(&mut self.text);

                ui.add_space(50.0);
                if self.end >= 1 && self.text.len() >= self.end {
                    ui.add(|ui: &mut Ui| {
                        let text = format!(
                            "Found: {}",
                            self.text[self.start..self.end].to_string()
                        );
                        ui.colored_label(egui::Color32::from_rgb(124, 243, 120), text)
                    });
                } else {
                    ui.add(|ui: &mut Ui| {
                        ui.colored_label(egui::Color32::from_rgb(252, 117, 117), "No match")
                    });
                }
                ui.add_space(50.0);
                let response =
                    ui.add(TextEdit::multiline(&mut self.regex).font(TextStyle::Monospace));

                if response.changed() {
                    let re_exp = Regex::new(&self.regex);
                    match re_exp {
                        Ok(re) => {
                            let captures = re.captures(&self.text);
                            if captures.is_some() {
                                for cap in captures.unwrap().iter() {
                                    self.start = cap.unwrap().start();
                                    self.end = cap.unwrap().end();
                                }
                            }
                        }
                        _ => {}
                    }
                }
            });
        });

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(self.version.to_owned());
            });
        });
    }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(350., 450.));
    native_options.resizable = false;
    native_options.vsync = true;
    eframe::run_native(
        "Rexus",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
