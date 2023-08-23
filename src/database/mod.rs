use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Database {
    config: Config,
}

impl Database {
    pub fn new(config: Config) -> Database {
        Database { config }
    }
}
