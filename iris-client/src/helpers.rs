use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::{Europe::Berlin, Tz};

pub fn parse_iris_date(date_string: &str) -> Option<DateTime<Tz>> {
    let date = NaiveDateTime::parse_from_str(date_string, "%y%m%d%H%M").ok();

    if date.is_none() {
        return None;
    }

    let date = date.unwrap();

    let date = Berlin.from_local_datetime(&date).single();

    date
}
