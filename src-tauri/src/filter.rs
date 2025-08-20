use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterCondition {
    pub field: String,
    pub operator: String, // "eq", "ne", "like", "gt", "lt", "gte", "lte"
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateRange {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterParams {
    pub filters: Vec<FilterCondition>,
    pub date_ranges: Vec<(String, DateRange)>,
    pub age_min: Option<i32>,
    pub age_max: Option<i32>,
}