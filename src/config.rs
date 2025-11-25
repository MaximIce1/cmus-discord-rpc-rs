use std::fs::OpenOptions;

use config_tools::{general_defaults, Config};

pub fn parse_config(some_path: Option<String>) -> Config {
    let path = some_path.unwrap_or(format!("{}{}", std::env::var("HOME").unwrap(), "/.config/cmus-discord-rpc-rs.ini"));
    let outcome = Config::load_or_default_outcome(&path, 
    general_defaults! 
        {
            "line1" => "%status%",
            "line2" => "%artist% - %title%",
            "button1_text" => "",
            "button1_url" => "",
            "button2_text" => "",
            "button2_url" => "",
        }
    );
    if outcome.used_default() {
        println!("Failed to import config, uisng fallback");
        let config = outcome.clone().into_inner();
        let _file = OpenOptions::new().create(true).open(&path);
        config.save(path).expect("failed to save config");
    }
    else {
        println!("Loaded config in {}", path);
    }
    let conf = outcome.into_inner();
    conf
}