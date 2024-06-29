use anyhow::Result;
use google_api_rust_client_unoffical::{
    auth::service_account::ServiceAccountCredentials,
    services::business_service::{business::Location, BusinessService},
};
use std::{collections::HashMap, path::PathBuf, str::FromStr, vec};

#[tokio::main]
async fn main() -> Result<()> {
    let filepath: PathBuf = PathBuf::from_str("credentials.json").expect("Is this missing?");
    let credentials = ServiceAccountCredentials::from_service_account_file(filepath)
        .expect("this other one missing?");
    let mut business_service = BusinessService::new_with_credentials(credentials.clone());
    let account_id = business_service.accounts().await?;
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

    let single = &locations[0];

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

    Ok(())
}
