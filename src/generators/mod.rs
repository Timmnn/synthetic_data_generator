pub mod equities;
use crate::config_reader::DatasetConfig;

pub trait Generator {
    fn generate(config: DatasetConfig) -> ();
}
