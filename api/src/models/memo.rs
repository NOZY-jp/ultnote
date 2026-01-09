use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MemoType {
    Flash,
    Permanent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memo {
    pub id: Uuid,
    pub content: String,
    #[serde(rename = "type")]
    pub memo_type: MemoType,
    pub from: Option<NaiveDate>,
    pub until: Option<NaiveDate>,
    pub tags: Vec<String>,
    pub date_added: DateTime<Utc>,
    pub access_count: u32,
    pub last_accessed: DateTime<Utc>,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateMemoRequest {
    pub content: String,
    #[serde(rename = "type")]
    pub memo_type: MemoType,
    pub from: Option<NaiveDate>,
    pub until: Option<NaiveDate>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemoRequest {
    pub content: Option<String>,
    #[serde(rename = "type")]
    pub memo_type: Option<MemoType>,
    pub from: Option<NaiveDate>,
    pub until: Option<NaiveDate>,
    pub tags: Option<Vec<String>>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct MemoResponse {
    pub id: Uuid,
    pub content: String,
    #[serde(rename = "type")]
    pub memo_type: MemoType,
    pub from: Option<NaiveDate>,
    pub until: Option<NaiveDate>,
    pub tags: Vec<String>,
    pub date_added: DateTime<Utc>,
}

impl From<Memo> for MemoResponse {
    fn from(memo: Memo) -> Self {
        Self {
            id: memo.id,
            content: memo.content,
            memo_type: memo.memo_type,
            from: memo.from,
            until: memo.until,
            tags: memo.tags,
            date_added: memo.date_added,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    #[serde(default)]
    pub filters: SearchFilters,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    20
}

#[derive(Debug, Default, Deserialize)]
pub struct SearchFilters {
    pub from_gte: Option<NaiveDate>,
    pub until_lte: Option<NaiveDate>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub memo_type: Option<MemoType>,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub id: Uuid,
    pub content: String,
    pub score: f32,
    pub tags: Vec<String>,
    pub from: Option<NaiveDate>,
    pub date_added: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total: usize,
}
