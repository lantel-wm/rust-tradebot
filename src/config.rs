pub struct Config {
    bridge: String,
    use_margin: bool,
    scout_multiplier: f64,
    scout_margin: f64,
    scout_sleep_time: u32,
    hourToKeepScoutHistory: u32,
    tld: String,
    strategy: String,
    sell_timeout: u32,
    buy_timeout: u32,
}

impl Config {
    fn new(cfg_file_path: String) -> Config {

        Config {
            bridge: String::from(""),
            use_margin: false,
            scout_multiplier: 0.0,
            scout_margin: 0.0,
            scout_sleep_time: 0,
            hourToKeepScoutHistory: 0,
            tld: String::from(""),
            strategy: String::from(""),
            sell_timeout: 0,
            buy_timeout: 0,
        }
    }
}

mod yml {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    fn load(file_path: String) -> HashMap {
        let yml_content = serde_yaml::from_str(file_path);
    }
}