use cached::proc_macro::cached;
use chrono::{Datelike, Days, Months, NaiveDate};

use core::fmt::Display;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[cached]
pub fn days_in_month(m: u8) -> u8 {
    match m {
        1 => 31,
        2 => 28, // TODO: Leap years
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimpleRecurrence {
    Day,
    Month,
    Year,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Recurrence {
    /// Amount of days
    Day(u8),
    /// Day of the month, amount of months
    Month(u8, u8),
    /// Day of the month, month of the year, amount of years
    Year(u8, u8, u8),
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

#[cached]
pub fn times_until(recurrence: Recurrence, from: NaiveDate, to: NaiveDate) -> u32 {
    match recurrence {
        Recurrence::Day(each_days) => {
            (to.signed_duration_since(from).num_days() as f32 / each_days as f32).trunc() as u32
        }
        Recurrence::Month(day, each_months) => {
            // Count the amount of times the day "day" has passed since today to the target date
            let mut start = from.clone().with_day(day as u32).unwrap();

            let mut times: u32 = 0;

            if start < from {
                start = start
                    .checked_add_months(Months::new(each_months as u32))
                    .unwrap();
            } else {
                times += 1;
            }

            let target = to
                .clone()
                .with_day(day as u32)
                .unwrap()
                .checked_add_days(Days::new(1))
                .unwrap();

            while target > start {
                times += 1;

                start = start
                    .checked_add_months(Months::new(each_months as u32))
                    .unwrap();
            }

            times
        }
        Recurrence::Year(day, month, each_years) => {
            // Count the amount of times the day "day" has passed since today to the target date
            let mut start = from
                .clone()
                .with_day(day as u32)
                .unwrap()
                .with_month(month as u32)
                .unwrap();

            let mut times: u32 = 0;

            if start < from {
                start = start
                    .checked_add_months(Months::new(each_years as u32 * 12))
                    .unwrap();
            } else {
                times += 1;
            }

            let target = to
                .clone()
                .with_day(day as u32)
                .unwrap()
                .checked_add_days(Days::new(1))
                .unwrap()
                .with_month(month as u32)
                .unwrap();

            while target > start {
                times += 1;

                start = start
                    .checked_add_months(Months::new(each_years as u32 * 12))
                    .unwrap();
            }

            times
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
