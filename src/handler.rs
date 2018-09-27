use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::*;

use d20::*;
use regex::Regex;

pub struct Handler;

impl Handler {
    fn role_response(roll: &str) -> Result<String, &str> {
        info!("rolling: {}", roll);
        let r = roll_dice(roll)?;
        Ok(format!("Well you rolled {}", r.total))
    }
}

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        info!("Got message {:?}", msg);
        let roll_re = Regex::new(r"^roll\s+(.+)$").expect("Regex doesn't compile");

        for cap in roll_re.captures_iter(&msg.content.to_lowercase()) {
            if let Some(roll) = cap.get(1).map(|m| m.as_str()) {
                match Handler::role_response(roll) {
                    Ok(response) => {
                        if let Err(why) = msg.channel_id.say(response) {
                            error!("Error sending message: {:?}", why);
                        }
                    }
                    Err(_) => {
                        if let Err(why) = msg.channel_id.say("Well I never.") {
                            error!("Error sending message: {:?}", why);
                        }
                    }
                }
            }
        }

        let d_re = Regex::new(r"play despacito\s*2").expect("Regex doesn't compile");
        if d_re.is_match(&msg.content.to_lowercase()) {
            if let Err(why) = msg.channel_id.send_message(|m| {
                m.content("Well it's finally out").embed(|e| {
                    e.title("Despacito 2")
                        .url("https://www.youtube.com/watch?v=W3GrSMYbkBE")
                        .thumbnail("https://i.chzbgr.com/original/5809413/h184437FB/")
                        .image("https://www.billboard.com/files/media/02-Luis-Fonsi-Despacito-ft.-Daddy-Yankee-screenshot-2017-billboard-1548.jpg")
                        .description("Alexa plays despacito 2")
                        .fields(vec![
                            ("This is so sad", "comming soon", true),
                        ]).field("Are you not entertained", "Despacito means slowly", false)
                        .footer(|f| f.text("WELLS FOR THE WELL GOD!")
                        .icon_url("https://vignette.wikia.nocookie.net/overwatch/images/6/65/Ilios_controlpoint_well.png/revision/latest?cb=20160904213011")
                        )
                })
            }) {
                error!("Error sending message: {:?}", why);
            }
        }
    }
}
