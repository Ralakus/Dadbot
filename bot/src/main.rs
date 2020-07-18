use std::env;

use futures::executor::block_on;

use serenity::{
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        id::ChannelId,
        user::OnlineStatus,
    },
    prelude::*,
};

const TALK_CHANNEL_ID: u64 = 489913156165042177;
const BACKEND_URL: &str = "http://backend:8080";

async fn request_response(message: String) -> Result<String, String> {
    let client = reqwest::Client::new();

    let http_respnse = match client.post(&format!("{}/poll", BACKEND_URL)).body(message).send().await {
        Ok(res) => res,
        Err(e) => return Err(format!("{}", e)),
    };

    let response = match http_respnse.text().await {
        Ok(response) => response,
        Err(e) => return Err(format!("{}", e)),
    };

    Ok(response)
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.set_presence(
            Some(Activity::listening("#talk-to-daddy")),
            OnlineStatus::Online,
        )
    }

    fn message(&self, ctx: Context, message: Message) {
        if message.channel_id != TALK_CHANNEL_ID {
            return;
        }

        let response = block_on(request_response(message.content));
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

#[tokio::main]
async fn main() {
    println!("Creating client");
    let token = env::var("DISCORD_TOKEN").expect("Expects discord token in DISCORD_TOKEN variable");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    println!("Connecting");
    if let Err(why) = client.start_shards(1) {
        println!("Client error: {:?}", why);
        std::process::exit(1);
    }
}
