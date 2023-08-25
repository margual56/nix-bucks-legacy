use std::{collections::HashMap, fs::File, io::Read};

use directories::ProjectDirs;
use eframe::egui::{self, RichText};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{FixedExpense, NewSubscriptionWindow, Subscription};

#[derive(Serialize, Deserialize)]
pub struct App {
    initial_savings: f64,
    subscriptions: HashMap<Uuid, Subscription>,
    fixed_expenses: HashMap<Uuid, FixedExpense>,

    #[serde(skip)]
    show_new_subscription_window: Option<NewSubscriptionWindow>,
}

impl Default for App {
    fn default() -> Self {
        if let Some(dir) = ProjectDirs::from("com", "margual56", "Budgeting App") {
            let mut path = match std::fs::File::open(dir.config_dir().join("config.json")) {
                Ok(p) => p,
                Err(e) => {
                    println!("Error while opening file: {}", e);
                    return Self {
                        initial_savings: 0.0,
                        subscriptions: HashMap::new(),
                        fixed_expenses: HashMap::new(),

                        show_new_subscription_window: None,
                    };
                }
            };

            let mut buffer = String::new();

            path.read_to_string(&mut buffer).unwrap();

            serde_json::from_str(&buffer).unwrap()
        } else {
            println!("Directory not found, returning default value");
            Self {
                initial_savings: 0.0,
                subscriptions: HashMap::new(),
                fixed_expenses: HashMap::new(),

                show_new_subscription_window: None,
            }
        }
    }
}

impl App {
    fn save_data(&self) {
        if let Some(dir) = ProjectDirs::from("com", "margual56", "Budgeting App") {
            if !dir.config_dir().exists() {
                std::fs::create_dir_all(dir.config_dir()).unwrap();
            }

            let path = File::create(dir.config_dir().join("config.json")).unwrap();

            serde_json::to_writer_pretty(path, self).unwrap();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_debug_on_hover(true);
        if let Some(win) = self.show_new_subscription_window.as_mut() {
            let mut show = true;

            if let Some(result) = win.show(ctx, &mut show) {
                self.subscriptions.insert(result.uuid(), result);

                self.save_data();

                self.show_new_subscription_window = None;
            } else if !show {
                self.show_new_subscription_window = None;
            }
        }

        use egui_extras::{Size, StripBuilder};
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.3).at_most(400.0).at_least(200.0))
                    .size(Size::remainder().at_most(50.0))
                    .size(Size::relative(0.3).at_most(400.0).at_least(200.0))
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.vertical_centered_justified(|ui| {
                                ui.push_id("subscriptions", |ui| {
                                    TableBuilder::new(ui)
                                        .striped(true)
                                        .auto_shrink([true, true])
                                        .cell_layout(egui::Layout::left_to_right(
                                            egui::Align::Center,
                                        ))
                                        .column(Column::auto().at_most(100.0).resizable(true))
                                        .column(Column::auto().at_most(100.0).resizable(true))
                                        .column(
                                            Column::auto()
                                                .at_most(200.0)
                                                .at_least(150.0)
                                                .resizable(true),
                                        )
                                        .column(
                                            Column::auto()
                                                .at_most(100.0)
                                                .at_least(50.0)
                                                .resizable(true),
                                        )
                                        .header(20.0, |mut header| {
                                            header.col(|ui| {
                                                ui.heading("Concept");
                                            });
                                            header.col(|ui| {
                                                ui.heading("Cost");
                                            });
                                            header.col(|ui| {
                                                ui.heading("Recurrence");
                                            });
                                        })
                                        .body(|mut body| {
                                            for (uuid, subscription) in self.subscriptions.clone() {
                                                body.row(25.0, |mut row| {
                                                    row.col(|ui| {
                                                        ui.label(RichText::new(
                                                            subscription.name(),
                                                        ));
                                                    });
                                                    row.col(|ui| {
                                                        ui.label(RichText::new(format!(
                                                            "{:.2}€",
                                                            subscription.cost()
                                                        )));
                                                    });
                                                    row.col(|ui| {
                                                        ui.label(RichText::new(
                                                            subscription.recurrence().to_string(),
                                                        ));
                                                    });
                                                    row.col(|ui| {
                                                        if ui.button("Delete").clicked() {
                                                            self.subscriptions.remove(&uuid);
                                                            self.save_data();
                                                        }
                                                    });
                                                });
                                            }
                                        });
                                });

                                ui.separator();

                                if ui.button("New subscription").clicked() {
                                    self.show_new_subscription_window =
                                        Some(NewSubscriptionWindow::default());
                                }
                            });
                        });

                        strip.empty();

                        strip.cell(|ui| {
                            ui.vertical_centered_justified(|ui| {
                                ui.push_id("expenses", |ui| {
                                    TableBuilder::new(ui)
                                        .striped(true)
                                        .auto_shrink([true, true])
                                        .column(Column::auto().resizable(true))
                                        .column(Column::auto().resizable(true))
                                        .column(Column::remainder())
                                        .header(20.0, |mut header| {
                                            header.col(|ui| {
                                                ui.heading("Concept");
                                            });
                                            header.col(|ui| {
                                                ui.heading("Cost");
                                            });
                                            header.col(|ui| {
                                                ui.heading("Recurrence");
                                            });
                                        })
                                        .body(|mut body| {
                                            body.row(20.0, |mut _row| {});
                                        });
                                });
                                ui.separator();

                                if ui.button("New subscription").clicked() {
                                    self.show_new_subscription_window =
                                        Some(NewSubscriptionWindow::default());
                                }
                            });
                        });
                    });
            });
        });
    }
}
