use pyo3::prelude::{Py, PyAny, PyErr, ToPyObject};
use pyo3::types::IntoPyDict;
use pyo3::{IntoPy, Python};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    pub model: String,
    pub name: String,
    pub prefix: String,
    pub min_length: i64,
    pub max_length: i64,
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
    model: Py<PyAny>,
    tokenizer: Py<PyAny>,
}

impl Conversation {
    pub fn new(profile: Profile) -> Result<Self, PyErr> {
        let (model, tokenizer) = Self::load_model(&profile.model)?;
        Ok(Self {
            profile,
            history: Vec::new(),
            model,
            tokenizer,
        })
    }

    fn load_model(model: &str) -> Result<(Py<PyAny>, Py<PyAny>), PyErr> {
        Python::with_gil(|py| {
            let model_args = (model,);
            let transformers = py.import("transformers").unwrap();

            let tokenizer = transformers
                .getattr("AutoTokenizer")?
                .getattr("from_pretrained")?
                .call1(model_args)?
                .extract()?;

            let model = transformers
                .getattr("AutoModelForCausalLM")?
                .getattr("from_pretrained")?
                .call1(model_args)?
                .extract()?;

            Ok((model, tokenizer))
        })
    }

    pub fn load_profile(&mut self, profile: Profile) -> Result<(), PyErr> {
        if self.profile.model != profile.model {
            (self.model, self.tokenizer) = Self::load_model(&profile.model)?;
        }
        self.profile = profile;
        Ok(())
    }

    pub fn generate(&self, input: &str) -> anyhow::Result<Vec<String>> {
        let results: Vec<String> = Python::with_gil(|py| -> Result<Vec<String>, PyErr> {
            let inputs: Py<PyAny> = self
                .tokenizer
                .call(
                    py,
                    (input,),
                    Some([("return_tensors", "pt".to_object(py))].into_py_dict(py)),
                )?
                .extract(py)?;

            let tokens_in: i64 = py
                .eval(
                    r#"len(inputs["input_ids"][0])"#,
                    Some([("inputs", &inputs)].into_py_dict(py)),
                    None,
                )?
                .extract()?;

            let outputs: Py<PyAny> = self
                .model
                .getattr(py, "generate")?
                .call(
                    py,
                    (inputs.as_ref(py).get_item("input_ids")?,),
                    Some(
                        [
                            (
                                "min_length",
                                (tokens_in + self.profile.min_length).into_py(py),
                            ),
                            (
                                "max_length",
                                (tokens_in + self.profile.max_length).into_py(py),
                            ),
                            ("do_sample", self.profile.do_sample.into_py(py)),
                            ("early_stopping", self.profile.early_stopping.into_py(py)),
                            ("top_p", self.profile.top_p.into_py(py)),
                            ("top_k", self.profile.top_k.into_py(py)),
                            ("temperature", self.profile.temperature.into_py(py)),
                            (
                                "repetition_penalty",
                                self.profile.repetition_penalty.into_py(py),
                            ),
                            ("length_penalty", self.profile.length_penalty.into_py(py)),
                            ("num_beams", self.profile.num_beams.into_py(py)),
                            ("num_beam_groups", self.profile.num_beam_groups.into_py(py)),
                            (
                                "num_return_sequences",
                                self.profile.num_return_sequences.into_py(py),
                            ),
                            (
                                "no_repeat_ngram_size",
                                self.profile.no_repeat_ngram_size.into_py(py),
                            ),
                        ]
                        .into_py_dict(py),
                    ),
                )?
                .extract(py)?;

            Ok((0..self.profile.num_return_sequences)
                .map(|i| {
                    self.tokenizer
                        .getattr(py, "decode")
                        .and_then(|decode| {
                            decode.call(
                                py,
                                (outputs.as_ref(py).get_item(i).unwrap(),),
                                Some([("skip_special_tokens", true)].into_py_dict(py)),
                            )
                        })
                        .and_then(|result| result.extract(py))
                })
                .filter_map(Result::ok)
                .collect())
        })?;

        let pattern = Regex::new(r#"\n.+:"#).unwrap();

        Ok(results
            .iter()
            .map(|result| {
                result[input.len()..]
                    .split(&pattern)
                    .next()
                    .unwrap_or_default()
                    .to_string()
            })
            .collect())
    }

    pub fn query(&mut self, name: &str, query: &str) -> String {
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
            .unwrap_or_else(|e| {
                error = true;
                vec![format!("Python error: {}", e)]
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
