use anyhow::Result;
use google_api_rust_client_not_official::{
    auth::service_account::ServiceAccountCredentials,
    services::{
        business_service::{BusinessRequest, BusinessService},
        ServiceBase,
    },
};
use std::{path::PathBuf, str::FromStr};

static MY_BUSINESS_SERVICE_SCOPE: &str = "https://www.googleapis.com/auth/plus.business.manage";

#[tokio::main]
async fn main() -> Result<()> {
    let _ = fn_get_locations().await?;

    Ok(())
}

async fn fn_get_locations() -> Result<()> {
    let filepath: PathBuf = PathBuf::from_str("credentials.json").expect("Is this missing?");
    let credentials = ServiceAccountCredentials::from_service_account_file(filepath)
        .expect("this other one missing?");

    let creds = ServiceBase::new_with_credentials(credentials, vec![MY_BUSINESS_SERVICE_SCOPE]);
    let access_token = creds
        .service_account_credentials
        .expect("should have creds")
        .get_access_token()
        .await?;
    let mut business_service = BusinessService::new(&access_token);

    let locations = business_service.get_locations("-").await?;

    println!("got {} locations", locations.locations.len());
    Ok(())
}
