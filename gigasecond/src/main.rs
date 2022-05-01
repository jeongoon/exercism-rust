#![allow(unused)]

use time::ext::NumericalDuration;
use time::PrimitiveDateTime as DateTime;
use time::{Date, Month, Result, Time};

const GIGA: i64 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.checked_add(GIGA.seconds()).expect("overflow!")
}

fn main() {
    let date_res = Date::from_calendar_date(2022, Month::May, 1);
    let date = match date_res {
        Ok(date_val) => date_val,
        Err(_) => Date::MIN,
    };

    println!("{}", after(DateTime::new(date, Time::MIDNIGHT,)));
}
