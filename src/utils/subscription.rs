use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Recurrence, SimpleRecurrence};

#[derive(Clone)]
pub struct TmpSubscription {
    pub name: String,
    pub cost: f64,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(with = "uuid::serde::compact")]
    uuid: Uuid,
    name: String,
    cost: f64,
    recurrence: Recurrence,
}

impl Subscription {
    pub fn new(name: String, cost: f64, recurrence: Recurrence) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            cost,
            recurrence,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cost(&self) -> f64 {
        self.cost
    }

    pub fn recurrence(&self) -> Recurrence {
        self.recurrence
    }
}
