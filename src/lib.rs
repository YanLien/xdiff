pub mod cli;
pub mod config;
mod utils;

pub use config::{
    get_body_text, get_headers_text, get_status_text, is_default, DiffConfig, DiffProfile,
    LoadConfig, RequestConfig, RequestProfile, ResponseProfile, ValidateConfig,
};
pub use utils::{diff_text, highlight_text, process_error_output};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExtraArgs {
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
}

impl ExtraArgs {
    pub fn new_with_headers(headers: Vec<(String, String)>) -> Self {
        Self {
            headers,
            ..Default::default()
        }
    }

    pub fn new_with_query(query: Vec<(String, String)>) -> Self {
        Self {
            query,
            ..Default::default()
        }
    }

    pub fn new_with_body(body: Vec<(String, String)>) -> Self {
        Self {
            body,
            ..Default::default()
        }
    }
}