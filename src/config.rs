use config_tools::{general_defaults, Config};

pub fn parse_config() -> Config {
    let outcome = Config::load_or_default_outcome("~/.config/cmus-discord-rpc-rs.ini", 
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
        //config.save("~/.config/cmus-discord-rpc-rs.ini").expect("failed to save config");
    }

    let conf = outcome.into_inner();

    conf
}