mod config;
mod cmus_response;

use core::time;
use std::{collections::HashMap, process::exit, thread};

use discord_presence::{
    models::{ActivityType, DisplayType},
    Client, Event,
};
fn main() {
    if std::env::args().next().unwrap() == "-h" {
        println!("cmus-discord-rpc-rs [FILE] - open specific config file");
        println!("Args:");
        println!("-h - show this text");
        exit(0);
    }
    let mut args = std::env::args();
    args.next();
    let args:Vec<_> = args.collect();
    let mut drpc = Client::new(940683540100706345);
    let conf = config::parse_config(args.get(0).cloned()); //std::env::args().next()
    let mut tags: HashMap<String, String>;
    let mut artist: String = "".to_string();
    let mut title: String = "".to_string();
    let mut msg1: String;
    let mut msg2: String;

    drpc.on_ready(|_ctx| {
        println!("ready?");
    })
    .persist();
    drpc.start();
    drpc.block_until_event(Event::Ready).unwrap();
    
    loop {   
        tags = cmus_response::read_response();
        let _artist = tags.get("artist").cloned();
        let _title = tags.get("title").cloned();
        
        
        
        artist = tags.get("artist").unwrap().to_string();
        title = tags.get("title").unwrap().to_string();

        msg1 = replace_placeholder(conf.get(None, "line1").unwrap().as_str(), &tags);
        msg2 = replace_placeholder(conf.get(None, "line2").unwrap().as_str(), &tags);

        msg1 = msg1.replace("%album%", "");
        msg1 = msg1.replace("%title", "");
        msg1 = msg1.replace("%artist", "");
        msg2 = msg2.replace("%album%", "");
        msg2 = msg2.replace("%title", "");
        msg2 = msg2.replace("%artist", "");


        drpc.set_activity(|act| {
            act .state  (msg1)
                .details(msg2)
                .status_display(DisplayType::State)
                .activity_type(ActivityType::Listening)
                .append_buttons(|button| button.label("aaaaa").url("https://github.com"))
                .append_buttons(|button| button.label("trying to rust").url("https://example.com"))
        })
            .expect("Failed to set activity");



        thread::sleep(time::Duration::from_secs(1));
    }
}

fn replace_placeholder(input: &str, tags: &HashMap<String, String>) -> String {
    let mut output = input.to_string();
    for (key, value) in tags {
        output = output.replace(&format!("%{}%", key), value)
    }
    output
}