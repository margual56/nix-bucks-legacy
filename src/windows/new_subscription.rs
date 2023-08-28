use eframe::egui;
use internationalization::t;

use crate::{SimpleRecurrence, Subscription, TmpSubscription};

#[derive(Default, Clone)]
pub struct NewSubscriptionWindow {
    tmp_subscription: TmpSubscription,
}

impl NewSubscriptionWindow {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        show: &mut bool,
        lang: &str,
    ) -> Option<Subscription> {
        let mut subs: Option<Subscription> = None;
        egui::Window::new(t!("window.subscription.title", lang))
            .open(show)
            .auto_sized()
            .default_size([600.0, 200.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal_centered(|ui| {
                        ui.vertical(|ui| {
                            ui.label(t!("window.common.concept", lang));

                            ui.text_edit_singleline(&mut self.tmp_subscription.name);
                        });

                        ui.vertical(|ui| {
                            ui.label(t!("window.common.cost", lang));

                            ui.add(
                                egui::DragValue::new(&mut self.tmp_subscription.cost)
                                    .speed(0.01)
                                    .max_decimals(2)
                                    .min_decimals(2)
                                    .suffix(" €"),
                            );
                        });

                        ui.vertical(|ui| {
                            ui.label(t!("window.common.recurrence", lang));

                            egui::ComboBox::from_label(t!("window.common.pick", lang))
                                .selected_text(self.tmp_subscription.recurrence.to_lang_str(&lang))
                                .show_ui(ui, |ui| {
                                    ui.style_mut().wrap = Some(false);
                                    ui.set_min_width(60.0);
                                    ui.selectable_value(
                                        &mut self.tmp_subscription.recurrence,
                                        SimpleRecurrence::Day,
                                        t!("window.common.daily", lang),
                                    );
                                    ui.selectable_value(
                                        &mut self.tmp_subscription.recurrence,
                                        SimpleRecurrence::Month,
                                        t!("window.common.monthly", lang),
                                    );
                                    ui.selectable_value(
                                        &mut self.tmp_subscription.recurrence,
                                        SimpleRecurrence::Year,
                                        t!("window.common.yearly", lang),
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
                                                .prefix(t!("window.common.every", lang))
                                                .suffix(t!("window.common.days", lang)),
                                        );
                                    }
                                    SimpleRecurrence::Month => {
                                        ui.add(
                                            egui::DragValue::new(&mut self.tmp_subscription.days)
                                                .speed(1.0)
                                                .max_decimals(0)
                                                .clamp_range(1..=31)
                                                .prefix(t!("window.common.the", lang))
                                                .suffix(t!("window.common.each_month", lang)),
                                        );
                                        ui.add(
                                            egui::DragValue::new(&mut self.tmp_subscription.months)
                                                .speed(1.0)
                                                .max_decimals(0)
                                                .clamp_range(1..=12)
                                                .prefix(t!("window.common.every", lang))
                                                .suffix(t!("window.common.months", lang)),
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
                                                .prefix(t!("window.common.the", lang)),
                                            );
                                            ui.add(
                                                egui::DragValue::new(
                                                    &mut self.tmp_subscription.months,
                                                )
                                                .speed(1.0)
                                                .max_decimals(0)
                                                .clamp_range(1..=12)
                                                .prefix(t!("window.common.of_month", lang)),
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

                    if ui.button(t!("window.common.add", lang)).clicked() {
                        let sub: Subscription = self.tmp_subscription.clone().into();
                        subs = Some(sub);
                    }
                });
            });

        subs
    }
}
