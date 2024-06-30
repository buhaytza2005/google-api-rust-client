#[cfg(test)]
#[test]
fn test_build_endpoint() {
    use google_api_rust_client_not_official::services::business_service::endpoint::EndPoint;

    let url = EndPoint::build(EndPoint::AccountsEndpoint).expect("could not build accounts url");

    assert_eq!(
        url,
        "https://mybusinessbusinessinformation.googleapis.com/v1/accounts".to_string(),
    );
}
#[cfg(test)]
#[test]
fn test_locations_enpoint() {
    use google_api_rust_client_not_official::services::business_service::endpoint::EndPoint;

    let url = EndPoint::build(EndPoint::LocationsEnpoint("-".into()))
        .expect("could not build accounts url");

    assert_eq!(
        url,
        "https://mybusinessbusinessinformation.googleapis.com/v1/accounts".to_string(),
    );
}
