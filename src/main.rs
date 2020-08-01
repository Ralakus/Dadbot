use std::{env, time::Instant};

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
const BACKEND_URL: &str = "https://brainbot.botlibre.com";

fn request_response(
    message: String,
    (id, time, application): &mut (String, std::time::Instant, String),
) -> Result<String, String> {
    let mut rt = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => return Err(format!("{}", e)),
    };
    let client = reqwest::Client::new();

    if Instant::now().duration_since(*time).as_secs() > 60 {
        *id = String::default();
    }

    let packet = if id.is_empty() {
        format!("<chat instance=\"165\" application=\"{}\"><message>{}</message></chat>", application, message)
    } else {
        format!("<chat instance=\"165\" application=\"{}\" conversation=\"{}\"><message>{}</message></chat>", application, id, message)
    };

    let http_respnse = match rt.block_on(
        client
            .post(&format!("{}/rest/api/chat", BACKEND_URL))
            .header("Content-type", "application/xml")
            .body(packet)
            .send(),
    ) {
        Ok(res) => res,
        Err(e) => return Err(format!("{}", e)),
    };

    let response_xml = match rt.block_on(http_respnse.text()) {
        Ok(response) => response,
        Err(e) => return Err(format!("{}", e)),
    };

    if response_xml.contains("Invalid application id") {
        return Err(String::from("Application ID has expired, please reset dadbot"))
    }

    let convseration = match response_xml.find("conversation=\"") {
        Some(i) => i,
        None => {
            return Err(format!(
                "Failed to find conversation id opening for response: {}",
                response_xml
            ))
        }
    };
    let convseration_close = match response_xml.find("\" emote") {
        Some(i) => i,
        None => {
            return Err(format!(
                "Failed to find conversation id closing for response: {}",
                response_xml
            ))
        }
    };

    *id = String::from(&response_xml[convseration + 14..convseration_close]);

    let opening = match response_xml.find("<message>") {
        Some(i) => i,
        None => {
            return Err(format!(
                "Failed to find opening delimiter for response: {}",
                response_xml
            ))
        }
    };
    let closing = match response_xml.find("</message>") {
        Some(i) => i,
        None => {
            return Err(format!(
                "Failed to find closing delimiter for response: {}",
                response_xml
            ))
        }
    };
    let response = String::from(&response_xml[opening + 9..closing]);

    *time = Instant::now();

    Ok(response)
}

fn handle_resset((id, time, application): &mut (String, std::time::Instant, String)) -> String {
    *id = String::default();
    *time = Instant::now();
    
    let mut rt = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => return format!("Error: {}", e),
    };
    
    let body = match rt.block_on(reqwest::get("https://www.botlibre.com/api-test.jsp")) {
        Ok(body) => body,
        Err(e) => return format!("Error: {}", e),
    };

    let text = match rt.block_on(body.text()) {
        Ok(text) => text,
        Err(e) => return format!("Error: {}", e),
    };

    let application_start = match text.find("&application=") {
        Some(i) => i,
        None => {
            return String::from("Failed to find application id opening delimiter in source")
        }
    };
    let application_end = match text.find("&instance=") {
        Some(i) => i,
        None => {
            return String::from("Failed to find application id opening delimiter in source")
        }
    };

    let application_id = &text[application_start + "&application=".len()..application_end];

    *application = String::from(application_id);
    String::from("Reset conversation id, timer, and application id")
}

struct SelfId;

impl TypeMapKey for SelfId {
    type Value = UserId;
}

struct ConversationId;

impl TypeMapKey for ConversationId {
    type Value = (String, Instant, String);
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

        if message.channel_id != TALK_CHANNEL_ID
            || u64::from(message.author.id) == u64::from(*self_id)
        {
            return;
        }

        if ChannelId::from(TALK_CHANNEL_ID)
            .broadcast_typing(&ctx)
            .is_err()
        {
            println!("Failed to send broadcast event");
        }

        let conversation_id = data.get_mut::<ConversationId>().unwrap();

        let response = if message.content.starts_with("!!get id") {
            format!("Conversation id: {}", conversation_id.0)
        } else if message.content.starts_with("!!get reset time") {
            format!(
                "Reset timer: {}",
                Instant::now().duration_since(conversation_id.1).as_secs()
            )
        } else if message.content.starts_with("!!reset") {
            handle_resset(conversation_id)
        } else if message.content.starts_with("!!") {
            String::from("Invalid command:\n\t`!!get id`: Get conversation id\n\t`!!get reset time`: Get the reset timer\n\t`!!reset`: Resets current state")
        } else {
            match request_response(message.content, conversation_id) {
                Ok(s) => s,
                Err(e) => format!("Error getting response: {}", e),
            }
        };

        let result = ChannelId::from(TALK_CHANNEL_ID).send_message(&ctx, |m| m.content(response));
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
        data.insert::<ConversationId>((
            String::default(),
            std::time::Instant::now(),
            env::var("APPLICATION_ID").expect("Expects application id in APPLICATION_ID env"),
        ));
    }

    println!("Connecting");
    if let Err(why) = client.start_shards(1) {
        println!("Client error: {:?}", why);
        std::process::exit(1);
    }
}
