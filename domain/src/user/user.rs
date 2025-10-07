use postgres_types::{FromSql, ToSql};
use crate::base::base::{AuditMetadata, Auditable};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "user_status")]
pub enum UserStatus {
    #[postgres(name = "Active")]
    Active,
    #[postgres(name = "Inactive")]
    Inactive,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub name: String,
    pub audit: AuditMetadata,
}

impl User {
    pub fn new(email: &str, name: &str) -> Self {
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
