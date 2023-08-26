use chrono::{NaiveDate, Utc};
use eframe::egui;

use crate::FixedExpense;

#[derive(Clone)]
pub struct NewPunctualIncomeWindow {
    name: String,
    cost: f32,
    date: NaiveDate,
}

impl Default for NewPunctualIncomeWindow {
    fn default() -> Self {
        Self {
            name: String::new(),
            cost: 0.0,
            date: Utc::now().naive_utc().date(),
        }
    }
}

impl NewPunctualIncomeWindow {
    pub fn show(&mut self, ctx: &egui::Context, show: &mut bool) -> Option<FixedExpense> {
        let mut subs: Option<FixedExpense> = None;
        egui::Window::new("New punctual income")
            .open(show)
            .auto_sized()
            .default_size(&[600.0, 200.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal_centered(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Name (Concept)");

                            ui.text_edit_singleline(&mut self.name);
                        });

                        ui.vertical(|ui| {
                            ui.label("Amount (€)");

                            ui.add(
                                egui::DragValue::new(&mut self.cost)
                                    .speed(0.01)
                                    .max_decimals(2)
                                    .min_decimals(2)
                                    .suffix(" €"),
                            );
                        });

                        ui.vertical(|ui| {
                            ui.label("Date");

                            ui.add(egui_extras::DatePickerButton::new(&mut self.date));
                        });
                    });
                    ui.separator();

                    if ui.button("Add").clicked() {
                        subs = Some(FixedExpense::new(self.name.clone(), self.cost, self.date));
                    }
                });
            });

        return subs;
    }
}
