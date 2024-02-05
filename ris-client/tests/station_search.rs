use dotenvy::dotenv;
use ris_client::RisClient;

#[tokio::test]
async fn station_search() {
    dotenv().ok();

    let api_key = std::env::var("RIS_API_KEY").expect("RIS_API_KEY not set");
    let client_id = std::env::var("RIS_CLIENT_ID").expect("RIS_CLIENT_ID not set");

    let ris_client = RisClient::new(None, None, None, &client_id, &api_key);

    let result = ris_client.station_search_by_name("Leipzig", Some(25)).await;

    let result = result.expect("Failed to get station search");

    assert_eq!(result.len(), 25, "limit was exceeded");
    assert_eq!("8010205", result[0].eva_number, "first result should be leipzig hbf (eva)");
    assert_eq!("Leipzig Hbf", result[0].names.de.name_long, "first result should be leipzig hbf (name)");
}
