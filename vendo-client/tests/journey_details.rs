use vendo_client::{station_board::StationBoardDeparturesElement, VendoClient};

#[tokio::test]
async fn journey_details() {
    let client = VendoClient::default();

    let station_board = client
        .station_board_departures("8000105", None, None)
        .await
        .unwrap();

    let highspeed_trains = station_board
        .departures
        .into_iter()
        .filter(|train| train.product_type == "ICE")
        .collect::<Vec<StationBoardDeparturesElement>>();

    let first_train = highspeed_trains.first();

    assert!(
        first_train.is_some(),
        "No ICE train found in Frankfurt Hbf, is it night?"
    );

    let first_train = first_train.unwrap();

    let journey_details = client
        .journey_details(first_train.id.clone())
        .await
        .unwrap();

    assert_eq!(journey_details.name, first_train.name, "Names do not equal");
}
