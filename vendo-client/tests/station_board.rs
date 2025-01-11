use vendo_client::{station_board::VendoTransportType, VendoClient};

#[tokio::test]
async fn station_board_departures_all() {
    let client = VendoClient::default();

    let response = client.station_board_departures("8000105", None, None).await;

    assert!(response.is_ok(), "Return of station board is not ok");

    let response = response.unwrap();

    assert!(
        response.departures.len() > 2,
        "Not more than 2 trains found in Frankfurt Hbf, is it night?"
    );
}

#[tokio::test]
async fn station_board_arrivals_all() {
    let client = VendoClient::default();

    let response = client.station_board_arrivals("8000105", None, None).await;

    assert!(response.is_ok(), "Return of station board is not ok");

    let response = response.unwrap();

    assert!(
        response.arrivals.len() > 2,
        "Not more than 2 trains found in Frankfurt Hbf, is it night?"
    );
}

#[tokio::test]
async fn station_board_departures_filtered() {
    let client = VendoClient::default();

    let response = client
        .station_board_departures(
            "8000105",
            None,
            Some(vec![VendoTransportType::HighspeedTrains]),
        )
        .await;

    assert!(response.is_ok(), "Return of station board is not ok");

    let response = response.unwrap();

    assert!(
        response.departures.len() > 1,
        "Not more than 1 trains found in Frankfurt Hbf, is it night?"
    );

    assert!(
        response
            .departures
            .iter()
            .all(|train| train.product_type == "ICE"),
        "Not High-speed Train found in response"
    )
}

#[tokio::test]
async fn station_board_arrivals_filtered() {
    let client = VendoClient::default();

    let response = client
        .station_board_arrivals(
            "8000105",
            None,
            Some(vec![VendoTransportType::HighspeedTrains]),
        )
        .await;

    assert!(
        response.is_ok(),
        "Return of station board is not ok: {}",
        response.unwrap_err()
    );

    let response = response.unwrap();

    assert!(
        response.arrivals.len() > 1,
        "Not more than 1 trains found in Frankfurt Hbf, is it night?"
    );

    assert!(
        response
            .arrivals
            .iter()
            .all(|train| train.product_type == "ICE"),
        "Not High-speed Train found in response"
    )
}
