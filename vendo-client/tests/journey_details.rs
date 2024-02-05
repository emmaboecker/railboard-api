use vendo_client::VendoClient;

#[tokio::test]
async fn journey_details() {
    // let http_client =
    //     Client::builder()
    //         .add_root_certificate(Certificate::from_pem(include_bytes!("../../mitm.pem")).unwrap())
    //         .proxy(Proxy::all("http://localhost:8080").unwrap())
    //         .build()
    //         .unwrap();

    let client = VendoClient::default();

    let station_board = client
        .station_board_departures("8000105", None, None)
        .await
        .unwrap();

    let highspeed_trains = station_board
        .departures
        .into_iter()
        .filter(|train| train.product_type == "ICE")
        .collect::<Vec<_>>();

    let first_train = highspeed_trains.first();

    assert!(
        first_train.is_some(),
        "No ICE train found in Frankfurt Hbf, is it night?"
    );

    let first_train = first_train.unwrap();

    let journey_details = client.journey_details(&first_train.id).await.unwrap();

    assert_eq!(journey_details.name, first_train.name, "Names do not equal");
}
