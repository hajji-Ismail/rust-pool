use chrono::{NaiveDate, Weekday, Datelike};

pub fn middle_day(year: i32) -> Option<Weekday> {

    let is_leap = NaiveDate::from_ymd_opt(year, 2, 29).is_some();
    let days = if is_leap { 366 } else { 365 };

    let mid_day = days / 2 + 1;


    NaiveDate::from_yo_opt(year, mid_day).map(|d| d.weekday())
}


