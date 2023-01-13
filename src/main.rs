use clap::Parser;
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

mod conversation;

#[derive(Deserialize, Serialize)]
struct Config {
    listen_channels: Vec<u64>,
}

/// Serenity Discord api handler
struct Handler {
    config: Config,
    loaded_profile: String,
    profile: Mutex<conversation::Profile>,
    conversation: Mutex<conversation::Conversation>,
}

impl Handler {
    fn new(
        config: Config,
        loaded_profile: String,
        profile: conversation::Profile,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            config,
            loaded_profile,
            profile: Mutex::new(profile.clone()),
            conversation: Mutex::new(conversation::Conversation::new(profile)?),
        })
    }
}

#[async_trait]
impl EventHandler for Handler {
    /// Function is called whenever a message is sent that is visible to the bot
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == ctx.cache.current_user_id() {
            return;
        }

        let response = match msg.content.as_str() {
            "!debug" => {
                let profile = self.profile.lock().await;

                format!(
                    "**Profile: {}**\n\nLoaded model: {}\n\nParameters:\nMinimum Token Count: {}\nMaximum Token Count: {}\nDo Sample: {}\nEarly Stopping: {}\nTop P Value: {}\nTop K Value: {}\nTemperature: {}\nRepititon Penalty: {}\nInverse Length Penalty: {}\nBeam Count: {}\nBeam Groups: {}\nReturn Sequence Count: {}\nNo Repeat NGram Size: {}",
                    profile.name,
                    profile.model,
                    profile.min_length,
                    profile.max_length,
                    profile.do_sample,
                    profile.early_stopping,
                    profile.top_p,
                    profile.top_k,
                    profile.temperature,
                    profile.repetition_penalty,
                    profile.length_penalty,
                    profile.num_beams,
                    profile.num_beam_groups,
                    profile.num_return_sequences,
                    profile.no_repeat_ngram_size
                    )
            }
            "!reset" => {
                let mut conversation = self.conversation.lock().await;
                conversation.history.clear();

                println!("Conversation history cleared");

                "Conversation history cleared".to_string()
            }
            "!reload" => {
                let result: anyhow::Result<()> = (|| async {
                    let mut conversation = self.conversation.lock().await;

                    let profile: conversation::Profile =
                        serde_json::from_str(&std::fs::read_to_string(&self.loaded_profile)?)?;

                    *self.profile.lock().await = profile.clone();

                    conversation.load_profile(profile)?;

                    Ok(())
                })()
                .await;
                println!("Attempt profile reload");
                match result {
                    Ok(_) => "Profile reloaded".to_string(),
                    Err(e) => format!("Failed to reload profile: {}", e),
                }
            }
            _ if self.config.listen_channels.contains(&msg.channel_id.0) => {
                let name = msg
                    .author_nick(&ctx.http)
                    .await
                    .unwrap_or_else(|| msg.author.name.clone());

                println!("{}: {}", name, msg.content);

                let mut conversation = self.conversation.lock().await;
                let response = conversation.query(&name, &msg.content);

                println!("-> {}", response);

                response
            }
            _ => return,
        };

        if let Err(e) = msg.reply(&ctx.http, response).await {
            eprintln!("Failed to send message: {}", e);
        }
    }

    /// Function is called once upon startup after the bot is connected
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} ({}) is connected!", ready.user.name, ready.user.id);
    }
}

/// Command line arguments for server.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Sets path for the personality prefix file
    #[clap(default_value = "res/dadbot.json")]
    profile: String,

    /// Sets path for the Discord bot's config file
    #[clap(default_value = "res/config.json")]
    config: String,
}

// Entry point of server
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pyo3::prepare_freethreaded_python();

    // Command line parsing.
    let args = Args::parse();

    // AI profile
    let profile: conversation::Profile =
        serde_json::from_str(&std::fs::read_to_string(&args.profile)?)?;

    // Config
    let config: Config = serde_json::from_str(&std::fs::read_to_string(&args.config)?)?;

    // Discord API token
    let token = env::var("DISCORD_TOKEN").expect("Expeced DISCORD_TOKEN in environment");

    // Discord bot intents to recieve events for
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Bot client
    let mut client = Client::builder(token, intents)
        .event_handler(Handler::new(config, args.profile, profile)?)
        .await?;

    client.start().await?;

    Ok(())
}
