#![windows_subsystem = "windows"]

pub mod types;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Status Tracker",
        options,
        Box::new(|_cc| Ok(Box::<Content>::default())),
    )
}

#[derive(Default)]
struct Content {
    game: types::Game,
    text: String,
    last_index: usize,
    last_entered_commands: Vec<String>,
    last_result: String,
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Enter Commands");
            ui.label(self.last_result.clone());
            ui.text_edit_singleline(&mut self.text);
            if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.last_index = 0;
                let result = self.game.process_command(self.text.clone());
                if result.is_err() {
                    self.last_result = result.err().unwrap();
                } else {
                    self.last_result = result.ok().unwrap().to_string();
                }
                self.last_entered_commands.push(self.text.clone());
                self.text = String::new();
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                if self.last_index < self.last_entered_commands.len() {
                    self.last_index += 1;
                }
                if self.last_index != 0 {
                    self.text = self.last_entered_commands
                        [self.last_entered_commands.len() - self.last_index]
                        .clone()
                }
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                if self.last_index > 0 {
                    self.last_index -= 1;
                }
                if self.last_index != 0 {
                    self.text = self.last_entered_commands
                        [self.last_entered_commands.len() - self.last_index]
                        .clone()
                }
                if self.last_index == 0 {
                    self.text = " ".to_string();
                }
            }
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(self.game.get_entities_list());
            });
        });
    }
}
