use dotenvy::dotenv;
use iris_client::IrisClient;
use ris_client::RisClient;

#[tokio::test]
pub async fn journey_search() {
    dotenv().ok();

    let api_key = std::env::var("RIS_API_KEY").expect("RIS_API_KEY not set");
    let client_id = std::env::var("RIS_CLIENT_ID").expect("RIS_CLIENT_ID not set");

    let iris_client = IrisClient::default();
    let ris_client = RisClient::new(None, None, None, &client_id, &api_key);

    let iris = iris_client
        .station_board("8000105", None, None, None)
        .await
        .expect("Failed to get station board from iris");

    let re_train = iris
        .stops
        .iter()
        .find(|stop| stop.train_type == "RE")
        .expect("Didn't find a RE train");

    let ris_re_train = ris_client
        .journey_search(&re_train.train_type, &re_train.train_number)
        .await
        .expect("Failed to get journey search from ris");

    let ris_re_train = ris_re_train
        .journeys
        .first()
        .expect("Didn't find any journeys");

    assert_eq!(
        ris_re_train.origin_schedule.name,
        re_train.route.first().unwrap().name,
        "Origin station name doesn't match"
    );
    assert_eq!(
        ris_re_train.destination_schedule.name,
        re_train.route.last().unwrap().name,
        "Destination station name doesn't match"
    );
}
