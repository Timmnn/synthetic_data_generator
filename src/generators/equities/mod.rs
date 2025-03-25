use super::Generator;

pub struct EquitiesGenerator {}

impl Generator for EquitiesGenerator {
    fn generate(config: crate::config_reader::DatasetConfig) -> () {
        let mut time = config.daterange.from;

        while (time < config.daterange.to) {
            println!("Generating Entry...");
            time = time.checked_add_days(chrono::Days::new(1)).unwrap();
        }
    }
}
