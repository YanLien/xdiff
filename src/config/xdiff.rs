use crate::{diff_text, ExtraArgs, RequestProfile};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{is_default, LoadConfig, ValidateConfig};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiffConfig {
    #[serde(flatten)]
    // #[serde(flatten)] 属性宏的作用是将嵌套结构体的字段展平，
    // 即将内部结构体的字段合并到外部结构体中，以便更方便地序列化和反序列化。
    // 这个属性通常用于处理复杂的嵌套结构体，将它们展平为单层的数据结构，
    // 从而简化代码和数据格式。
    pub profiles: HashMap<String, DiffProfile>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    #[serde(skip_serializing_if = "is_default", default)]
    pub res: ResponseProfile,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

impl ResponseProfile {
    pub fn new(skip_headers: Vec<String>, skip_body: Vec<String>) -> Self {
        Self {
            skip_headers,
            skip_body,
        }
    }
}

impl LoadConfig for DiffConfig {}

impl DiffConfig {
    pub fn new(profiles: HashMap<String, DiffProfile>) -> Self {
        Self { 
            profiles
        }
    }

    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        self.profiles.get(name)
    }
}

impl DiffProfile {
    pub fn new(req1: RequestProfile, req2: RequestProfile, res: ResponseProfile) -> Self {
        Self { req1, req2, res }
    }

    pub async fn diff(&self, args: ExtraArgs) -> Result<String> {
        let res1 = self.req1.send(&args).await?;
        let res2 = self.req2.send(&args).await?;

        let text1 = res1.get_text(&self.res).await?;
        let text2 = res2.get_text(&self.res).await?;

        diff_text(&text1, &text2)
    }
}

impl ValidateConfig for DiffConfig {
    fn validate(&self) -> Result<()> {
        for (name, profile) in &self.profiles {
            profile
                .validate()
                .context(format!("failed to validate profile: {}", name))?;
        }
        Ok(())
    }
}

impl ValidateConfig for DiffProfile {
    fn validate(&self) -> Result<()> {
        self.req1.validate().context("req1 failed to validate")?;
        self.req2.validate().context("req2 failed to validate")?;
        Ok(())
    }
}
