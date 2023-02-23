use chrono::{TimeZone, Utc};
use chrono_tz::Europe::Berlin;
use dotenvy::dotenv;
use ris_client::RisClient;

#[tokio::test]
async fn journey_details() {
    dotenv().ok();

    let api_key = std::env::var("RIS_API_KEY").expect("RIS_API_KEY not set");
    let client_id = std::env::var("RIS_CLIENT_ID").expect("RIS_CLIENT_ID not set");

    let ris_client = RisClient::new(None, None, None, &client_id, &api_key);

    let current = Berlin.from_utc_datetime(&Utc::now().naive_utc());

    let station_board = ris_client
        .station_board_departures(
            "8000105",
            Some(current),
            Some(current + chrono::Duration::hours(1)),
        )
        .await;

    let station_board = station_board.expect("Failed to get station board");

    let first = station_board
        .items
        .into_iter()
        .find(|item| item.train.category == "ICE")
        .expect("No ICE in departure board of Frankfurt. Is it night?");

    let journey_details = ris_client
        .journey_details(&first.train.journey_id)
        .await
        .unwrap_or_else(|e| {
            panic!(
                "Failed to get journey details for train: {:#?} \n Error: {:#?}",
                first.train, e
            )
        });

    let event = journey_details
        .events
        .into_iter()
        .find(|train| train.station.eva_number == "8000105")
        .expect("Failed to get right station");

    assert_eq!(first.train.category, event.transport.category);

    assert_eq!(
        first.train.line_name,
        event
            .transport
            .line
            .unwrap_or(event.transport.number.to_string())
    );
}
