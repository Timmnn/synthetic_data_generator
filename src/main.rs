mod config_reader;
mod generators;

use config_reader::read_config;

use clap::{Command, arg};
use generators::Generator;
use std::fs;

fn main() {
    let matches = Command::new("SyntheticDataGenerator")
        .about("Generates Random Data for https://github.com/Timmnn/backtester2")
        .get_matches();

    let contents = fs::read_to_string("example.config.json").unwrap();

    let json: serde_json::Value =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");

    let config = read_config("example.config.json").unwrap();

    fs::create_dir_all("./data/").unwrap();

    for dataset in config.datasets {
        fs::create_dir_all(format!("./data/{}", &dataset.name)).unwrap();

        match dataset.dataset_type.to_lowercase().as_str() {
            "equities" => crate::generators::equities::EquitiesGenerator::generate(dataset),
            "futures" => todo!(),
            _ => panic!("Invalid Type"),
        }
    }
}
