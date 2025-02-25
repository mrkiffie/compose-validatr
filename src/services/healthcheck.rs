use serde::{Deserialize, Serialize};

use crate::compose::{Compose, Validate};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HealthCheck {
    pub test: Option<Test>,
    pub interval: Option<String>,
    pub timeout: Option<String>,
    pub retries: Option<u8>,
    pub start_period: Option<String>,
    pub start_interval: Option<String>,
    pub disable: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Test {
    String(String),
    List(Vec<String>),
}

impl Validate for HealthCheck {
    fn validate(&self, _: &Compose, _: &mut crate::errors::ValidationErrors) {
        ()
    }
}
