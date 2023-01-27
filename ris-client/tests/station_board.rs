use chrono::{Datelike, Duration, TimeZone, Timelike};
use chrono_tz::Europe::Berlin;
use dotenvy::dotenv;
use ris_client::RisClient;

#[tokio::test]
async fn station_board() {
    dotenv().ok();

    let api_key = std::env::var("RIS_API_KEY").expect("RIS_API_KEY not set");
    let client_id = std::env::var("RIS_CLIENT_ID").expect("RIS_CLIENT_ID not set");

    let ris_client = RisClient::new(None, None, None, &client_id, &api_key);

    let result = ris_client
        .station_board_departures("8000105", None, None)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn station_board_time_range() {
    dotenv().ok();

    let api_key = std::env::var("RIS_API_KEY").expect("RIS_API_KEY not set");
    let client_id = std::env::var("RIS_CLIENT_ID").expect("RIS_CLIENT_ID not set");

    let ris_client = RisClient::new(None, None, None, &client_id, &api_key);

    let current_time = Berlin.from_utc_datetime(&chrono::Utc::now().naive_utc());

    let time_start = current_time - Duration::minutes(20);
    let time_end = current_time + Duration::minutes(60);

    let result = ris_client
        .station_board_departures("8000105", Some(time_start), Some(time_end))
        .await;

    let result = result.expect("Failed to get Station Board");

    println!("{time_start} == {}", result.time_start);

    assert_eq!(
        result.time_start.naive_utc().day(),
        time_start.naive_utc().day(),
        "Wrong start time (day)"
    );
    assert_eq!(
        result.time_start.naive_utc().hour(),
        time_start.naive_utc().hour(),
        "Wrong start time (hour)"
    );
    assert_eq!(
        result.time_start.naive_utc().minute(),
        time_start.naive_utc().minute(),
        "Wrong start time (minute)"
    );
    assert_eq!(
        result.time_end.naive_utc().day(),
        time_end.naive_utc().day(),
        "Wrong end time (day)"
    );
    assert_eq!(
        result.time_end.naive_utc().hour(),
        time_end.naive_utc().hour(),
        "Wrong end time (hour)"
    );
    assert_eq!(
        result.time_end.naive_utc().minute(),
        time_end.naive_utc().minute(),
        "Wrong end time (minute)"
    );
}
