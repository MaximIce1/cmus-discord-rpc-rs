use core::time;
use std::thread;

use discord_presence::{
    models::{ActivityType, DisplayType},
    Client, Event,
};
fn main() {
    let mut drpc = Client::new(1415831582236872724);
    let msg = "aaaaaaa";

    drpc.on_ready(|_ctx| {
        println!("ready?");
    })
    .persist();

    drpc.start();
    drpc.block_until_event(Event::Ready).unwrap();
    drpc.set_activity(|act| {
        act.state(msg)
            .status_display(DisplayType::State)
            .activity_type(ActivityType::Listening)
            .append_buttons(|button| button.label("aaaaa").url("https://github.com"))
            .append_buttons(|button| button.label("trying to rust").url("https://example.com"))
    })
        .expect("Failed to set activity");

        let mut i = 0;
        loop {
            thread::sleep(time::Duration::from_secs(1));
            i = i + 1;
            println!("{i}");
        }

}