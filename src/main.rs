#[macro_use]
extern crate serenity;
extern crate d20;
extern crate regex;

use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::*;

use d20::*;
use regex::Regex;

use std::env;

struct Handler;

impl Handler {
    fn role_response(roll: &str) -> Result<String, &str> {
        println!("rolling: {}", roll);
        let r = roll_dice(roll)?;
        Ok(format!("Well you rolled {}", r.total))
    }
}

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        println!("Got message {:?}", msg);
        let roll_re = Regex::new(r"^roll\s+(.+)$").expect("Regex doesn't compile");

        for cap in roll_re.captures_iter(&msg.content) {
            if let Some(roll) = cap.get(1).map(|m| m.as_str()) {
                if let Ok(response) = Handler::role_response(roll) {
                    if let Err(why) = msg.channel_id.say(response) {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }
    }
}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN"), Handler)
        .expect("Error creating client");

    // start listening for events by starting a single shard
    println!("Listening!");
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});
