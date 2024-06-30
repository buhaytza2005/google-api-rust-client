pub mod accounts;
pub mod business;
pub mod endpoint;
pub mod locations;

use accounts::{Accounts, Admins, PageAdmins};
use anyhow::{anyhow, Result};
use endpoint::EndPoint;
use locations::{Location, Locations};
use log::info;
use reqwest::{
    header::{self, HeaderValue},
    Response,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct BusinessService {
    access_token: String,
    account_id: Option<String>,
}

pub trait BusinessRequest {
    fn request(
        &mut self,
        endpoint: EndPoint,
    ) -> impl std::future::Future<Output = Result<Response>> + Send;

    fn resource_request(
        &mut self,
        endpoint: EndPoint,
        next_page_token: Option<serde_json::Value>,
    ) -> impl std::future::Future<Output = Result<Response>> + Send;

    fn accounts(&mut self) -> impl std::future::Future<Output = Result<Accounts>> + Send;

    fn locations(
        &mut self,
        account_id: &str,
    ) -> impl std::future::Future<Output = Result<Locations>> + Send;
    fn admin(
        &mut self,
        location: &Location,
    ) -> impl std::future::Future<Output = Result<PageAdmins>> + Send;
}

impl BusinessService {
    pub fn new(access_token: &str) -> Self {
        BusinessService {
            access_token: access_token.to_string(),
            ..Default::default()
        }
    }
}

impl BusinessRequest for BusinessService {
    async fn request(&mut self, endpoint: EndPoint) -> Result<Response> {
        let url = EndPoint::build(endpoint).expect("could not build accounts url");

        let client = reqwest::Client::builder().build()?;
        let res = client
            .get(url)
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token.as_str())).unwrap(),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .expect("Error with request");

        Ok(res)
    }
    async fn resource_request(
        &mut self,
        endpoint: EndPoint,
        next_page_token: Option<serde_json::Value>,
    ) -> Result<Response> {
        let mut url = EndPoint::build(endpoint).expect("could not build accounts url");
        if let Some(token) = next_page_token {
            url.push_str(format!("&pageToken={}", token.as_str().unwrap()).as_str())
        }

        println!("url is {}", url);
        let client = reqwest::Client::builder().build()?;
        let res = client
            .get(url)
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token.as_str())).unwrap(),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .expect("Error with request");

        Ok(res)
    }
    async fn accounts(&mut self) -> Result<Accounts> {
        let response = self.request(EndPoint::AccountsEndpoint).await?;
        let accounts: Accounts = response.json().await?;
        if accounts.accounts.len() == 0 {
            return Err(anyhow!("no accounts, something went wrong!"));
        }
        Ok(accounts)
    }
    async fn locations(&mut self, account_id: &str) -> Result<Locations> {
        let mut locations = Locations::default();
        let mut next_page_token = None;
        loop {
            let response = self
                .resource_request(
                    EndPoint::LocationsEnpoint(account_id.into()),
                    next_page_token.clone(),
                )
                .await?;
            let resp: Value = response.json().await?;
            let val_pages = &resp.get("locations").unwrap().as_array().unwrap().clone();
            let pages: Vec<Location> = val_pages
                .iter()
                .map(|v| serde_json::from_value(v.clone()).unwrap())
                .collect();
            locations.locations.extend(pages);
            next_page_token = resp.get("nextPageToken").cloned();
            if next_page_token.is_none() {
                break;
            };
        }
        info!("Retrieved {} locations", locations.locations.len());
        Ok(locations)
    }
    async fn admin(&mut self, location: &Location) -> Result<PageAdmins> {
        let endpoint = EndPoint::AdminEndpoint(location.name.clone());

        let response = self.request(endpoint).await?;
        let resp: Admins = response.json().await?;
        println!("{:#?}", resp);

        Ok(PageAdmins {
            page_name: location.name.clone(),
            page_title: location.title.clone(),
            store_code: location.store_code.clone(),
            admin_count: resp.admins.len(),
        })
    }
}
