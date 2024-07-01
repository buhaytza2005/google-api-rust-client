use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub name: String,
    pub title: String,
    pub store_code: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub phone_numbers: Option<PhoneNumbers>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storefront_address: Option<PostalAddress>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub open_info: Option<OpenInfo>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub regular_hours: Option<BusinessHours>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneNumbers {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub primary_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub additional_phones: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Locations {
    pub locations: Vec<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostalAddress {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub region_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub administrative_area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sublocality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub address_lines: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub organization: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenInfo {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status: Option<OpenForBusiness>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub can_reopen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub opening_date: Option<GoogleDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OpenForBusiness {
    #[serde(rename = "OPEN_FOR_BUSINESS_UNSPECIFIED")]
    OpenForBusinessUnspecified,
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "CLOSED_PERMANENTLY")]
    ClosedPermanently,
    #[serde(rename = "CLOSED_TEMPORARILY")]
    ClosedTemporarily,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoogleDate {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub year: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub month: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub day: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BusinessHours {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub periods: Option<Vec<TimePeriod>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimePeriod {
    pub open_day: DayOfWeek,
    pub open_time: Option<TimeOfDay>,
    pub close_day: DayOfWeek,
    pub close_time: Option<TimeOfDay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DayOfWeek {
    #[serde(rename = "DAY_OF_THE_WEEK_UNSPECIFIED")]
    DayOfTheWeekUnspecified,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeOfDay {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hours: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub minutes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nanos: Option<u64>,
}
