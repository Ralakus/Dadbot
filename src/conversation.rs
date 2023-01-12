use regex::Regex;
use rust_bert::{
    pipelines::text_generation::{TextGenerationConfig, TextGenerationModel},
    RustBertError,
};
use tokio::sync::Mutex;
pub struct Conversation {
    model: Mutex<TextGenerationModel>,
    bot_name: String,
    max_generation_length: i64,
    history: Vec<(String, String)>,
    prefix: String
}

impl Conversation {
    pub fn new(
        config: TextGenerationConfig,
        bot_name: String,
        prefix: String,
    ) -> Result<Self, RustBertError> {
        Ok(Self {
            bot_name,
            max_generation_length: config.max_length.unwrap_or_default(),
            model: Mutex::new(TextGenerationModel::new(config)?),
            history: Vec::new(),
            prefix,
        })
    }

    pub async fn query(&mut self, name: &str, query: &str) -> String {
        let input = format!("{}: {}\n{}: ", name, query.trim(), self.bot_name);
        let input_len = input.len();

        let mut transcript_length_sum = self.prefix.len() + input_len;
        let transcript = format!(
            "{}\n{}",
            self.prefix,
            self.history
                .iter()
                .rev()
                .take_while(|h| {
                    if transcript_length_sum + h.0.len() + h.1.len() + 2
                        > 2048 - self.max_generation_length.try_into().unwrap_or(0usize) + input_len
                    {
                        false
                    } else {
                        transcript_length_sum += h.0.len() + h.1.len() + 2;
                        true
                    }
                })
                .fold(String::default(), |transcript, (name, line)| {
                    format!("{}: {}\n{}", name, line, transcript)
                }),
        );

        loop {
            let output = self
                .model
                .lock()
                .await
                .generate(&[&input], Some(transcript.as_str()));

            let pattern = Regex::new(r#"\n(.)+:"#).unwrap();
            // let pattern = Regex::new(r#"\[(.?)+\]:"#).unwrap();

            let responses = output
                .iter()
                .inspect(|o| eprintln!("{:?}", o))
                .filter(|o| {
                    self.history
                        .iter()
                        .rev()
                        .take(4)
                        .filter(|h| {
                            h.1.chars()
                                .zip(o[input_len..].chars())
                                .filter(|(h, o)| h == o)
                                .count()
                                > h.1.len() / 2
                        })
                        .count()
                        == 0
                })
                .map(|o| {
                    o[input_len..]
                        .split(&pattern)
                        .find(|o| !o.trim().is_empty())
                        .unwrap_or_default()
                        .trim()
                });

            let longest_response = responses.max_by(|x, y| x.len().cmp(&y.len()));

            if let Some(response) = longest_response {
                self.history
                    .push((name.to_string(), query.trim().to_string()));
                self.history
                    .push((self.bot_name.clone(), response.to_string()));
                return response.to_string();
            }
        }
    }
}
