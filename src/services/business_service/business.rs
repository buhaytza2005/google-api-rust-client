use super::BusinessService;
use anyhow::{anyhow, bail, Result};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{header::HeaderMap, Client, Url};
use serde::Deserialize;

impl BusinessService {
    /*
    pub async fn accounts(&mut self) -> Result<String> {
        let base_url = Url::parse(&format!(
            "https://mybusinessbusinessinformation.googleapis.com/v1/accounts/{}",
            ""
        ))?;
        let headers = self.base.create_headers().await?;
        let response = Client::new().get(base_url).headers(headers).send().await?;

        let resp: serde_json::Value = response.json().await?;

        let accounts: Vec<_> = resp.get("accounts").unwrap().as_array().unwrap().to_vec();
        if accounts.len() == 0 {
            return Err(anyhow!("no accounts, something went wrong!"));
        }

        let my_account = &accounts[0];
        let acc_id = my_account
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
            .split("/")
            .collect::<Vec<_>>()[1]
            .to_string();
        Ok(acc_id)
    }
    pub async fn account(&mut self, account_id: &str) -> &mut Self {
        self.account_id = Some(account_id.to_string());
        self
    }

    pub async fn locations(&mut self) -> Result<Vec<Location>> {
        let account_id = match self.base.service_account_credentials {
            Some(_) => "-".to_string(),
            None => self.account_id.clone().unwrap(),
        };
        let mut base_url = Url::parse(&format!(
            //locations
            "https://mybusinessbusinessinformation.googleapis.com/v1/accounts/{}/locations?readMask=name,title,storeCode",
            //location
            //"https://mybusinessbusinessinformation.googleapis.com/v1/accounts/{}/locations/9288306414684957101",
            // reviews
            // "https://mybusiness.googleapis.com/v4/accounts/{}/locations/5031657144081502405/reviews",
            // review
            // "https://mybusiness.googleapis.com/v4/accounts/{}/locations/5031657144081502405/reviews/AbFvOqkfvxRZQjTHdDzmv40njTkoJHGEV7HRvzlp8hYx4ZHFs_6gjDKuPDjgzmKmRZBfjVPTFAgtkQ",
           // ""
            account_id
        ))?;
        let headers = self.base.create_headers().await?;

        let mut result: Vec<Location> = vec![];
        loop {
            let response = Client::new()
                .get(base_url.clone())
                .headers(headers.clone())
                .send()
                .await?;
            let resp: serde_json::Value = response.json().await?;
            let val_pages = resp.get("locations").unwrap().as_array().unwrap().clone();
            let pages: Vec<Location> = val_pages
                .iter()
                .map(|v| serde_json::from_value(v.clone()).unwrap())
                .collect();
            result.extend(pages);

            if let Some(next_page_token) = resp["nextPageToken"].as_str() {
                base_url = Url::parse(&format!(
                    "{}&pageToken={}",
                    base_url.clone().as_str(),
                    next_page_token
                ))?;
            } else {
                break;
            }
        }

        //println!("{:#?}", result);

        Ok(result)
    }

    pub async fn location(&mut self, location: Location) -> &mut Self {
        self.location_id = Some(location.name);
        self
    }
    pub async fn get_admins(&mut self, locations: &Vec<Location>) -> Result<Vec<PageAdmins>> {
        let headers = self.base.create_headers().await?;
        let mut results: Vec<PageAdmins> = vec![];
        for location in locations {
            let base_url = Url::parse(&format!(
                "https://mybusinessaccountmanagement.googleapis.com/v1/{}/admins",
                location.name
            ))?;
            let response = Client::new()
                .get(base_url)
                .headers(headers.clone())
                .send()
                .await?;
            let resp: serde_json::Value = response.json().await?;
            let admin_count = resp.get("admins").unwrap().as_array().unwrap().len();
            let resp = PageAdmins {
                store_code: location.store_code.clone(),
                page_name: location.name.clone(),
                page_title: location.title.clone(),
                admin_count,
            };

            results.push(resp);
        }

        Ok(results)
    }
    */
}

#[derive(Debug, Deserialize, Clone)]
pub struct PageAdmins {
    pub page_name: String,
    pub page_title: String,
    #[serde(rename = "storeCode")]
    pub store_code: String,
    pub admin_count: usize,
}

pub struct Admin {
    pub account: String,
    pub admin: String,
    pub name: String,
    pub role: String,
}
