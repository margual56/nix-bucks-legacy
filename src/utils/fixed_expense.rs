use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FixedExpense {
    name: String,
    cost: f64,

    #[serde(with = "chrono::serde::ts_seconds")]
    date: DateTime<Utc>,
}


