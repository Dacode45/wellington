extern crate bots;
#[macro_use]
extern crate serenity;
extern crate d20;
extern crate regex;
#[macro_use]
extern crate log;

use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::*;

use d20::*;
use regex::Regex;

use std::env;

use bots::handler::Handler;
use bots::logging;

pub fn main() {
    logging::setup_logging(0).expect("faild to initialize logging.");

    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN"), Handler)
        .expect("Error creating client");

    // start listening for events by starting a single shard
    info!("Listening!");
    if let Err(why) = client.start() {
        error!("An error occurred while running the client: {:?}", why);
    }
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});
