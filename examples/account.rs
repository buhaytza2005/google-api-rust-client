use std::{path::PathBuf, str::FromStr};

use google_api_rust_client_not_official::{
    auth::service_account::ServiceAccountCredentials,
    services::{
        business_service::{BusinessRequest, BusinessService},
        ServiceBase,
    },
};

static MY_BUSINESS_SERVICE_SCOPE: &str = "https://www.googleapis.com/auth/plus.business.manage";

#[tokio::main]
async fn main() {
    let access_token = get_token().await;
    let mut b_serv = BusinessService::new(&access_token);

    let r = b_serv
        .account("-")
        .await
        .expect("Should get account details");

    let resp: serde_json::Value = r.json().await.expect("JSON");
    println!("{:#?}", resp);
}

async fn get_token() -> String {
    let filepath: PathBuf = PathBuf::from_str("credentials.json").expect("Is this missing?");
    let credentials = ServiceAccountCredentials::from_service_account_file(filepath)
        .expect("this other one missing?");

    let creds = ServiceBase::new_with_credentials(credentials, vec![MY_BUSINESS_SERVICE_SCOPE]);
    let access_token = creds
        .service_account_credentials
        .expect("should have creds")
        .get_access_token()
        .await
        .expect("Should have token");

    access_token
}
