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
    CreationContext,
};
use egui_extras::{Column, TableBuilder};
use internationalization::t;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    FixedExpense, NewExpenseWindow, NewIncomeWindow, NewPunctualIncomeWindow,
    NewSubscriptionWindow, Subscription,
};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "margual56";
const APPLICATION: &str = "NixBucks";

#[derive(Serialize, Deserialize, Clone)]
pub struct App {
    initial_savings: f32,
    subscriptions: HashMap<Uuid, Subscription>,
    incomes: HashMap<Uuid, Subscription>,
    fixed_expenses: HashMap<Uuid, FixedExpense>,
    p_incomes: HashMap<Uuid, FixedExpense>,
    dismissed_ad: bool,
    lang: String,

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
        if let Some(dir) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
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
                        dismissed_ad: false,
                        lang: String::from("en"),

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
                dismissed_ad: false,
                lang: String::from("en"),

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
    /// Creates a new app instance with custom styles.
    /// This is needed because we need to redefine text styles to use bigger fonts
    /// Otherwise, it just returns `Self::default()`
    pub fn new(cc: &CreationContext) -> Self {
        // Get current context style
        let mut style = (*cc.egui_ctx.style()).clone();

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
        cc.egui_ctx.set_style(style);

        Self::default()
    }

    /// Saves the data to the config file. It uses the [`directories::ProjectDirs`](https://docs.rs/directories/latest/directories/struct.ProjectDirs.html) struct to find the config folder with:
    /// - QUALIFIER: "com"
    /// - ORGANIZATION: "margual56"
    /// - APPLICATION: "NixBucks"
    ///
    /// And appends "config.json" to the path. Then, it overwrites the file with the serialized data.
    fn save_data(&self) {
        if let Some(dir) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            if !dir.config_dir().exists() {
                std::fs::create_dir_all(dir.config_dir()).unwrap();
            }

            let path = File::create(dir.config_dir().join("config.json")).unwrap();

            serde_json::to_writer_pretty(path, self).unwrap();
        }
    }

    /// Updates the app by removing the expired subscriptions and incomes and adding the amounts to the "initial amount".
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

    /// Removes an expense.
    /// # Arguments
    /// - `uuid`: The UUID of the expense to remove.
    pub fn remove_expense(&mut self, uuid: Uuid) {
        self.fixed_expenses.remove(&uuid);
    }

    /// Remove a punctual income.
    /// # Arguments
    /// - `uuid`: The UUID of the income to remove.
    pub fn remove_punctual_income(&mut self, uuid: &Uuid) {
        self.p_incomes.remove(uuid);
    }

    /// Returns the total cost of all subscriptions in a whole year.
    #[allow(dead_code)]
    fn yearly_costs(&self) -> f32 {
        let mut amount = 0.0;

        for subscription in self.subscriptions.values() {
            amount += subscription.cost_per_year();
        }

        amount
    }

    /// Returns the total cost of all subscriptions in a month.
    fn monthly_costs(&self) -> f32 {
        let mut amount = 0.0;

        for subscription in self.subscriptions.values() {
            amount += subscription.cost_per_month();
        }

        amount
    }

    /// Returns the balance at the end of each month (all income streams - all subscriptions).
    fn monthly_balance(&self) -> f32 {
        let mut amount = 0.0;

        for income in self.incomes.values() {
            amount += income.cost_per_month();
        }

        for subscription in self.subscriptions.values() {
            amount -= subscription.cost();
        }

        amount
    }

    /// Just draws the pop-up windows.
    fn draw_windows(&mut self, ctx: &egui::Context) {
        if let Some(win) = self.new_subscription_window.as_mut() {
            let mut show = true;

            if let Some(result) = win.show(ctx, &mut show, &self.lang) {
                self.subscriptions.insert(result.uuid(), result);

                self.save_data();

                self.new_subscription_window = None;
            } else if !show {
                self.new_subscription_window = None;
            }
        }
        if let Some(win) = self.new_expense_window.as_mut() {
            let mut show = true;

            if let Some(result) = win.show(ctx, &mut show, &self.lang) {
                self.fixed_expenses.insert(result.uuid(), result);

                self.save_data();

                self.new_expense_window = None;
            } else if !show {
                self.new_expense_window = None;
            }
        }

        if let Some(win) = self.new_income_window.as_mut() {
            let mut show = true;

            if let Some(result) = win.show(ctx, &mut show, &self.lang) {
                self.incomes.insert(result.uuid(), result);

                self.save_data();

                self.new_income_window = None;
            } else if !show {
                self.new_income_window = None;
            }
        }

        if let Some(win) = self.new_p_income_window.as_mut() {
            let mut show = true;

            if let Some(result) = win.show(ctx, &mut show, &self.lang) {
                self.p_incomes.insert(result.uuid(), result);

                self.save_data();

                self.new_p_income_window = None;
            } else if !show {
                self.new_p_income_window = None;
            }
        }
    }

    /// Draws the subscriptions table.
    /// # Arguments
    /// - `ui`: The [`egui::Ui`](https://docs.rs/egui/0.12.2/egui/struct.Ui.html) to draw the table into.
    /// # Returns
    /// - `InnerResponse<()>`: The response of the table.
    fn subscriptions_table(&mut self, ui: &mut egui::Ui) -> InnerResponse<()> {
        ui.vertical_centered_justified(|ui| {
            ui.heading(t!("app.title.subscriptions", self.lang));
            ui.separator();
            ui.push_id("subscriptions", |ui| {
                egui::ScrollArea::both()
                    .id_source("Subscriptions scroll area")
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
                                    ui.heading(t!("app.table.title.concept", self.lang));
                                });
                                header.col(|ui| {
                                    ui.heading(t!("app.table.title.cost", self.lang));
                                });
                                header.col(|ui| {
                                    ui.heading(t!("app.table.title.recurrence", self.lang));
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
                                                subscription.recurrence().to_lang_str(&self.lang),
                                            ));
                                        });
                                        row.col(|ui| {
                                            if ui
                                                .button(t!("app.button.delete", self.lang))
                                                .clicked()
                                            {
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

            if ui
                .button(t!("app.button.new.subscription", self.lang))
                .clicked()
            {
                self.new_subscription_window = Some(NewSubscriptionWindow::default());
            }
        })
    }

    /// Draws the expenses table.
    /// # Arguments
    /// - `ui`: The [`egui::Ui`](https://docs.rs/egui/0.12.2/egui/struct.Ui.html) to draw the table into.
    /// # Returns
    /// - `InnerResponse<()>`: The response of the table.
    fn expenses_table(&mut self, ui: &mut egui::Ui) -> InnerResponse<()> {
        ui.vertical_centered_justified(|ui| {
            ui.heading(t!("app.title.fixed_expenses", self.lang));
            ui.separator();
            egui::ScrollArea::both()
                .id_source("Expenses scroll area")
                .auto_shrink([true, true])
                .max_height(ui.available_height() - 35.0)
                .show(ui, |ui| {
                    ui.push_id("expenses", |ui| {
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
                                    ui.heading(t!("app.table.title.concept", self.lang));
                                });
                                header.col(|ui| {
                                    ui.heading(t!("app.table.title.cost", self.lang));
                                });
                                header.col(|ui| {
                                    ui.heading(t!("app.table.title.date", self.lang));
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
                                            if ui
                                                .button(t!("app.button.delete", self.lang))
                                                .clicked()
                                            {
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

            if ui
                .button(t!("app.button.new.fixed_expense", self.lang))
                .clicked()
            {
                self.new_expense_window = Some(NewExpenseWindow::default());
            }
        })
    }

    /// Draws the results table, with the stats of the money.
    /// # Arguments
    /// - `ui`: The [`egui::Ui`](https://docs.rs/egui/0.12.2/egui/struct.Ui.html) to draw the table into.
    /// # Returns
    /// - `InnerResponse<()>`: The response of the table.
    fn results_table(&self, ui: &mut egui::Ui) -> InnerResponse<()> {
        ui.vertical(|ui| {
            ui.add_space(20.0);
            ui.vertical_centered(|ui| {
                ui.heading(t!("app.title.stats", self.lang));
            });
            ui.spacing();

            ui.horizontal(|ui| {
                ui.push_id("results", |ui| {
                    TableBuilder::new(ui)
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
                                    ui.label(RichText::new(t!("stats.avg_cost_month", self.lang)));
                                });
                                row.col(|ui| {
                                    ui.label(
                                        RichText::new(format!("{:+.2}€", self.monthly_costs()))
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
                                    ui.label(RichText::new(t!(
                                        "stats.total_cost_til_eoy",
                                        self.lang
                                    )));
                                });
                                row.col(|ui| {
                                    ui.label(
                                        RichText::new(format!(
                                            "{:+.2}€",
                                            cost_to_year_end(
                                                self.subscriptions.clone().into_values().collect(),
                                                self.fixed_expenses.clone().into_values().collect()
                                            )
                                        ))
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
                                    ui.label(RichText::new(t!(
                                        "stats.total_income_til_eoy",
                                        self.lang
                                    )));
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
                                        RichText::new(t!("stats.balance_eoy", self.lang)).strong(),
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
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.spacing();
                                });
                                row.col(|ui| {
                                    ui.label(
                                        RichText::new(t!("stats.balance_eom", self.lang)).strong(),
                                    );
                                });

                                row.col(|ui| {
                                    let balance = self.monthly_balance();

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

    /// Draws the income table.
    /// # Arguments
    /// - `ui`: The [`egui::Ui`](https://docs.rs/egui/0.12.2/egui/struct.Ui.html) to draw the table into.
    /// # Returns
    /// - `InnerResponse<()>`: The response of the table.
    fn income_table(&mut self, ui: &mut egui::Ui) -> InnerResponse<()> {
        ui.vertical_centered_justified(|ui| {
            ui.heading(t!("app.title.income_streams", self.lang));
            ui.separator();
            egui::ScrollArea::both()
                .id_source("Subscriptions1 scroll area")
                .auto_shrink([true, true])
                .max_height(ui.available_height() - 35.0)
                .show(ui, |ui| {
                    ui.push_id("incomes", |ui| {
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
                                    ui.heading(t!("app.table.title.concept", self.lang));
                                });
                                header.col(|ui| {
                                    ui.heading(t!("app.table.title.cost", self.lang));
                                });
                                header.col(|ui| {
                                    ui.heading(t!("app.table.title.recurrence", self.lang));
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
                                                subscription.recurrence().to_lang_str(&self.lang),
                                            ));
                                        });
                                        row.col(|ui| {
                                            if ui
                                                .button(t!("app.button.delete", self.lang))
                                                .clicked()
                                            {
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

            if ui
                .button(t!("app.button.new.income_stream", self.lang))
                .clicked()
            {
                self.new_income_window = Some(NewIncomeWindow::default());
            }
        })
    }

    /// Draws the punctual income table.
    /// # Arguments
    /// - `ui`: The [`egui::Ui`](https://docs.rs/egui/0.12.2/egui/struct.Ui.html) to draw the table into.
    /// # Returns
    /// - `InnerResponse<()>`: The response of the table.
    fn punctual_income_table(&mut self, ui: &mut egui::Ui) -> InnerResponse<()> {
        ui.vertical_centered_justified(|ui| {
            ui.heading(t!("app.title.punctual_income", self.lang));
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
                                    ui.heading(t!("app.table.title.concept", self.lang));
                                });
                                header.col(|ui| {
                                    ui.heading(t!("app.table.title.cost", self.lang));
                                });
                                header.col(|ui| {
                                    ui.heading(t!("app.table.title.date", self.lang));
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
                                            if ui
                                                .button(t!("app.button.delete", self.lang))
                                                .clicked()
                                            {
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

            if ui
                .button(t!("app.button.new_punctual_income", self.lang))
                .clicked()
            {
                self.new_p_income_window = Some(NewPunctualIncomeWindow::default());
            }
        })
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.draw_windows(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.menu_button(t!("app.language", self.lang), |ui| {
                let lang = self.lang.clone();

                ui.radio_value(&mut self.lang, String::from("en"), t!("english", lang));
                ui.radio_value(&mut self.lang, String::from("es"), t!("spanish", lang));

                if lang != self.lang {
                    self.save_data();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.collapsing(
                        RichText::new(t!("app.collapsing.expenses", self.lang)).heading(),
                        |ui| {
                            ui.horizontal(|ui| {
                                egui::ScrollArea::horizontal().show(ui, |ui| {
                                    TableBuilder::new(ui)
                                        .auto_shrink([false, true])
                                        .vscroll(false)
                                        .column(
                                            Column::auto()
                                                .at_least(450.0)
                                                .clip(true)
                                                .resizable(false),
                                        )
                                        .column(Column::auto().at_least(25.0).resizable(false))
                                        .column(
                                            Column::auto()
                                                .at_least(450.0)
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
                        },
                    );

                    ui.add_space(25.0);

                    ui.collapsing(
                        RichText::new(t!("app.collapsing.income", self.lang)).heading(),
                        |ui| {
                            ui.horizontal(|ui| {
                                egui::ScrollArea::horizontal().show(ui, |ui| {
                                    TableBuilder::new(ui)
                                        .vscroll(false)
                                        .auto_shrink([false, true])
                                        .column(
                                            Column::auto()
                                                .at_least(450.0)
                                                .clip(true)
                                                .resizable(false),
                                        )
                                        .column(Column::auto().at_least(25.0))
                                        .column(
                                            Column::auto()
                                                .at_least(450.0)
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
                        },
                    );

                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);

                    ui.horizontal(|ui| {
                        ui.heading(t!("app.title.initial_savings", self.lang));

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
