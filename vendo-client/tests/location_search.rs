use vendo_client::VendoClient;

#[tokio::test]
async fn location_search() {
    let client = VendoClient::default();

    let response = client
        .location_search(String::from("Frankfurt"), None)
        .await;

    assert!(response.is_ok(), "Return of location search is not ok");

    let response = response.unwrap();

    assert!(
        response.len() > 1,
        "Not more than 1 locations found in Frankfurt"
    );

    assert!(
        response
            .iter()
            .any(|result| result.name == "Frankfurt(Main)Hbf"),
        "Frankfurt(Main)Hbf was not found in results"
    );
}
