use anyhow::{anyhow, Result};
pub enum EndPoint {
    AccountsEndpoint,
    ///Must contain location name ''
    AdminEndpoint(String),
    LocationsEnpoint(String),
}

pub enum ResourceType {
    Admin,
    Location,
}

impl EndPoint {
    ///returns the endpoint
    pub fn as_str(&self) -> String {
        match self {
            EndPoint::AccountsEndpoint => "/v1/accounts".to_string(),
            EndPoint::AdminEndpoint(location_name) => {
                format!("/v1/{}/admins", location_name)
            }
            EndPoint::LocationsEnpoint(account) => {
                format!(
                    "/v1/accounts/{}/locations?readMask=name,title,storeCode",
                    account
                )
            }
        }
    }
    ///builds url for endpoint, including the main url and endpoint
    pub fn build(endpoint: EndPoint) -> Result<String> {
        let mut base_url = EndPoint::get_base_url(&endpoint)?;
        base_url.push_str(&endpoint.as_str());

        Ok(base_url)
    }
    ///pattern matching on endpoint types to get the main url
    pub fn get_base_url(endpoint: &EndPoint) -> Result<String> {
        match endpoint {
            EndPoint::AccountsEndpoint => {
                Ok("https://mybusinessbusinessinformation.googleapis.com".into())
            }
            EndPoint::AdminEndpoint(_) => {
                Ok("https://mybusinessaccountmanagement.googleapis.com".into())
            }
            EndPoint::LocationsEnpoint(_) => {
                Ok("https://mybusinessbusinessinformation.googleapis.com".into())
            }
            _ => Err(anyhow!("Could not find base url for this endpoint")),
        }
    }
}
