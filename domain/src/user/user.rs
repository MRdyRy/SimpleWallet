use crate::base::base::{AuditMetadata, Auditable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub name: String,
    pub audit: AuditMetadata,
}

impl User {
    pub fn new(id: i32, email: &str, name: &str) -> Self {
        Self {
            id: None,
            email: email.into(),
            name: name.into(),
            audit: AuditMetadata::new(),
        }
    }
}

impl Auditable for User {
    fn audit(&self) -> &AuditMetadata {
        &self.audit
    }
    fn audit_mut(&mut self) -> &mut AuditMetadata {
        &mut self.audit
    }
}
