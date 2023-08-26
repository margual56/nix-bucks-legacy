use std::{collections::HashMap, fs::File, io::Read};

use cached::proc_macro::cached;
use chrono::{Datelike, NaiveDate, Utc};
use directories::ProjectDirs;
use eframe::{
    egui::{
        self, InnerResponse, RichText,
        TextStyle::{Body, Button, Heading, Monospace, Name, Small},
    },
    epaint::{Color32, FontFamily, FontId},
};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    FixedExpense, NewExpenseWindow, NewIncomeWindow, NewPunctualIncomeWindow,
    NewSubscriptionWindow, Subscription,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct App {
    initial_savings: f32,
    subscriptions: HashMap<Uuid, Subscription>,
    incomes: HashMap<Uuid, Subscription>,
    fixed_expenses: HashMap<Uuid, FixedExpense>,
    p_incomes: HashMap<Uuid, FixedExpense>,

    #[serde(skip)]
    new_subscription_window: Option<NewSubscriptionWindow>,

    #[serde(skip)]
    new_expense_window: Option<NewExpenseWindow>,

    #[serde(skip)]
    new_income_window: Option<NewIncomeWindow>,

    #[serde(skip)]
    new_p_income_window: Option<NewPunctualIncomeWindow>,
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
                        incomes: HashMap::new(),
                        p_incomes: HashMap::new(),

                        new_subscription_window: None,
                        new_expense_window: None,
                        new_income_window: None,
                        new_p_income_window: None,
                    };
                }
            };

            let mut buffer = String::new();

            path.read_to_string(&mut buffer).unwrap();

            serde_json::from_str::<Self>(&buffer).unwrap().update()
        } else {
            println!("Directory not found, returning default value");
            Self {
                initial_savings: 0.0,
                subscriptions: HashMap::new(),
                fixed_expenses: HashMap::new(),
                incomes: HashMap::new(),
                p_incomes: HashMap::new(),

                new_subscription_window: None,
                new_expense_window: None,
                new_income_window: None,
                new_p_income_window: None,
            }
        }
    }
}

