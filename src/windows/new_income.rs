use eframe::egui;

use crate::{SimpleRecurrence, Subscription, TmpSubscription};

#[derive(Default, Clone)]
pub struct NewIncomeWindow {
    tmp_subscription: TmpSubscription,
}

impl NewIncomeWindow {
    pub fn show(&mut self, ctx: &egui::Context, show: &mut bool) -> Option<Subscription> {
        let mut subs: Option<Subscription> = None;
        egui::Window::new("New income source")
            .open(show)
            .auto_sized()
            .default_size(&[600.0, 200.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal_centered(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Name (Concept)");

                            ui.text_edit_singleline(&mut self.tmp_subscription.name);
                        });

                        ui.vertical(|ui| {
                            ui.label("Cost (€)");

                            ui.add(
                                egui::DragValue::new(&mut self.tmp_subscription.cost)
                                    .speed(0.01)
                                    .max_decimals(2)
                                    .min_decimals(2)
                                    .suffix(" €"),
                            );
                        });

                        ui.vertical(|ui| {
                            ui.label("Recurrence");

                            egui::ComboBox::from_label("Take your pick")
                                .selected_text(format!("{:?}", self.tmp_subscription.recurrence))
                                .show_ui(ui, |ui| {
                                    ui.style_mut().wrap = Some(false);
                                    ui.set_min_width(60.0);
                                    ui.selectable_value(
                                        &mut self.tmp_subscription.recurrence,
                                        SimpleRecurrence::Day,
                                        "Daily",
                                    );
                                    ui.selectable_value(
                                        &mut self.tmp_subscription.recurrence,
                                        SimpleRecurrence::Month,
                                        "Monthly",
                                    );
                                    ui.selectable_value(
                                        &mut self.tmp_subscription.recurrence,
                                        SimpleRecurrence::Year,
                                        "Yearly",
                                    );
                                });

                            ui.vertical(|ui| {
                                match self.tmp_subscription.recurrence {
                                    SimpleRecurrence::Day => {
                                        ui.add(
                                            egui::DragValue::new(&mut self.tmp_subscription.days)
                                                .speed(1.0)
                                                .max_decimals(0)
                                                .clamp_range(1..=31)
                                                .prefix("Every ")
                                                .suffix(" days"),
                                        );
                                    }
                                    SimpleRecurrence::Month => {
                                        ui.add(
                                            egui::DragValue::new(&mut self.tmp_subscription.days)
                                                .speed(1.0)
                                                .max_decimals(0)
                                                .clamp_range(1..=31)
                                                .prefix("The ")
                                                .suffix(" of each month"),
                                        );
                                        ui.add(
                                            egui::DragValue::new(&mut self.tmp_subscription.months)
                                                .speed(1.0)
                                                .max_decimals(0)
                                                .clamp_range(1..=12)
                                                .prefix("Every ")
                                                .suffix(" months"),
                                        );
                                    }
                                    SimpleRecurrence::Year => {
                                        ui.horizontal(|ui| {
                                            ui.add(
                                                egui::DragValue::new(
                                                    &mut self.tmp_subscription.days,
                                                )
                                                .speed(1.0)
                                                .max_decimals(0)
                                                .clamp_range(1..=31)
                                                .prefix("The "),
                                            );
                                            ui.add(
                                                egui::DragValue::new(
                                                    &mut self.tmp_subscription.months,
                                                )
                                                .speed(1.0)
                                                .max_decimals(0)
                                                .clamp_range(1..=12)
                                                .prefix(" of month "),
                                            );
                                        });
                                        ui.add(
                                            egui::DragValue::new(&mut self.tmp_subscription.years)
                                                .speed(1.0)
                                                .max_decimals(0)
                                                .clamp_range(2023..=2100),
                                        );
                                    }
                                };
                            });
                        });
                    });
                    ui.separator();

                    if ui.button("Add").clicked() {
                        let sub: Subscription = self.tmp_subscription.clone().into();
                        subs = Some(sub);
                    }
                });
            });

        return subs;
    }
}
