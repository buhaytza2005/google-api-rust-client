use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub review_id: String,
    pub name: String,
    pub comment: Option<String>,
    pub reviewer: Option<Reviewer>,
    pub star_rating: Option<Rating>,
    pub review_reply: Option<ReviewReply>,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reviewer {
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Rating {
    #[serde(rename = "STAR_RATING_UNSPECIFIED")]
    StarRatingUnspecified,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReviewReply {
    pub comment: Option<String>,
    pub update_time: Option<DateTime<Utc>>,
}
