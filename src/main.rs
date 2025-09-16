mod config;
mod cmus_response;

use core::time;
use std::{collections::HashMap, thread};

use discord_presence::{
    models::{ActivityType, DisplayType},
    Client, Event,
};
fn main() {
    let mut drpc = Client::new(940683540100706345);
    let msg = "%artist%";
    let conf = config::parse_config();
    let mut tags: HashMap<String, String> = HashMap::new();
    let mut artist: String = "".to_string();
    let mut title: String = "".to_string();

    drpc.on_ready(|_ctx| {
        println!("ready?");
    })
    .persist();

    drpc.start();
    drpc.block_until_event(Event::Ready).unwrap();
    
    loop {   
        tags = cmus_response::read_response();
        if (tags.get("artist").unwrap().to_string() == artist) && (tags.get("title").unwrap().to_string() == title) {
            thread::sleep(time::Duration::from_secs(3));
            continue;
        }

        artist = tags.get("artist").unwrap().to_string();
        title = tags.get("title").unwrap().to_string();

        drpc.set_activity(|act| {
            act.state(replace_placeholder(msg, &tags))
                .details(replace_placeholder(conf.get(None, "line2").unwrap().as_str(), &tags))
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