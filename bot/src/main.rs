use std::{collections::hash_map::HashMap, env};

use serenity::{
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        id::{ChannelId, UserId},
        user::OnlineStatus,
    },
    prelude::*,
};
use tokio::runtime::Runtime;


const TALK_CHANNEL_ID: u64 = 489913156165042177;
const BACKEND_URL: &str = "http://backend:80";

fn request_response(message: String) -> Result<String, String> {
    let mut rt = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => return Err(format!("{}", e))
    };
    let client = reqwest::Client::new();

    let http_respnse = match rt.block_on(client.post(&format!("{}/poll", BACKEND_URL)).body(message).send()) {
        Ok(res) => res,
        Err(e) => return Err(format!("{}", e)),
    };

    let response = match rt.block_on(http_respnse.text()) {
        Ok(response) => response,
        Err(e) => return Err(format!("{}", e)),
    };

    Ok(response)
}

struct SelfId;

impl TypeMapKey for SelfId {
    type Value = UserId;
}


struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.set_presence(
            Some(Activity::listening("#talk-to-daddy")),
            OnlineStatus::Online,
        );

        let mut data = ctx.data.write();
        let self_id = data.get_mut::<SelfId>().unwrap();
        *self_id = ready.user.id;
    }

    fn message(&self, ctx: Context, message: Message) {
        let mut data = ctx.data.write();
        let self_id = data.get_mut::<SelfId>().unwrap();

        if message.channel_id != TALK_CHANNEL_ID || u64::from(message.author.id) == u64::from(*self_id) {
            return;
        }

        let response = request_response(message.content);
        let response_message = match response {
            Ok(s) => s,
            Err(e) => format!("Error getting response: {}", e),
        };

        let result =
            ChannelId::from(TALK_CHANNEL_ID).send_message(&ctx, |m| m.content(response_message));
        if let Err(e) = result {
            println!("Error sending response message: {}", e);
        }
    }
}

fn main() {
    println!("Creating client");
    let token = env::var("DISCORD_TOKEN").expect("Expects discord token in DISCORD_TOKEN variable");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<SelfId>(0.into());
    }

    println!("Connecting");
    if let Err(why) = client.start_shards(1) {
        println!("Client error: {:?}", why);
        std::process::exit(1);
    }
}
