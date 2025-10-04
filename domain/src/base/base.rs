use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BaseResponse {
    pub trace_id : String,
    pub message: String,
    pub data: Option<Value>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

