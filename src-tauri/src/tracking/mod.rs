pub mod calculator;
pub mod parser;
pub mod quota;
pub mod watcher;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UsageRecord {
    pub id: Option<i64>,
    pub account_id: Option<String>,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub cache_read_tokens: i64,
    pub cache_write_tokens: i64,
    pub cost_usd: f64,
    pub session_id: Option<String>,
    pub project: Option<String>,
    pub timestamp: Option<String>,
}
