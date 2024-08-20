use serde::Deserialize;
use serde_json::{Value, from_reader};

#[derive(Debug, Deserialize)]
pub struct JSONInput {
    pub name: String,
    pub r#type: String,
    pub url: String,
    pub results_field_name: String,
    pub schema: Value 
}

impl JSONInput {
    pub async fn build(file: &str) -> Self {
        let json = std::fs::File::open(file).expect("File not found!");
        let json_reader: JSONInput = from_reader(json).expect("Failed to read file into JSON, check build function");

        json_reader
    }

    pub async fn fetch(mut self) -> Self {
        let response = reqwest::get(&self.url)
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();

        let results = &response[&self.results_field_name];

        self.schema = results.clone();

        self
    }
}
