use core::fmt::Display;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimpleRecurrence {
    Day,
    Month,
    Year,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Recurrence {
    Day(u8),          // Amount of days
    Month(u8, u8),    // Day of the month, amount of months
    Year(u8, u8, u8), // Day of the month, month of the year, amount of years
}

impl Recurrence {
    pub fn from_simple_recurrence(
        value: SimpleRecurrence,
        days: u8,
        months: u8,
        years: u8,
    ) -> Self {
        match value {
            SimpleRecurrence::Day => Self::Day(days),
            SimpleRecurrence::Month => Self::Month(days, months),
            SimpleRecurrence::Year => Self::Year(days, months, years),
        }
    }
}

impl Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Day(days) => write!(f, "Each {} days", days),
            Self::Month(day, months) => write!(f, "Each {} months on day {}", months, day),
            Self::Year(day, month, years) => {
                write!(f, "Each {} years on day {} of month {}", years, day, month)
            }
        }
    }
}

impl Recurrence {
    pub fn to_simple_str(&self) -> &str {
        match self {
            Self::Day(_) => "Day",
            Self::Month(_, _) => "Month",
            Self::Year(_, _, _) => "Year",
        }
    }
}
