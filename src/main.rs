use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 330.0]),
        ..Default::default()
    };
    eframe::run_native("Egui calculator", options, Box::new(|cc| Ok(Box::<MyApp>::default())))
}
enum Screen {
    Calculator,
    Converter,
}
struct MyApp {
    current_screen: Screen,
    input: String,
    result: String,
    from_base: usize,
    to_base: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_screen: Screen::Calculator,
            input: String::new(),
            result: "0".to_string(),
            from_base: 10,
            to_base: 16,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Calculator").frame(false)).clicked() {
                    self.current_screen = Screen::Calculator
                }
                if ui.add(egui::Button::new("Converter").frame(false)).clicked() {
                    self.current_screen = Screen::Converter
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_screen {
                Screen::Calculator => {
                    ui.heading(&self.input);
                    ui.label(egui::RichText::new(format!("= {}", self.result)).color(egui::Color32::GREEN).strong().monospace());

                    let rows = vec![
                        vec!["(", ")", "%", "CE"],
                        vec!["7", "8", "9", "÷"],
                        vec!["4", "5", "6", "×"],
                        vec!["1", "2", "3", "−"],
                        vec!["0", ".", "=", "+"],
                        vec!["π", "√", "^", "C"],
                        vec!["sin", "cos", "tan", "exp"],
                        vec!["ln", "log", "abs", "rand"],
                    ];

                    let available_width = ui.available_width();
                    let button_width = available_width / 4.0;
                    let button_height = 30.0;

                    egui::Grid::new("parent grid").spacing([1.0, 1.0]).striped(true).show(ui, |ui| {
                        for row in &rows {
                            for &label in row {
                                let button = ui.add_sized([button_width, button_height], egui::Button::new(label));

                                if button.clicked() {
                                    self.on_button(label);
                                }
                            }
                            ui.end_row();
                        }
                    });
                }
                Screen::Converter => {
                    ui.heading("Converter");

                    ui.horizontal(|ui| {
                        ui.label("Number:");
                        ui.text_edit_singleline(&mut self.input);
                    });

                    ui.horizontal(|ui| {
                        ui.label("From system:");
                        egui::ComboBox::from_id_salt("from_base")
                            .selected_text(self.from_base.to_string())
                            .show_ui(ui, |ui| {
                                for base in &[2, 8, 10, 16] {
                                    ui.selectable_value(&mut self.from_base, *base, base.to_string());
                                }
                        });
                    });

                    ui.horizontal(|ui| {
                        ui.label("To system:");
                        egui::ComboBox::from_id_salt("to_base")
                            .selected_text(self.to_base.to_string())
                            .show_ui(ui, |ui| {
                                for base in &[2, 8, 10, 16] {
                                    ui.selectable_value(&mut self.to_base, *base, base.to_string());
                                }
                        });
                    });

                    if ui.button("Convert!").clicked() {
                        self.convert();
                    }

                    ui.label(egui::RichText::new(format!("Result: {}", self.result)).color(egui::Color32::GREEN).strong().size(15.0).monospace());
                }
            }
        });
    }
}

impl MyApp {
    fn on_button(&mut self, label: &str) {
        match label {
            "C" => {
                self.input.clear();
                self.result = "0".to_string()
            }
            "CE" => {
                self.input.pop();
            }
            "=" => {
                self.evaluate();
            }
            "π" => self.input.push_str("pi"),
            "√" => self.input.push_str("sqrt("),
            "÷" => self.input.push('/'),
            "×" => self.input.push('*'),
            "−" => self.input.push('-'),
            "sin" | "cos" | "tan" | "ln" | "log" | "exp" | "abs" => {
                self.input.push_str(label);
                self.input.push('(');
            }
            "rand" => {
                self.input.push_str("rand()");
            }
            _ => self.input.push_str(label),
        }
    }

    fn evaluate(&mut self) {
        match meval::eval_str(&self.input) {
            Ok(value) => self.result = value.to_string(),
            Err(_) => self.result = "Error".to_string(),
        }
    }

    fn convert(&mut self) {
        match u64::from_str_radix(&self.input, self.from_base as u32) {
            Ok(value) => {
                self.result = match self.to_base {
                    2 => format!("{:b}", value),
                    8 => format!("{:o}", value),
                    10 => value.to_string(),
                    16 => format!("{:X}", value),
                    _ => "Invalid system".to_string(),
                };
            }
            Err(_) => {
                self.result = "Input error".to_string();
            }
        }
    }
}