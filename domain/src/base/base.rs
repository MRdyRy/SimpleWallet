use chrono::{DateTime, Utc};
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BaseResponse<T> {
    pub trace_id : String,
    pub message: String,
    pub data: Option<T>
}

impl<T> BaseResponse<T> {
    pub fn new(trace_id : String, message : String, data: Option<T>) -> Self {
        Self { trace_id, message, data }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, FromSql, ToSql)]
pub struct AuditMetadata {
    pub created_date: DateTime<Utc>,
    pub updated_date: Option<DateTime<Utc>>,
}

impl AuditMetadata {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            created_date: now,
            updated_date: None
        }
    }
    pub fn touch(&mut self) {
        self.updated_date = Some(Utc::now());
    }
}

/// trait for all auditable entities
pub trait Auditable {
    fn audit(&self) -> &AuditMetadata;
    fn audit_mut(&mut self) -> &mut AuditMetadata;
}

