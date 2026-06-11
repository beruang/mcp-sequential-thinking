use chrono::{DateTime, Duration, Utc};

pub fn is_expired(updated_at: DateTime<Utc>, ttl_seconds: u64) -> bool {
    let deadline = updated_at + Duration::seconds(ttl_seconds as i64);
    Utc::now() > deadline
}
