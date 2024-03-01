use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub date: NaiveDate,
    pub title: String,
    pub description: String,
}
