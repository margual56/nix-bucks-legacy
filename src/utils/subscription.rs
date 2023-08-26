use std::hash::Hash;

use chrono::{NaiveDate, Utc};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{times_until, Recurrence, SimpleRecurrence};

#[derive(Clone)]
pub struct TmpSubscription {
    pub name: String,
    pub cost: f32,
    pub recurrence: SimpleRecurrence,
    pub days: u8,
    pub months: u8,
    pub years: u8,
}

impl Default for TmpSubscription {
    fn default() -> Self {
        Self {
            name: String::new(),
            cost: 10.0,
            recurrence: SimpleRecurrence::Month,
            days: 1,
            months: 1,
            years: 1,
        }
    }
}

impl Into<Subscription> for TmpSubscription {
    fn into(self) -> Subscription {
        Subscription::new(
            self.name.to_string(),
            self.cost,
            Recurrence::from_simple_recurrence(self.recurrence, self.days, self.months, self.years),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Subscription {
    uuid: Uuid,
    name: String,
    cost: OrderedFloat<f32>,
    recurrence: Recurrence,
}

impl Subscription {
    pub fn new(name: String, cost: f32, recurrence: Recurrence) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            cost: OrderedFloat(cost),
            recurrence,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cost(&self) -> f32 {
        self.cost.0
    }

    pub fn recurrence(&self) -> Recurrence {
        self.recurrence
    }

    pub fn cost_until(&self, datetime: NaiveDate) -> f32 {
        let times = times_until(self.recurrence, Utc::now().naive_utc().date(), datetime);

        self.cost.0 * times as f32
    }

    pub fn cost_per_year(&self) -> f32 {
        let times = match self.recurrence {
            Recurrence::Day(each_days) => 365 / each_days as u32,
            Recurrence::Month(_, each_months) => 12 / each_months as u32,
            Recurrence::Year(_, _, each_years) => 1 / each_years as u32,
        };

        self.cost.0 * times as f32
    }

    pub fn cost_per_month(&self) -> f32 {
        let times = match self.recurrence {
            Recurrence::Day(each_days) => 30 / each_days as u32,
            Recurrence::Month(_, each_months) => 1 / each_months as u32,
            Recurrence::Year(_, _, each_years) => 1 / (each_years * 12) as u32,
        };

        self.cost.0 * times as f32
    }
}
