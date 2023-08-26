use chrono::{NaiveDate, Utc};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct FixedExpense {
    uuid: Uuid,
    pub name: String,
    pub cost: OrderedFloat<f32>,

    pub date: NaiveDate,
}

impl Default for FixedExpense {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: String::new(),
            cost: OrderedFloat(0.0),
            date: Utc::now().naive_utc().date(),
        }
    }
}

impl FixedExpense {
    pub fn new(name: String, cost: f32, date: NaiveDate) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            cost: OrderedFloat(cost),
            date,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cost(&self) -> f32 {
        self.cost.0
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}
