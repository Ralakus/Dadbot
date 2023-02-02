use regex::Regex;
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;

use crate::backend::task_client::TaskClient;
use crate::backend::TaskRequest;
use tonic::Request;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    pub model: String,
    pub name: String,
    pub prefix: String,
    pub min_length: i64,
    pub max_length: i64,
    pub token_window: i64,
    pub do_sample: bool,
    pub early_stopping: bool,
    pub top_p: f64,
    pub top_k: i64,
    pub temperature: f64,
    pub repetition_penalty: f64,
    pub length_penalty: f64,
    pub num_beams: i64,
    pub num_beam_groups: i64,
    pub num_return_sequences: i64,
    pub no_repeat_ngram_size: i64,
}

pub struct Conversation {
    profile: Profile,
    pub history: Vec<(String, String)>,
    client: TaskClient<Channel>,
}

impl Conversation {
    pub async fn new(profile: Profile, backend_url: &str) -> anyhow::Result<Self> {
        let client = TaskClient::connect(backend_url.to_string()).await?;
        Ok(Self {
            profile,
            history: Vec::new(),
            client,
        })
    }

    pub fn load_profile(&mut self, profile: Profile) {
        self.profile = profile;
    }

    pub async fn generate(&mut self, input: &str) -> anyhow::Result<Vec<String>> {
        let payload = TaskRequest {
            model: self.profile.model.clone(),
            input: input.to_string(),
            min_length: self.profile.min_length,
            max_length: self.profile.max_length,
            token_window: self.profile.token_window,
            do_sample: self.profile.do_sample,
            early_stopping: self.profile.early_stopping,
            top_p: self.profile.top_p,
            top_k: self.profile.top_k,
            temperature: self.profile.temperature,
            repetition_penalty: self.profile.repetition_penalty,
            length_penalty: self.profile.length_penalty,
            num_beams: self.profile.num_beams,
            num_beam_groups: self.profile.num_beam_groups,
            num_return_sequences: self.profile.num_return_sequences,
            no_repeat_ngram_size: self.profile.no_repeat_ngram_size,
        };

        let results: Vec<String> = self
            .client
            .run_task(Request::new(payload))
            .await?
            .into_inner()
            .data;

        let pattern = Regex::new(r#"\n.+:"#).unwrap();

        Ok(results
            .iter()
            .map(|result| {
                result
                    .split(&pattern)
                    .next()
                    .unwrap_or_default()
                    .to_string()
            })
            .collect())
    }

    pub async fn query(&mut self, name: &str, query: &str) -> String {
        let input = format!(
            "{}\n{}{}: {}\n{}: ",
            self.profile.prefix,
            self.history
                .iter()
                .map(|(name, message)| format!("{}: {}\n", name, message))
                .collect::<String>(),
            name,
            query.trim(),
            self.profile.name
        );

        let mut error = false;
        let result = self
            .generate(&input)
            .await
            .unwrap_or_else(|e| {
                error = true;
                vec![format!("Backend error: {}", e)]
            })
            .get(0)
            .or_else(|| {
                error = true;
                None
            })
            .unwrap_or(&"Failure to generate at least one valid response".to_string())
            .trim()
            .to_string();

        if !error {
            self.history.push((name.to_string(), query.to_string()));
            self.history
                .push((self.profile.name.clone(), result.clone()));
        }

        result
    }
}
