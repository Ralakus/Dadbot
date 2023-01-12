use clap::Parser;
use rust_bert::{
    pipelines::{common::ModelType, text_generation::TextGenerationConfig},
    resources::LocalResource,
};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, Write},
    path::PathBuf,
};

mod conversation;

#[derive(Serialize, Deserialize)]
struct Profile {
    model: String,
    name: String,
    prefix: String,
    min_length: i64,
    max_length: i64,
    early_stopping: bool,
    temperature: f64,
    repetition_penalty: f64,
    length_penalty: f64,
    num_beams: i64,
    num_beam_groups: i64,
    num_return_sequences: i64,
}

/// Command line arguments for server.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Sets path for the personality prefix file
    #[clap(default_value = "res/dadbot.json")]
    profile: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Command line parsing.
    let args = Args::parse();

    let profile: Profile = serde_json::from_str(&std::fs::read_to_string(args.profile)?)?;

    println!("Loading {}...", profile.model);
    let model_resource = Box::new(LocalResource::from(PathBuf::from(format!(
        "{}/rust_model.ot",
        profile.model
    ))));
    let config_resource = Box::new(LocalResource::from(PathBuf::from(format!(
        "{}/config.json",
        profile.model
    ))));
    let vocab_resource = Box::new(LocalResource::from(PathBuf::from(format!(
        "{}/vocab.json",
        profile.model
    ))));
    let merges_resource = Box::new(LocalResource::from(PathBuf::from(format!(
        "{}/merges.txt",
        profile.model
    ))));

    println!("Generating config...");
    let config = TextGenerationConfig {
        model_type: ModelType::GPTNeo,
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource: Some(merges_resource),
        min_length: profile.min_length,
        max_length: Some(profile.max_length),
        early_stopping: profile.early_stopping,
        temperature: profile.temperature,
        repetition_penalty: profile.repetition_penalty,
        length_penalty: profile.length_penalty,
        num_beams: profile.num_beams,
        num_beam_groups: Some(profile.num_beam_groups),
        num_return_sequences: profile.num_return_sequences,
        ..Default::default()
    };

    println!("Initializing personality prefix {}...\n", profile.name);

    let mut conversation =
        conversation::Conversation::new(config, profile.name.clone(), profile.prefix.clone())?;

    print!("What's your name? ");
    std::io::stdout().lock().flush()?;
    let mut name = String::default();
    std::io::stdin().lock().read_line(&mut name)?;
    name = name.trim().to_string();

    loop {
        print!("{}: ", name);
        std::io::stdout().lock().flush()?;
        let mut input = String::default();
        std::io::stdin().lock().read_line(&mut input)?;

        println!(
            "{}: {}",
            profile.name,
            conversation.query(&name, input.trim()).await
        );
    }
}
