use dotenvy::dotenv;
use ris_client::RisClient;

#[tokio::test]
async fn station_information() {
    dotenv().ok();

    let api_key = std::env::var("RIS_API_KEY").expect("RIS_API_KEY not set");
    let client_id = std::env::var("RIS_CLIENT_ID").expect("RIS_CLIENT_ID not set");

    let ris_client = RisClient::new(None, None, None, &client_id, &api_key);

    let result = ris_client.station_information("8000105").await;

    let result = result.expect("Failed to get Station Information");

    let first = result.stations.first();
    assert!(first.is_some(), "No stations in response");

    let first = first.unwrap();

    assert_eq!(first.eva_number, "8000105", "Wrong EVA number");
    assert_eq!(first.names.de.name_long, "Frankfurt(Main)Hbf", "Wrong Name");
}