#[cached]
fn cost_to_year_end(subscriptions: Vec<Subscription>, expenses: Vec<FixedExpense>) -> f32 {
    let mut amount = 0.0;
    let year_end = NaiveDate::from_ymd_opt(Utc::now().year(), 12, 31).unwrap();

    for subscription in subscriptions {
        amount += subscription.cost_until(year_end);
    }

    for expense in expenses {
        if Utc::now().naive_utc().date() <= expense.date() && expense.date() <= year_end {
            amount += expense.cost();
        }
    }

    amount
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

    fn update(&self) -> Self {
        let mut app = self.clone();

        let today = Utc::now().date_naive();

        for (uuid, expense) in self.fixed_expenses.clone() {
            if today > expense.date {
                app.initial_savings -= expense.cost();
                app.remove_expense(uuid);
            }
        }

        for (uuid, income) in self.p_incomes.clone() {
            if today > income.date {
                app.initial_savings += income.cost();
                app.remove_punctual_income(&uuid);
            }
        }

        app.save_data();

        app.clone()
    }

    pub fn remove_expense(&mut self, uuid: Uuid) {
        self.fixed_expenses.remove(&uuid);
    }

    pub fn remove_punctual_income(&mut self, uuid: &Uuid) {
        self.p_incomes.remove(uuid);
    }

    #[allow(dead_code)]
    fn yearly_costs(&self) -> f32 {
        let mut amount = 0.0;

        for subscription in self.subscriptions.values() {
            amount += subscription.cost_per_year();
        }

        amount
    }

    fn monthly_costs(&self) -> f32 {
        let mut amount = 0.0;

        for subscription in self.subscriptions.values() {
            amount += subscription.cost_per_month();
        }

        amount
    }

    fn draw_windows(&mut self, ctx: &egui::Context) {
        if let Some(win) = self.new_subscription_window.as_mut() {
            let mut show = true;

            if let Some(result) = win.show(ctx, &mut show) {
                self.subscriptions.insert(result.uuid(), result);

                self.save_data();

                self.new_subscription_window = None;
            } else if !show {
                self.new_subscription_window = None;
            }
        }
        if let Some(win) = self.new_expense_window.as_mut() {
            let mut show = true;

            if let Some(result) = win.show(ctx, &mut show) {
                self.fixed_expenses.insert(result.uuid(), result);

                self.save_data();

                self.new_expense_window = None;
            } else if !show {
                self.new_expense_window = None;
            }
        }

        if let Some(win) = self.new_income_window.as_mut() {
            let mut show = true;

            if let Some(result) = win.show(ctx, &mut show) {
                self.incomes.insert(result.uuid(), result);

                self.save_data();

                self.new_income_window = None;
            } else if !show {
                self.new_income_window = None;
            }
        }

        if let Some(win) = self.new_p_income_window.as_mut() {
            let mut show = true;

            if let Some(result) = win.show(ctx, &mut show) {
                self.p_incomes.insert(result.uuid(), result);

                self.save_data();

                self.new_p_income_window = None;
            } else if !show {
                self.new_p_income_window = None;
            }
        }
    }

    fn subscriptions_table(&mut self, ui: &mut egui::Ui) -> InnerResponse<()> {
        ui.vertical_centered_justified(|ui| {
            ui.heading("Subscriptions");
            ui.separator();
            ui.push_id("subscriptions", |ui| {
                egui::ScrollArea::both()
                    .id_source("Subscriptions scroll area")
                    .auto_shrink([true, true])
                    .max_height(200.0)
                    .show(ui, |ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .auto_shrink([true, true])
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(
                                Column::auto()
                                    .at_least(100.0)
                                    .at_most(200.0)
                                    .resizable(true),
                            )
                            .column(Column::auto().at_most(100.0).resizable(true))
                            .column(
                                Column::auto()
                                    .at_most(200.0)
                                    .at_least(150.0)
                                    .resizable(true),
                            )
                            .column(Column::auto().at_least(50.0).at_most(100.0).resizable(true))
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
                                            ui.label(RichText::new(subscription.name()));
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
            });

            ui.separator();

            if ui.button("New subscription").clicked() {
                self.new_subscription_window = Some(NewSubscriptionWindow::default());
            }
        })
    }

    fn expenses_table(&mut self, ui: &mut egui::Ui) -> InnerResponse<()> {
        ui.vertical_centered_justified(|ui| {
            ui.heading("Fixed expenses");
            ui.separator();
            ui.push_id("expenses", |ui| {
                egui::ScrollArea::both()
                    .id_source("Expenses scroll area")
                    .auto_shrink([true, true])
                    .max_height(ui.available_height() - 35.0)
                    .show(ui, |ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .auto_shrink([true, true])
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(
                                Column::auto()
                                    .at_least(100.0)
                                    .at_most(200.0)
                                    .resizable(true),
                            )
                            .column(Column::auto().at_most(100.0).resizable(true))
                            .column(
                                Column::auto()
                                    .at_least(150.0)
                                    .at_most(200.0)
                                    .resizable(true),
                            )
                            .column(Column::auto().at_least(50.0).at_most(100.0).resizable(true))
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.heading("Concept");
                                });
                                header.col(|ui| {
                                    ui.heading("Cost");
                                });
                                header.col(|ui| {
                                    ui.heading("Date");
                                });
                            })
                            .body(|mut body| {
                                for (uuid, expense) in self.fixed_expenses.clone() {
                                    body.row(25.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(RichText::new(expense.name()));
                                        });
                                        row.col(|ui| {
                                            ui.label(RichText::new(format!(
                                                "{:.2}€",
                                                expense.cost()
                                            )));
                                        });
                                        row.col(|ui| {
                                            ui.label(RichText::new(expense.date().to_string()));
                                        });
                                        row.col(|ui| {
                                            if ui.button("Delete").clicked() {
                                                self.fixed_expenses.remove(&uuid);
                                                self.save_data();
                                            }
                                        });
                                    });
                                }
                            });
                    });
            });
            ui.separator();

            if ui.button("New fixed expense").clicked() {
                self.new_expense_window = Some(NewExpenseWindow::default());
            }
        })
    }

    fn results_table(&self, ui: &mut egui::Ui) -> InnerResponse<()> {
        ui.vertical(|ui| {
            ui.add_space(20.0);
            ui.vertical_centered(|ui| {
                ui.heading("Stats");
            });
            ui.spacing();

            ui.horizontal(|ui| {
                ui.push_id("results", |ui| {
                    TableBuilder::new(ui)
                        .striped(false)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                        .auto_shrink([true, true])
                        .column(Column::remainder().at_least(150.0).at_most(300.0))
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .column(Column::remainder().at_least(200.0))
                        .header(20.0, |mut _header| {})
                        .body(|mut body| {
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.spacing();
                                });

                                row.col(|ui| {
                                    ui.label(RichText::new(
                                        "Total cost until the end of the year:",
                                    ));
                                });
                                row.col(|ui| {
                                    ui.label(
                                        RichText::new(format!(
                                            "-{:.2}€",
                                            cost_to_year_end(
                                                self.subscriptions.clone().into_values().collect(),
                                                self.fixed_expenses.clone().into_values().collect()
                                            )
                                        ))
                                        .color(ui.visuals().error_fg_color),
                                    );
                                });
                                row.col(|ui| {
                                    ui.spacing();
                                });
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.spacing();
                                });
                                row.col(|ui| {
                                    ui.label(RichText::new("Total (average) cost per month:"));
                                });
                                row.col(|ui| {
                                    ui.label(
                                        RichText::new(format!("-{:.2}€", self.monthly_costs()))
                                            .color(Color32::RED),
                                    );
                                });
                                row.col(|ui| {
                                    ui.spacing();
                                });
                            });

                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.spacing();
                                });
                                row.col(|ui| {
                                    ui.label(RichText::new(
                                        "Total income until the end of the year:",
                                    ));
                                });

                                row.col(|ui| {
                                    ui.label(
                                        RichText::new(format!(
                                            "+{:.2}€",
                                            cost_to_year_end(
                                                self.incomes.clone().into_values().collect(),
                                                self.p_incomes.clone().into_values().collect()
                                            )
                                        ))
                                        .color(Color32::GREEN),
                                    );
                                });
                                row.col(|ui| {
                                    ui.spacing();
                                });
                            });

                            body.row(5.0, |mut row| {
                                row.col(|ui| {
                                    ui.spacing();
                                });
                                row.col(|ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        ui.separator();
                                    });
                                });
                                row.col(|ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        ui.separator();
                                    });
                                });
                                row.col(|ui| {
                                    ui.spacing();
                                });
                            });

                            // Total balance at the end of the year
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.spacing();
                                });
                                row.col(|ui| {
                                    ui.label(
                                        RichText::new("Total balance at the end of the year:")
                                            .strong(),
                                    );
                                });

                                row.col(|ui| {
                                    let balance = self.initial_savings
                                        + cost_to_year_end(
                                            self.incomes.clone().into_values().collect(),
                                            self.p_incomes.clone().into_values().collect(),
                                        )
                                        - cost_to_year_end(
                                            self.subscriptions.clone().into_values().collect(),
                                            self.fixed_expenses.clone().into_values().collect(),
                                        );

                                    ui.label(
                                        RichText::new(format!("{:+.2}€", balance))
                                            .color(if balance < 0.0 {
                                                Color32::RED
                                            } else {
                                                Color32::GREEN
                                            })
                                            .strong(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.spacing();
                                });
                            });
                        });
                });
            });
        })
    }

    fn income_table(&mut self, ui: &mut egui::Ui) -> InnerResponse<()> {
        ui.vertical_centered_justified(|ui| {
            ui.heading("Income streams");
            ui.separator();
            ui.push_id("incomes", |ui| {
                egui::ScrollArea::both()
                    .id_source("Subscriptions1 scroll area")
                    .auto_shrink([true, true])
                    .max_height(ui.available_height() - 35.0)
                    .show(ui, |ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .auto_shrink([true, true])
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(
                                Column::auto()
                                    .at_least(100.0)
                                    .at_most(200.0)
                                    .resizable(true),
                            )
                            .column(Column::auto().at_most(100.0).resizable(true))
                            .column(
                                Column::auto()
                                    .at_most(200.0)
                                    .at_least(150.0)
                                    .resizable(true),
                            )
                            .column(Column::auto().at_least(50.0).at_most(100.0).resizable(true))
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.heading("Concept");
                                });
                                header.col(|ui| {
                                    ui.heading("Amount");
                                });
                                header.col(|ui| {
                                    ui.heading("Recurrence");
                                });
                            })
                            .body(|mut body| {
                                for (uuid, subscription) in self.incomes.clone() {
                                    body.row(25.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(RichText::new(subscription.name()));
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
                                                self.incomes.remove(&uuid);
                                                self.save_data();
                                            }
                                        });
                                    });
                                }
                            });
                    });
            });

            ui.separator();

            if ui.button("New income stream").clicked() {
                self.new_income_window = Some(NewIncomeWindow::default());
            }
        })
    }

    fn punctual_income_table(&mut self, ui: &mut egui::Ui) -> InnerResponse<()> {
        ui.vertical_centered_justified(|ui| {
            ui.heading("Punctial income");
            ui.separator();
            egui::ScrollArea::both()
                .id_source("Expenses1 scroll area")
                .auto_shrink([true, true])
                .max_height(ui.available_height() - 35.0)
                .show(ui, |ui| {
                    ui.push_id("punctial_incomes", |ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .auto_shrink([true, true])
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(
                                Column::auto()
                                    .at_least(100.0)
                                    .at_most(200.0)
                                    .resizable(true),
                            )
                            .column(Column::auto().at_most(100.0).resizable(true))
                            .column(
                                Column::auto()
                                    .at_most(200.0)
                                    .at_least(150.0)
                                    .resizable(true),
                            )
                            .column(Column::auto().at_least(50.0).at_most(100.0).resizable(true))
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.heading("Concept");
                                });
                                header.col(|ui| {
                                    ui.heading("Amount");
                                });
                                header.col(|ui| {
                                    ui.heading("Date");
                                });
                            })
                            .body(|mut body| {
                                for (uuid, expense) in self.p_incomes.clone() {
                                    body.row(25.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(RichText::new(expense.name()));
                                        });
                                        row.col(|ui| {
                                            ui.label(RichText::new(format!(
                                                "{:.2}€",
                                                expense.cost()
                                            )));
                                        });
                                        row.col(|ui| {
                                            ui.label(RichText::new(expense.date().to_string()));
                                        });
                                        row.col(|ui| {
                                            if ui.button("Delete").clicked() {
                                                self.p_incomes.remove(&uuid);
                                                self.save_data();
                                            }
                                        });
                                    });
                                }
                            });
                    });
                });
            ui.separator();

            if ui.button("New punctial income").clicked() {
                self.new_p_income_window = Some(NewPunctualIncomeWindow::default());
            }
        })
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Get current context style
        let mut style = (*ctx.style()).clone();

        // Redefine text_styles
        style.text_styles = [
            (Heading, FontId::new(25.0, FontFamily::Proportional)),
            (
                Name("Context".into()),
                FontId::new(23.0, FontFamily::Proportional),
            ),
            (Body, FontId::new(18.0, FontFamily::Proportional)),
            (Monospace, FontId::new(15.0, FontFamily::Proportional)),
            (Button, FontId::new(16.0, FontFamily::Proportional)),
            (Small, FontId::new(10.0, FontFamily::Proportional)),
        ]
        .into();

        // Mutate global style with above changes
        ctx.set_style(style);
        self.draw_windows(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.collapsing(RichText::new("Expenses").heading(), |ui| {
                        ui.horizontal(|ui| {
                            TableBuilder::new(ui)
                                .auto_shrink([false, true])
                                .vscroll(false)
                                .column(
                                    Column::remainder()
                                        .at_least(400.0)
                                        .at_most(450.0)
                                        .clip(true)
                                        .resizable(false),
                                )
                                .column(Column::auto().at_least(25.0))
                                .column(
                                    Column::remainder()
                                        .at_least(400.0)
                                        .at_most(450.0)
                                        .clip(true)
                                        .resizable(false),
                                )
                                .body(|mut body| {
                                    body.row(200.0, |mut row| {
                                        row.col(|ui| {
                                            self.subscriptions_table(ui);
                                        });

                                        row.col(|ui| {
                                            ui.spacing();
                                        });

                                        row.col(|ui| {
                                            self.expenses_table(ui);
                                        });
                                    })
                                });
                        });
                    });

                    ui.add_space(25.0);

                    ui.collapsing(RichText::new("Income").heading(), |ui| {
                        ui.horizontal(|ui| {
                            TableBuilder::new(ui)
                                .vscroll(false)
                                .auto_shrink([false, true])
                                .column(
                                    Column::remainder()
                                        .at_least(400.0)
                                        .at_most(450.0)
                                        .clip(true)
                                        .resizable(false),
                                )
                                .column(Column::auto().at_least(25.0))
                                .column(
                                    Column::remainder()
                                        .at_least(400.0)
                                        .at_most(450.0)
                                        .clip(true)
                                        .resizable(false),
                                )
                                .body(|mut body| {
                                    body.row(200.0, |mut row| {
                                        row.col(|ui| {
                                            self.income_table(ui);
                                        });

                                        row.col(|ui| {
                                            ui.spacing();
                                        });

                                        row.col(|ui| {
                                            self.punctual_income_table(ui);
                                        });
                                    });
                                });
                        });
                    });

                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);

                    ui.horizontal(|ui| {
                        ui.heading("Initial savings: ");

                        let prev = self.initial_savings;
                        ui.add(
                            egui::DragValue::new(&mut self.initial_savings)
                                .speed(0.01)
                                .max_decimals(2)
                                .min_decimals(2)
                                .suffix(" €"),
                        );

                        if prev != self.initial_savings {
                            self.save_data();
                        }
                    });

                    self.results_table(ui);
                });
            });
        });
    }
}
