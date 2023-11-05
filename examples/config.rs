use anyhow::Result;
use xdiff_live::config::{RequestConfig, LoadConfig};

fn main() -> Result<()> {
    let content = include_str!("../fixtures/xreq_test.yml");
    let config = RequestConfig::from_yaml(content)?;

    println!("{:#?}", config);
    Ok(())
}