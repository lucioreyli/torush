use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u16,
    pub name: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}
