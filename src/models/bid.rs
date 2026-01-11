use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub player_id: String,
    pub amount: u32,
    pub timestamp: DateTime<Utc>,
}
