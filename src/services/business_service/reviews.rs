use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Reviewer {
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

pub trait Stringer {
    fn to_str(&self) -> String;
}

impl Stringer for Rating {
    fn to_str(&self) -> String {
        match self {
            Rating::StarRatingUnspecified => "STAR_RATING_UNSPECIFIED".to_string(),
            Rating::One => "ONE".to_string(),
            Rating::Two => "TWO".to_string(),
            Rating::Three => "THREE".to_string(),
            Rating::Four => "FOUR".to_string(),
            Rating::Five => "FIVE".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stopper {
    pub review_id: String,
    pub total_reviews: i32,
    pub last_update: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReviewReply {
    pub comment: Option<String>,
    pub update_time: Option<DateTime<Utc>>,
}
