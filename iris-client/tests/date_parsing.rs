use chrono::{Datelike, Timelike};
use iris_client::helpers;

#[tokio::test]
async fn date_parsing() {
    let date_string = "1404011437";

    let parsed_date = helpers::parse_iris_date(date_string);

    assert!(parsed_date.is_some());

    let parsed_date = parsed_date.unwrap();

    println!("{:?}", parsed_date);

    assert_eq!(parsed_date.year(), 2014);
    assert_eq!(parsed_date.month(), 4);
    assert_eq!(parsed_date.day(), 1);
    assert_eq!(parsed_date.hour(), 14);
    assert_eq!(parsed_date.minute(), 37);
}
