#[tokio::test]
async fn station_board() {
    let iris_client = iris_client::IrisClient::default();

    let response = iris_client
        .realtime_station_board("8000105".to_string())
        .await;

    assert!(response.is_ok(), "Response is not ok: {:?}", response);

    let response = response.unwrap();

    println!("{:#?}", response)
}
