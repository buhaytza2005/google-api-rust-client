pub mod business;
use crate::auth::service_account::ServiceAccountCredentials;

use super::ServiceBase;

static MY_BUSINESS_SERVICE_SCOPE: &str = "https://www.googleapis.com/auth/plus.business.manage";

#[derive(Debug, Clone, Default)]
pub struct BusinessService {
    base: ServiceBase,
    account_id: Option<String>,
    location_id: Option<String>,
}

impl BusinessService {
    pub fn new_with_api_key(api_key: String) -> Self {
        return Self {
            base: ServiceBase::new_with_api_key(api_key),
            ..Default::default()
        };
    }

    pub fn new_with_credentials(service_account_credentials: ServiceAccountCredentials) -> Self {
        return Self {
            base: ServiceBase::new_with_credentials(
                service_account_credentials,
                vec![MY_BUSINESS_SERVICE_SCOPE],
            ),
            ..Default::default()
        };
    }
}
