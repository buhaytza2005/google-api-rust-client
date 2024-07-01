use anyhow::Result;
pub enum EndPoint {
    AccountsEndpoint,
    ///Must contain location name ''
    AdminEndpoint(String),
    LocationsDetailsEndpoint(String, String),
    LocationsEndpoint(String),
    Location(String),
    Reviews(String, String),
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
            EndPoint::Location(location_name) => {
                format!("/v1/{}", location_name)
            }
            EndPoint::LocationsEndpoint(account) => {
                format!(
                    "/v1/accounts/{}/locations?readMask=name,title,storeCode",
                    account
                )
            }
            EndPoint::LocationsDetailsEndpoint(account, read_mask) => {
                format!("/v1/accounts/{}/locations?readMask={}", account, read_mask)
            }
            EndPoint::Reviews(account_id, location_id) => {
                format!("/v4/accounts/{}/{}/reviews", account_id, location_id)
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
            EndPoint::Location(_) => {
                Ok("https://mybusinessbusinessinformation.googleapis.com".into())
            }

            EndPoint::LocationsEndpoint(_) => {
                Ok("https://mybusinessbusinessinformation.googleapis.com".into())
            }
            EndPoint::LocationsDetailsEndpoint(_, _) => {
                Ok("https://mybusinessbusinessinformation.googleapis.com".into())
            }
            EndPoint::Reviews(_, _) => Ok("https://mybusiness.googleapis.com".into()),
        }
    }
}
