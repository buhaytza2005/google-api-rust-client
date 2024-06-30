use anyhow::Result;
use google_api_rust_client_not_official::{
    auth::service_account::ServiceAccountCredentials,
    services::{
        business_service::{BusinessRequest, BusinessService},
        ServiceBase,
    },
};
use std::{path::PathBuf, str::FromStr, time::Instant};
static MY_BUSINESS_SERVICE_SCOPE: &str = "https://www.googleapis.com/auth/plus.business.manage";

#[tokio::main]
async fn main() -> Result<()> {
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
    let accounts = business_service.accounts().await?;

    let my_account = &accounts.accounts[0];
    let acc_id = my_account.name.split("/").collect::<Vec<_>>()[1].to_string();

    println!("{}", acc_id);
    println!("{:#?}", accounts);
    let locations = business_service.locations("-").await?;

    for location in &locations.locations {
        println!("{:#?}", location);
        //let _ = business_service.admin(location).await?;
    }

    /*
        let _account_id = business_service.accounts().await?;
        //let business_service = BusinessService::new_with_credentials(credentials.clone());
        let locations = business_service.locations().await?;
        //println!("response: {}", serde_json::to_string(&response)?);
        //
        let mut page_counts: HashMap<String, Vec<Location>> = HashMap::new();

        for location in &locations {
            page_counts
                .entry(location.store_code.clone())
                .or_insert(Vec::new())
                .push(location.clone());
        }

        let admins = business_service.get_admins(&locations).await?;

        let more_than_one_admin = admins
            .iter()
            .filter(|pa| pa.admin_count > 1)
            .collect::<Vec<_>>();

        println!("{:#?}", more_than_one_admin);

        /*
            let duplicates = page_counts
                .iter()
                .filter(|p| p.1.len() > 1)
                .collect::<Vec<_>>();
            println!("{:#?}", duplicates);
        */
    */
    Ok(())
}
