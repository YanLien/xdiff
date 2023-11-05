use std::{fmt::Debug, collections::HashMap};
use serde::{Serialize, Deserialize};
use crate::RequestProfile;
use anyhow::{Result, Context};

use super::{ValidateConfig, LoadConfig};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, RequestProfile>,
}

impl LoadConfig for RequestConfig {}

impl RequestConfig {
    pub fn new(profiles: HashMap<String, RequestProfile>) -> Self {
        Self {profiles}
    }

    pub fn get_profile(&self, name: &str) -> Option<&RequestProfile> {
        self.profiles.get(name)
    }
}

impl ValidateConfig for RequestConfig {
    fn validate(&self) -> Result<()> {
        for (name, profile) in &self.profiles {
            profile
                .validate()
                .context(format!("failed to validate profile: {}", name))?;
        }
        Ok(())
    }
}


