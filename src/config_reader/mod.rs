use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct DatasetConfig {
    pub name: String,
    pub dataset_type: String,
    pub period: String,
    pub daterange: DatasetDateRangeConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatasetDateRangeConfig {
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub datasets: Vec<DatasetConfig>,
}

pub fn read_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string("example.config.json")?;
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}
