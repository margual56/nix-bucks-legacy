use chrono::{NaiveDate, Utc};
use eframe::egui;
use internationalization::t;

use crate::FixedExpense;

#[derive(Clone)]
pub struct NewExpenseWindow {
    name: String,
    cost: f32,
    date: NaiveDate,
}

impl Default for NewExpenseWindow {
    fn default() -> Self {
        Self {
            name: String::new(),
            cost: 0.0,
            date: Utc::now().naive_utc().date(),
        }
    }
}

impl NewExpenseWindow {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        show: &mut bool,
        lang: &str,
    ) -> Option<FixedExpense> {
        let mut subs: Option<FixedExpense> = None;
        egui::Window::new(t!("window.f_expense.title", lang))
            .open(show)
            .auto_sized()
            .default_size([600.0, 200.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal_centered(|ui| {
                        ui.vertical(|ui| {
                            ui.label(t!("window.common.concept", lang));

                            ui.text_edit_singleline(&mut self.name);
                        });

                        ui.vertical(|ui| {
                            ui.label(t!("window.common.cost", lang));

                            ui.add(
                                egui::DragValue::new(&mut self.cost)
                                    .speed(0.01)
                                    .max_decimals(2)
                                    .min_decimals(2)
                                    .suffix(" €"),
                            );
                        });

                        ui.vertical(|ui| {
                            ui.label(t!("window.common.date", "en"));

                            ui.add(egui_extras::DatePickerButton::new(&mut self.date));
                        });
                    });
                    ui.separator();

                    if ui.button(t!("window.common.add", lang)).clicked() {
                        subs = Some(FixedExpense::new(self.name.clone(), self.cost, self.date));
                    }
                });
            });

        subs
    }
}
