#![windows_subsystem = "windows"]

pub mod types;
use eframe::egui;

#[derive(Default)]
struct Content {
    game: types::Game,
    text: String,
    last_index: usize,
    last_entered_commands: Vec<String>,
    last_result: String,
}

const COMMANDS: [&str; 9] = [
    "add_entity",
    "remove_entity",
    "add_effect",
    "remove_effect",
    "damage",
    "heal",
    "clear",
    "save",
    "load",
];

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Enter Commands, use help <command> for more info");

            ui.label(self.last_result.clone());

            let text_field = egui::TextEdit::singleline(&mut self.text)
                .desired_width(f32::INFINITY)
                .lock_focus(true);

            let output = text_field.show(ui);

            if ctx.input(|i| i.key_pressed(egui::Key::Tab)) {
                let last_word = self.text.split_whitespace().last();
                match last_word {
                    Some(word) => {
                        let mut new_text: Option<String> = None;
                        let mut matchables = self.game.get_matchable_names();
                        for command in COMMANDS.iter() {
                            matchables.push(command.to_string());
                        }
                        for name in matchables.iter() {
                            if name.to_lowercase().starts_with(&word.to_lowercase()) {
                                if new_text.is_some() {
                                    new_text = None;
                                    break;
                                }
                                new_text = Some(
                                    self.text[..self.text.len() - word.len()].to_string() + name,
                                );
                            }
                        }
                        if let Some(new_text) = new_text {
                            self.text = new_text;
                            let text_edit_id = output.response.id;
                            if let Some(mut state) =
                                egui::TextEdit::load_state(ui.ctx(), text_edit_id)
                            {
                                let ccursor = egui::text::CCursor::new(self.text.chars().count());
                                state
                                    .cursor
                                    .set_char_range(Some(egui::text::CCursorRange::one(ccursor)));
                                state.store(ui.ctx(), text_edit_id);
                            }
                            output.response.request_focus();
                        }
                    }
                    None => (),
                }
            }

            if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.handle_enter();
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                self.handle_arrow_up();
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                self.handle_arrow_down();
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(self.game.get_entities_list());
            });

            output.response.request_focus();
        });
    }
}

impl Content {
    fn handle_enter(&mut self) {
        self.last_index = 0;
        let result = self.game.process_command(self.text.clone());
        if result.is_err() {
            self.last_result = "ERROR: ".to_string() + &result.err().unwrap();
        } else {
            self.last_result = result.ok().unwrap().to_string();
        }
        self.last_entered_commands.push(self.text.clone());
        self.text = String::new();
    }

    fn handle_arrow_up(&mut self) {
        if self.last_index < self.last_entered_commands.len() {
            self.last_index += 1;
        }
        if self.last_index != 0 {
            self.text = self.last_entered_commands
                [self.last_entered_commands.len() - self.last_index]
                .clone()
        }
    }

    fn handle_arrow_down(&mut self) {
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
}

fn main() -> eframe::Result {
    let mut options = eframe::NativeOptions::default();
    options.viewport = options.viewport.with_title("Status Tracker").with_inner_size(egui::Vec2 { x: 1000.0, y: 600.0 });
    eframe::run_native(
        "Status Tracker",
        options,
        Box::new(|_cc| Ok(Box::<Content>::default())),
    )
}
